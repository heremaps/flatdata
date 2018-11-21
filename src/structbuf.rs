// Note: This module is called `structbuf` in contrast to `struct` in the C++
// implementation, since Rust does not allow module names to be one of the
// language keywords.
use archive::Struct;
use memory;

use std::fmt;
use std::marker;

/// A container holding a single flatdata struct in memory, and providing read
/// and write access to it.
///
/// Used in combination with [`ArchiveBuilder`] to serialize single struct
/// resources, cf. [coappearances] example.
///
/// A struct buffer derefs (const and mut) to a reference of the underlying
/// struct, therefore, struct getters and setters can be used directly on
/// buffer.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate flatdata;
/// # fn main() {
/// use flatdata::StructBuf;
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
/// let mut a = StructBuf::<A>::new();
/// a.get_mut().set_x(1);
/// a.get_mut().set_y(2);
/// assert_eq!(a.get().x(), 1);
/// assert_eq!(a.get().y(), 2);
/// # }
/// ```
///
/// [`ArchiveBuilder`]: trait.ArchiveBuilder.html
/// [coappearances]: https://github.com/boxdot/flatdata-rs/blob/master/tests/coappearances_test.rs#L183
pub struct StructBuf<T>
where
    T: for<'a> Struct<'a>,
{
    data: Vec<u8>,
    _phantom: marker::PhantomData<T>,
}

impl<T> StructBuf<T>
where
    T: for<'a> Struct<'a>,
{
    /// Creates an empty struct buffer.
    ///
    /// All fields are set to 0.
    pub fn new() -> Self {
        let data = vec![0; <T as Struct>::SIZE_IN_BYTES + memory::PADDING_SIZE];
        Self {
            data,
            _phantom: marker::PhantomData,
        }
    }

    /// Get the stored object
    pub fn get(&self) -> <T as Struct>::Item {
        <T as Struct>::create(&self.data)
    }

    /// Get the mutable version of the stored object
    pub fn get_mut(&mut self) -> <T as Struct>::ItemMut {
        <T as Struct>::create_mut(&mut self.data)
    }

    /// Returns a raw bytes representation of the buffer.
    pub fn as_bytes(&self) -> &[u8] {
        &self.data[0..<T as Struct>::SIZE_IN_BYTES]
    }
}

impl<T> fmt::Debug for StructBuf<T>
where
    T: for<'a> Struct<'a>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "StructBuf {{ resource: {:?} }}", self.get())
    }
}

impl<T> Default for StructBuf<T>
where
    T: for<'a> Struct<'a>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> AsRef<[u8]> for StructBuf<T>
where
    T: for<'a> Struct<'a>,
{
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
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
    fn test_new() {
        let a = StructBuf::<A>::new();
        let b = StructBuf::<A>::default();
        assert_eq!(a.get(), b.get());
    }

    #[test]
    fn test_setter_getter() {
        let mut a = StructBuf::<A>::new();
        a.get_mut().set_x(1);
        a.get_mut().set_y(2);
        assert_eq!(a.get().x(), 1);
        assert_eq!(a.get().y(), 2);
        a.get_mut().set_x(3);
        assert_eq!(a.get().x(), 3);
        assert_eq!(a.get().y(), 2);
        a.get_mut().set_y(4);
        assert_eq!(a.get().x(), 3);
        assert_eq!(a.get().y(), 4);
        let a_ref = a.get();
        assert_eq!(a_ref.x(), 3);
        assert_eq!(a_ref.y(), 4);
    }
}
