// Note: This module is called `structbuf` in contrast to `struct` in the C++ implementation,
// since Rust does not allow module names to be one of the language keywords.
use archive::Struct;
use memory;

use std::ops;
use std::slice;

/// A container holding a single flatdata struct in memory, and providing read and write access to
/// it.
///
/// Used in combination with [`ArchiveBuilder`] to serialize single struct resources, cf.
/// [coappearances] example.
///
/// A struct buffer derefs (const and mut) to a reference of the underlying struct, therefore,
/// struct getters and setters can be used directly on buffer.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate flatdata;
/// # fn main() {
/// use flatdata::StructBuf;
///
/// define_struct!(A, AMut, "no_schema", 4,
///     (x, set_x, u32, 0, 16),
///     (y, set_y, u32, 16, 16)
/// );
///
/// let mut a = StructBuf::<A>::new();
/// a.set_x(1);
/// a.set_y(2);
/// assert_eq!(a.x(), 1);
/// assert_eq!(a.y(), 2);
/// # }
/// ```
///
/// [`ArchiveBuilder`]: trait.ArchiveBuilder.html
/// [coappearances]: https://github.com/boxdot/flatdata-rs/blob/master/tests/coappearances_test.rs#L183
pub struct StructBuf<T: Struct> {
    buffer: Vec<u8>,
    data: T::Mut,
}

impl<T: Struct> StructBuf<T> {
    /// Creates an empty struct buffer.
    ///
    /// All fields are set to 0.
    pub fn new() -> Self {
        let mut buffer = vec![0; T::SIZE_IN_BYTES + memory::PADDING_SIZE];
        let ptr = buffer.as_mut_ptr();
        Self {
            buffer,
            data: T::Mut::from(ptr),
        }
    }

    /// Returns a raw bytes representation of the buffer.
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.buffer.as_ptr(), T::SIZE_IN_BYTES) }
    }
}

impl<T: Struct> Default for StructBuf<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Struct> ops::Deref for StructBuf<T> {
    type Target = T::Mut;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Struct> ops::DerefMut for StructBuf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Struct> AsRef<[u8]> for StructBuf<T> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

#[cfg(test)]
mod test {
    use super::super::test_structs::A;
    use super::*;

    #[test]
    fn test_new() {
        let a = StructBuf::<A>::new();
        let b = StructBuf::<A>::default();
        assert_eq!(a.as_ref(), b.as_ref());
    }

    #[test]
    fn test_setter_getter() {
        let mut a = StructBuf::<A>::new();
        a.set_x(1);
        a.set_y(2);
        assert_eq!(a.x(), 1);
        assert_eq!(a.y(), 2);
        a.set_x(3);
        assert_eq!(a.x(), 3);
        assert_eq!(a.y(), 2);
        a.set_y(4);
        assert_eq!(a.x(), 3);
        assert_eq!(a.y(), 4);
        let a_ref = (*a).as_ref();
        assert_eq!(a_ref.x(), 3);
        assert_eq!(a_ref.y(), 4);
    }
}
