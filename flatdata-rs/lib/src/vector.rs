use crate::{error::ResourceStorageError, structs::Struct, SliceExt};

use crate::storage::ResourceHandle;

use std::{borrow::BorrowMut, fmt, io};

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
/// ``` flatdata
/// struct A {
///     x : u32 : 16;
///     y : u32 : 16;
/// }
///
/// archive X {
///    data : vector< A >;
/// }
/// ```
///
/// ```
/// use flatdata::{ MemoryResourceStorage, Archive, ArchiveBuilder, Vector };
/// use flatdata::test::{A, X, XBuilder};
///
/// let storage = MemoryResourceStorage::new("/root/extvec");
/// let builder = XBuilder::new(storage.clone()).expect("failed to create builder");
/// let mut v: Vector<A> = Vector::new();
/// let mut a = v.grow();
/// a.set_x(1);
/// a.set_y(2);
///
/// let mut b = v.grow();
/// b.set_x(3);
/// b.set_y(4);
///
/// assert_eq!(v.len(), 2);
/// builder.set_data(&v.as_view());
///
/// let archive = X::open(storage).expect("failed to open");
/// let view = archive.data();
/// assert_eq!(view[1].x(), 3);
/// ```
///
/// [`ExternalVector`]: struct.ExternalVector.html
/// [`as_view`]: #method.as_view
pub struct Vector<T>
where
    T: Struct,
{
    data: Vec<T>,
}

impl<T> Vector<T>
where
    T: Struct,
{
    /// Creates an empty `Vector<T>`.
    #[inline]
    pub fn new() -> Self {
        Self::with_len(0)
    }

    /// Resets the 'Vector<T>' to its initial empty state.
    #[inline]
    pub fn clear(&mut self) {
        self.data.clear();
        self.data.push(unsafe { T::create_unchecked() });
    }

    /// Creates a `Vector<T>` with `len` many elements.
    ///
    /// `T`'s fields are all filled with zeroes.
    #[inline]
    pub fn with_len(len: usize) -> Self {
        let mut data = Vec::with_capacity(len + 1);
        data.resize_with(len + 1, || unsafe { T::create_unchecked() });
        Self { data }
    }

    /// Reserves capacity for at least `additional` more elements to be
    /// inserted in the given vector. The collection may reserve more space
    /// to avoid frequent reallocations. After calling reserve, capacity
    /// will be greater than or equal to `self.len() + additional`. Does nothing
    /// if capacity is already sufficient.
    #[inline]
    pub fn reserve(&mut self, additional: usize) {
        self.data.reserve(self.data.len() + additional)
    }

    /// Returns a slice for the vector contents.
    ///
    /// Hides sentinels in case T IS_OVERLAPPING_WITH_NEXT,
    /// i.e. a vector of size 4 get's converted into a slice of size 3
    #[inline]
    pub fn as_view(&self) -> &[T] {
        if T::IS_OVERLAPPING_WITH_NEXT {
            &self.data[0..self.data.len().saturating_sub(2)]
        } else {
            &self.data[0..self.data.len() - 1]
        }
    }

    /// Appends an element to the end of this vector and returns a mutable
    /// handle to it.
    #[inline]
    pub fn grow(&mut self) -> &mut T {
        let next = self.len();
        self.data.push(unsafe { T::create_unchecked() });
        &mut self.data[next]
    }
}

impl<T> std::ops::Deref for Vector<T>
where
    T: Struct,
{
    type Target = [T];

    fn deref(&self) -> &[T] {
        let len = self.data.len() - 1;
        &self.data[0..len]
    }
}

impl<T> std::ops::DerefMut for Vector<T>
where
    T: Struct,
{
    fn deref_mut(&mut self) -> &mut [T] {
        let len = self.data.len() - 1;
        &mut self.data[0..len]
    }
}

impl<T> Default for Vector<T>
where
    T: Struct,
{
    /// Creates an empty `Vector<T>`.
    fn default() -> Self {
        Vector::new()
    }
}

impl<T> fmt::Debug for Vector<T>
where
    T: Struct + std::fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_view().fmt(f)
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
/// it. After closing, it can not be used anymore.
///
/// # Examples
/// ``` flatdata
/// struct A {
///     x : u32 : 16;
///     y : u32 : 16;
/// }
///
/// archive X {
///    data : vector< A >;
/// }
/// ```
///
/// ```
/// use flatdata::{MemoryResourceStorage, Archive, ArchiveBuilder};
/// use flatdata::test::{A, X, XBuilder};
///
/// let storage = MemoryResourceStorage::new("/root/extvec");
/// let builder = XBuilder::new(storage.clone()).expect("failed to create builder");
/// {
///     let mut v = builder.start_data().expect("failed to start");
///     let mut a = v.grow().expect("grow failed");
///     a.set_x(0);
///     a.set_y(1);
///
///     let mut a = v.grow().expect("grow failed");
///     a.set_x(2);
///     a.set_y(3);
///
///     let view = v.close().expect("close failed");
///
///     // data can also be inspected directly after closing
///     assert_eq!(view.len(), 2);
///     assert_eq!(view[0].x(), 0);
///     assert_eq!(view[0].y(), 1);
/// }
///
/// let archive = X::open(storage).expect("failed to open");
/// let view = archive.data();
/// assert_eq!(view[1].x(), 2);
/// assert_eq!(view[1].y(), 3);
/// ```
///
/// [`grow`]: #method.grow
pub struct ExternalVector<'a, T>
where
    T: Struct,
{
    data: Vector<T>,
    len: usize,
    resource_handle: ResourceHandle<'a>,
}

impl<'a, T> ExternalVector<'a, T>
where
    T: Struct,
{
    /// Creates an empty `ExternalVector<T>` in the given resource storage.
    pub fn new(resource_handle: ResourceHandle<'a>) -> Self {
        Self {
            data: Vector::new(),
            len: 0,
            resource_handle,
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
    pub fn grow(&mut self) -> io::Result<&mut T> {
        if self.data.as_view().as_bytes().len() > 1024 * 1024 * 32 {
            self.flush()?;
        }
        self.len += 1;
        Ok(self.data.grow())
    }

    /// Flushes the not yet flushed content in this vector to storage.
    fn flush(&mut self) -> io::Result<()> {
        self.resource_handle
            .borrow_mut()
            .write(&self.data.as_view().as_bytes())?;
        self.data.clear();
        Ok(())
    }

    /// Flushes the remaining not yet flushed elements in this vector and
    /// finalizes the data inside the storage.
    ///
    /// An external vector *must* be closed
    pub fn close(mut self) -> Result<&'a [T], ResourceStorageError> {
        if self.data.len() > 0 || self.len == 0 {
            self.flush().map_err(|e| {
                ResourceStorageError::from_io_error(e, self.resource_handle.name().into())
            })?;
        }
        self.resource_handle
            .close()
            .map(|data| <&[T]>::from_bytes(data).expect("Corrupted data"))
    }
}

impl<T> fmt::Debug for ExternalVector<'_, T>
where
    T: Struct,
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
    use crate::test::{A, R};

    #[test]
    fn test_vector_new() {
        let v: Vector<A> = Vector::new();
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_vector_range() {
        let mut v: Vector<R> = Vector::with_len(3);
        v[0].set_first_x(10);
        v[1].set_first_x(20);
        v[2].set_first_x(30);

        assert_eq!(v[0].x(), 10..20);
        assert_eq!(v[1].x(), 20..30);
        assert_eq!(v[2].x(), 30..0);
    }

    #[test]
    fn test_vector_index() {
        let mut v: Vector<A> = Vector::with_len(2);
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
        let a = &mut v[0];
        assert_eq!(a.x(), 1);
        assert_eq!(a.y(), 2);
        let b = &mut v[1];
        assert_eq!(b.x(), 3);
        assert_eq!(b.y(), 4);
    }

    #[test]
    fn test_vector_as_view() {
        let mut v: Vector<A> = Vector::with_len(1);
        assert_eq!(v.len(), 1);
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
        let mut v: Vector<A> = Vector::with_len(1);
        assert_eq!(v.len(), 1);
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
