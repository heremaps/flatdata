use archive::Struct;
use arrayview::ArrayView;
use error::ResourceStorageError;

use memory;
use storage::ResourceHandle;

use std::borrow::{Borrow, BorrowMut};
use std::fmt;
use std::io;
use std::marker;

/// A container holding a contiguous sequence of flatdata structs of the same
/// type `T` in memory, and providing read and write access to it.
///
/// Vector data is fully stored and populated in memory before it is
/// serialized. This container is often used for data which needs to be changed
/// or updated after insertion in the container. When data can be incrementally
/// serialized without later updates, [`ExternalVector`] is usually a
/// better choice since it may decrease the memory footprint of serialization
/// significantly.
///
/// An archive builder provides a setter for each vector resource. Use
/// [`as_view`] and the corresponding setter to write a `Vector` to storage.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate flatdata;
/// # fn main() {
/// use flatdata::Vector;
///
/// define_struct!(
///     A,
///     RefA,
///     RefMutA,
///     "no_schema",
///     4,
///     (x, set_x, u32, 0, 16),
///     (y, set_y, u32, 16, 16)
/// );
///
/// let mut v: Vector<A> = Vector::new();
/// {
///     let mut a = v.grow();
///     a.set_x(1);
///     a.set_y(2);
/// }
/// {
///     let mut b = v.grow();
///     b.set_x(3);
///     b.set_y(4);
/// }
///
/// assert_eq!(v.len(), 2);
/// // serialize
/// // SomeArchiveBuilder.set_vector_resource_of_a_s(&v.as_view());
/// # }
/// ```
///
/// [`ExternalVector`]: struct.ExternalVector.html
/// [`as_view`]: #method.as_view
#[derive(Clone)]
pub struct Vector<T> {
    data: Vec<u8>,
    _phantom: marker::PhantomData<T>,
}

impl<T> Vector<T>
where
    T: for<'b> Struct<'b>,
{
    /// Creates an empty `Vector<T>`.
    #[inline]
    pub fn new() -> Self {
        Self::with_len(0)
    }

    /// Creates a `Vector<T>` with `len` many elements.
    ///
    /// `T`'s fields are all filled with zeroes.
    #[inline]
    pub fn with_len(len: usize) -> Self {
        let size = Self::calc_size(len);
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0);
        Self {
            data,
            _phantom: marker::PhantomData,
        }
    }

    /// Size of the vector in bytes.
    #[inline]
    pub fn size_in_bytes(&self) -> usize {
        self.data.len() - memory::PADDING_SIZE
    }

    /// Number of elements in the vector.
    #[inline]
    pub fn len(&self) -> usize {
        self.size_in_bytes() / <T as Struct>::SIZE_IN_BYTES
    }

    /// Returns `true` if the vector has a length 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Reserves capacity for at least `additional` more elements to be
    /// inserted in the given vector. The collection may reserve more space
    /// to avoid frequent reallocations. After calling reserve, capacity
    /// will be greater than or equal to `self.len() + additional`. Does nothing
    /// if capacity is already sufficient.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        let additional_bytes = Self::calc_size(self.len() + additional) - self.size_in_bytes();
        self.data.reserve(additional_bytes)
    }

    /// Returns an `ArrayView` to this vector.
    #[inline]
    pub fn as_view(&self) -> ArrayView<T> {
        ArrayView::new(&self.data[..self.size_in_bytes()])
    }

    /// Returns the contents of this vector as slice of bytes.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[..self.size_in_bytes()]
    }

    /// Appends an element to the end of this vector and returns a mutable
    /// handle to it.
    #[inline]
    pub fn grow(&mut self) -> <T as Struct>::ItemMut {
        let old_size = self.data.len();
        self.data.resize(old_size + <T as Struct>::SIZE_IN_BYTES, 0);
        let last_index = self.len() - 1;
        T::create_mut(&mut self.data[last_index * <T as Struct>::SIZE_IN_BYTES..])
    }

    /// Return an accessor handle to the element at position `index` in the
    /// vector.
    #[inline]
    pub fn at(&self, index: usize) -> <T as Struct>::Item {
        T::create(&self.data[index * <T as Struct>::SIZE_IN_BYTES..])
    }

    /// Return a mutable handle to the element at position `index` in the
    /// vector.
    #[inline]
    pub fn at_mut(&mut self, index: usize) -> <T as Struct>::ItemMut {
        T::create_mut(&mut self.data[index * <T as Struct>::SIZE_IN_BYTES..])
    }

    /// Calculates size in bytes (with padding) needed to store `len` many
    /// elements.
    #[inline]
    fn calc_size(len: usize) -> usize {
        len * <T as Struct>::SIZE_IN_BYTES + memory::PADDING_SIZE
    }
}

impl<T> Default for Vector<T>
where
    T: for<'b> Struct<'b>,
{
    /// Creates an empty `Vector<T>`.
    fn default() -> Self {
        Vector::new()
    }
}

impl<T> AsRef<[u8]> for Vector<T>
where
    T: for<'b> Struct<'b>,
{
    /// Returns the content of this vector as slice of bytes. Equivalent to
    /// [`as_bytes`].
    ///
    /// [`as_bytes`]: #method.as_bytes
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<T> fmt::Debug for Vector<T>
where
    T: for<'b> Struct<'b>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let view = self.as_view();
        let preview: Vec<_> = view.iter().take(super::DEBUG_PREVIEW_LEN).collect();
        write!(
            f,
            "Vector {{ len: {}, data: {:?}{} }}",
            self.len(),
            preview,
            if self.len() <= super::DEBUG_PREVIEW_LEN {
                ""
            } else {
                "..."
            }
        )
    }
}

/// Vector which flushes its content when growing.
///
/// Useful for serialization of data which does not fully fit in memory.
///
/// External vector does not provide access to elements previously added to it.
/// Only the last element added to the vector using the result of the method
/// [`grow`] can be accessed and written.
///
/// An external vector *must* be closed, after the last element was written to
/// it. After closing, it can not be used anymore. Not closing the vector will
/// result in panic on drop (in debug mode).
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate flatdata;
/// # fn main() {
/// use flatdata::{
///     create_external_vector, ArrayView, ExternalVector, MemoryResourceStorage, ResourceStorage,
/// };
///
/// define_struct!(
///     A,
///     RefA,
///     RefMutA,
///     "no_schema",
///     4,
///     (x, set_x, u32, 0, 16),
///     (y, set_y, u32, 16, 16)
/// );
///
/// let storage = MemoryResourceStorage::new("/root/extvec");
/// {
///     let mut v = create_external_vector::<A>(&*storage, "vector", "Some schema content")
///         .expect("failed to create ExternalVector");
///     {
///         let mut a = v.grow().expect("grow failed");
///         a.set_x(0);
///         a.set_y(1);
///     }
///     {
///         let mut a = v.grow().expect("grow failed");
///         a.set_x(2);
///         a.set_y(3);
///     }
///     let view = v.close().expect("close failed");
///
///     // data can also be inspected directly after closing
///     assert_eq!(view.len(), 2);
///     assert_eq!(view.at(0).x(), 0);
///     assert_eq!(view.at(0).y(), 1);
///     assert_eq!(view.at(1).x(), 2);
///     assert_eq!(view.at(1).y(), 3);
/// }
///
/// let resource = storage
///     .read_and_check_schema("vector", "Some schema content")
///     .expect("failed to read vector resource");
///
/// let view: ArrayView<A> = ArrayView::new(&resource);
/// assert_eq!(view.len(), 2);
/// assert_eq!(view.at(0).x(), 0);
/// assert_eq!(view.at(0).y(), 1);
/// assert_eq!(view.at(1).x(), 2);
/// assert_eq!(view.at(1).y(), 3);
///
/// # }
/// ```
///
/// [`grow`]: #method.grow
pub struct ExternalVector<'a, T> {
    data: Vec<u8>,
    len: usize,
    resource_handle: ResourceHandle<'a>,
    _phantom: marker::PhantomData<T>,
}

impl<'a, T> ExternalVector<'a, T>
where
    T: for<'b> Struct<'b>,
{
    /// Creates an empty `ExternalVector<T>` in the given resource storage.
    pub fn new(resource_handle: ResourceHandle<'a>) -> Self {
        Self {
            data: vec![0; memory::PADDING_SIZE],
            len: 0,
            resource_handle,
            _phantom: marker::PhantomData,
        }
    }

    /// Number of elements that where added to this vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if no element were added to this vector yet.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Appends an element to the end of this vector and returns a mutable
    /// handle to it.
    ///
    /// Calling this method may flush data to storage (cf. [`flush`]), which
    /// may fail due to different IO reasons.
    ///
    /// [`flush`]: #method.flush
    pub fn grow(&mut self) -> io::Result<<T as Struct>::ItemMut> {
        if self.data.len() > 1024 * 1024 * 32 {
            self.flush()?;
        }
        let old_size = self.data.len();
        self.data.resize(old_size + <T as Struct>::SIZE_IN_BYTES, 0);
        self.len += 1;
        Ok(T::create_mut(
            &mut self.data[old_size - memory::PADDING_SIZE..],
        ))
    }

    /// Flushes the not yet flushed content in this vector to storage.
    fn flush(&mut self) -> io::Result<()> {
        self.resource_handle
            .borrow_mut()
            .write(&self.data[..self.data.len() - memory::PADDING_SIZE])?;
        self.data.resize(0, 0);
        self.data.resize(memory::PADDING_SIZE, 0);
        Ok(())
    }

    /// Returns `true` if this vector was not closed yet and more elements can
    /// be added to it.
    pub fn is_open(&self) -> bool {
        self.resource_handle.borrow().is_open()
    }

    /// Flushes the remaining not yet flushed elements in this vector and
    /// finalizes the data inside the storage.
    ///
    /// An external vector *must* be closed, otherwise it will panic on drop
    pub fn close(mut self) -> Result<ArrayView<'a, T>, ResourceStorageError> {
        self.flush().map_err(|e| {
            ResourceStorageError::from_io_error(e, self.resource_handle.name().into())
        })?;
        self.resource_handle
            .close()
            .map(|data| ArrayView::new(data))
    }
}

impl<'a, T> fmt::Debug for ExternalVector<'a, T>
where
    T: for<'b> Struct<'b>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ExternalVector {{ len: {} }}", self.len())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    // Note: ExternalVector is tested in the corresponding example.

    use super::*;

    define_struct!(
        A,
        RefA,
        RefMutA,
        "no_schema",
        4,
        (x, set_x, u32, 0, 16),
        (y, set_y, u32, 16, 16)
    );

    #[test]
    fn test_vector_new() {
        let v: Vector<A> = Vector::new();
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_vector_index() {
        let mut v: Vector<A> = Vector::with_len(2);
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
        let mut v: Vector<A> = Vector::with_len(1);
        assert_eq!(v.len(), 1);
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
        let mut v: Vector<A> = Vector::with_len(1);
        assert_eq!(v.len(), 1);
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
}
