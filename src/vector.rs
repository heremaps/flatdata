use archive::Struct;
use arrayview::ArrayView;
use handle::{Handle, HandleMut};
use memory;
use storage::{MemoryDescriptor, ResourceHandle};

use std::borrow::BorrowMut;
use std::io;
use std::marker;

#[derive(Debug, Clone)]
pub struct Vector<T> {
    data: Vec<u8>,
    _phantom: marker::PhantomData<T>,
}

impl<T> Vector<T>
where
    T: Struct,
{
    // TODO: Add another method with size and remove it here.
    pub fn new(len: usize) -> Self {
        let size = Self::size(len);
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        Self {
            data,
            _phantom: marker::PhantomData,
        }
    }

    pub fn size_in_bytes(&self) -> usize {
        self.data.len() - memory::PADDING_SIZE
    }

    pub fn len(&self) -> usize {
        self.size_in_bytes() / T::SIZE_IN_BYTES
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reserve(&mut self, len: usize) {
        self.data.reserve(Self::size(len))
    }

    /// Calculate size in bytes (with padding) needed to store len-many elements.
    fn size(len: usize) -> usize {
        len * T::SIZE_IN_BYTES + memory::PADDING_SIZE
    }

    pub fn as_view(&self) -> ArrayView<T> {
        ArrayView::new(&MemoryDescriptor::new(&self.data[0], self.size_in_bytes()))
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.size_in_bytes()]
    }

    pub fn grow(&mut self) -> HandleMut<T::Mut> {
        let old_size = self.data.len();
        self.data.resize(old_size + T::SIZE_IN_BYTES, 0);
        let last_index = self.len() - 1;
        HandleMut::new(T::Mut::from(&mut self.data[last_index * T::SIZE_IN_BYTES]))
    }

    pub fn at(&self, index: usize) -> Handle<T> {
        Handle::new(T::from(&self.data[index * T::SIZE_IN_BYTES]))
    }

    pub fn at_mut(&mut self, index: usize) -> HandleMut<T::Mut> {
        HandleMut::new(T::Mut::from(&mut self.data[index * T::SIZE_IN_BYTES]))
    }
}

impl<T: Struct> AsRef<[u8]> for Vector<T> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Vector which flushes its content when growing.
///
/// Useful for serialization of data which does not fit fully in memory.
pub struct ExternalVector<T> {
    data: Vec<u8>,
    len: usize,
    resource_handle: ResourceHandle,
    _phantom: marker::PhantomData<T>,
}

impl<T: Struct> ExternalVector<T> {
    pub fn new(resource_handle: ResourceHandle) -> Self {
        Self {
            data: vec![0; memory::PADDING_SIZE],
            len: 0,
            resource_handle,
            _phantom: marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn grow(&mut self) -> io::Result<HandleMut<T::Mut>> {
        if self.data.len() > 1024 * 1024 * 32 {
            self.flush()?;
        }
        let old_size = self.data.len();
        self.data.resize(old_size + T::SIZE_IN_BYTES, 0);
        self.len += 1;
        Ok(HandleMut::new(T::Mut::from(
            &mut self.data[old_size - memory::PADDING_SIZE],
        )))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.resource_handle
            .borrow_mut()
            .write(&self.data[..self.data.len() - memory::PADDING_SIZE])?;
        self.data.resize(0, 0);
        self.data.resize(memory::PADDING_SIZE, 0);
        Ok(())
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.flush()?;
        self.resource_handle.borrow_mut().close()
    }
}

impl<T> Drop for ExternalVector<T> {
    fn drop(&mut self) {
        debug_assert!(!self.resource_handle.is_open(), "ExternalVector not closed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use memstorage::MemoryResourceStorage;
    use storage::ResourceStorage;
    use storage::create_external_vector;
    use test_structs::*;

    #[test]
    fn test_vector_index() {
        let mut v: Vector<A> = Vector::new(2);
        assert_eq!(v.len(), 2);
        {
            let mut a = v.at_mut(0);
            a.set_x(1);
            a.set_y(2);
            assert_eq!(a.x(), 1);
            assert_eq!(a.y(), 2);
        }
        {
            let mut b = v.at_mut(1);
            b.set_x(3);
            b.set_y(4);
            assert_eq!(b.x(), 3);
            assert_eq!(b.y(), 4);
        }
        let a = v.at(0);
        assert_eq!(a.x(), 1);
        assert_eq!(a.y(), 2);
        let b = v.at(1);
        assert_eq!(b.x(), 3);
        assert_eq!(b.y(), 4);
    }

    #[test]
    fn test_vector_as_view() {
        let mut v: Vector<A> = Vector::new(1);
        {
            let mut a = v.at_mut(0);
            a.set_x(1);
            assert_eq!(a.x(), 1);
            a.set_y(2);
            assert_eq!(a.y(), 2);
        }
        let view = v.as_view();
        let a = view.at(0);
        assert_eq!(a.x(), 1);
        assert_eq!(a.y(), 2);
    }

    #[test]
    fn test_vector_grow() {
        let mut v: Vector<A> = Vector::new(1);
        {
            let mut a = v.at_mut(0);
            a.set_x(1);
            a.set_y(2);
            assert_eq!(a.x(), 1);
            assert_eq!(a.y(), 2);
        }
        {
            let mut b = v.grow();
            b.set_x(3);
            b.set_y(4);
            assert_eq!(b.x(), 3);
            assert_eq!(b.y(), 4);
        }
        {
            assert_eq!(v.len(), 2);
            let a = &v.at(0);
            assert_eq!(a.x(), 1);
            assert_eq!(a.y(), 2);
            let b = &v.at(1);
            assert_eq!(b.x(), 3);
            assert_eq!(b.y(), 4);
        }
        v.grow();
        assert_eq!(v.len(), 3);
    }

    #[test]
    fn test_external_vector() {
        let mut storage = MemoryResourceStorage::new("/root/resources".into());
        {
            let mut v = create_external_vector::<A>(&mut storage, "vector", "Some schema content")
                .expect("failed to create ExternalVector");
            {
                let mut a = v.grow().expect("grow failed");
                a.set_x(0);
                a.set_y(1);
            }
            {
                let mut a = v.grow().expect("grow failed");
                a.set_x(2);
                a.set_y(3);
            }
            v.close().expect("close failed");
        }

        let resource = storage
            .read_and_check_schema("vector", "Some schema content")
            .expect("failed to read vector resource");

        let view: ArrayView<A> = ArrayView::new(&resource);
        assert_eq!(view.len(), 2);
        assert_eq!(view.at(0).x(), 0);
        assert_eq!(view.at(0).y(), 1);
        assert_eq!(view.at(1).x(), 2);
        assert_eq!(view.at(1).y(), 3);
    }
}
