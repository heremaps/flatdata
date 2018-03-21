use archive::Struct;
use arrayview::ArrayView;
use storage::MemoryDescriptor;
use memory;
use bytewriter;

use std::marker;
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Vector<T> {
    data: Vec<bytewriter::StreamType>,
    _phantom: marker::PhantomData<T>,
}

impl<T> Vector<T>
where
    T: Struct,
{
    pub fn new(len: usize) -> Self {
        let size = Self::size(len);
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0 as bytewriter::StreamType);
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
        ArrayView::new(&MemoryDescriptor::new(&self.data[0], self.data.len()))
    }

    pub fn grow(&mut self) -> &mut T {
        let old_size = self.data.len();
        self.data.resize(old_size + T::SIZE_IN_BYTES, 0);
        let last_index = self.len() - 1;
        &mut self[last_index]
    }
}

impl<T: Struct> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        unsafe { &*(&self.data[index * T::SIZE_IN_BYTES] as *const _ as *const T) }
    }
}

impl<T: Struct> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        unsafe { &mut *(&mut self.data[index * T::SIZE_IN_BYTES] as *mut _ as *mut T) }
    }
}

/// Vector which flushes its content when growing.
///
/// Useful for serialization of data which does not fit fully in memory.
pub struct ExternalVector<T> {
    data: Vec<u8>,
    len: usize,
    _phantom: marker::PhantomData<T>,
}

impl<T: Struct> ExternalVector<T> {
    // TODO: Provide something to write into here!
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(memory::PADDING_SIZE);
        data.resize(memory::PADDING_SIZE, 0);
        Self {
            data,
            len: 0,
            _phantom: marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn grow(&mut self) -> &mut T {
        if self.data.len() > 1024 * 1024 * 32 {
            self.flush();
        }
        let old_size = self.data.len();
        self.data.resize(old_size + T::SIZE_IN_BYTES, 0);
        self.len += 1;
        unsafe { &mut *(&mut self.data[old_size - memory::PADDING_SIZE] as *mut _ as *mut T) }
    }

    fn flush(&mut self) {
        // TODO: Implement writing into storage
        self.data.resize(0, 0);
        self.data.resize(memory::PADDING_SIZE, 0);
    }

    pub fn close(&mut self) {
        self.flush();
        // TODO: Close the resource we are writing into.
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice;

    #[derive(Debug, PartialEq)]
    #[repr(C)]
    struct A {
        first_byte: u8,
    }

    impl A {
        pub fn x(&self) -> u32 {
            read_bytes!(u32, &self.first_byte, 0, 16)
        }

        pub fn y(&self) -> u32 {
            read_bytes!(u32, &self.first_byte, 16, 16)
        }

        pub fn set_x(&mut self, value: u32) {
            let buffer =
                unsafe { slice::from_raw_parts_mut(&mut self.first_byte, Self::SIZE_IN_BYTES) };
            write_bytes!(u32; value, buffer, 0, 16);
        }

        pub fn set_y(&mut self, value: u32) {
            let buffer =
                unsafe { slice::from_raw_parts_mut(&mut self.first_byte, Self::SIZE_IN_BYTES) };
            write_bytes!(u32; value, buffer, 16, 16);
        }
    }

    impl Struct for A {
        const SCHEMA: &'static str = "struct A { }";
        const SIZE_IN_BYTES: usize = 4;
    }

    #[test]
    fn test_vector_index() {
        let mut v: Vector<A> = Vector::new(2);
        assert_eq!(v.len(), 2);
        {
            let a = &mut v[0];
            a.set_x(1);
            a.set_y(2);
            assert_eq!(a.x(), 1);
            assert_eq!(a.y(), 2);
        }
        {
            let b = &mut v[1];
            b.set_x(3);
            b.set_y(4);
            assert_eq!(b.x(), 3);
            assert_eq!(b.y(), 4);
        }
        let a = &v[0];
        assert_eq!(a.x(), 1);
        assert_eq!(a.y(), 2);
        let b = &v[1];
        assert_eq!(b.x(), 3);
        assert_eq!(b.y(), 4);
    }

    #[test]
    fn test_vector_as_view() {
        let mut v: Vector<A> = Vector::new(1);
        {
            let a = &mut v[0];
            a.set_x(1);
            assert_eq!(a.x(), 1);
            a.set_y(2);
            assert_eq!(a.y(), 2);
        }
        let view = v.as_view();
        let a = &view[0];
        assert_eq!(a.x(), 1);
        assert_eq!(a.y(), 2);
    }

    #[test]
    fn test_vector_grow() {
        let mut v: Vector<A> = Vector::new(1);
        {
            let a = &mut v[0];
            a.set_x(1);
            a.set_y(2);
            assert_eq!(a.x(), 1);
            assert_eq!(a.y(), 2);
        }
        {
            let b = v.grow();
            b.set_x(3);
            b.set_y(4);
            assert_eq!(b.x(), 3);
            assert_eq!(b.y(), 4);
        }
        {
            assert_eq!(v.len(), 2);
            let a = &v[0];
            assert_eq!(a.x(), 1);
            assert_eq!(a.y(), 2);
            let b = &v[1];
            assert_eq!(b.x(), 3);
            assert_eq!(b.y(), 4);
        }
        v.grow();
        assert_eq!(v.len(), 3);
    }
}
