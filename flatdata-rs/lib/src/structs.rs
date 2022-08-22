//! This module contains traits that are implemented by the generated code for
//! structs and archive resources.
//!
//! flatdata's code generator translates a flatdata schema to Rust code. The
//! generated code contains all schema definitions embedded as strings, and for
//! each schema element it implements all needed methods and traits.
//!
//! ## Structs
//!
//! For a flatdata struct, let's say `SomeData`, there are three generated
//! types in Rust: `SomeData`, SomeDataRef` and SomeDataRefMut`. The former
//! type is used to to create the latter two. `SomeDataRef` is used to read
//! data from a serialized archive, `SomeDataRefMut` to write data to archive.
//!
//! ## Indexes and variadic types
//!
//! A `MultiVector` is a heterogeneous container which consists of indexed
//! items, each containing several elements of different types (cf.
//! [`MultiVector`]). For each multivector resource in a flatdata archive
//! the generator creates a type with the same name and a type with suffix
//! `Ref`. The former is used as template parameter of containers
//! `MultiVector` and `MultiArrayView`. They implement the traits `VariadicRef`
//! resp. `VariadicStruct`. Additionally, the multivector is indexed by
//! a struct which is automatically generated flatdata struct. It implements
//! the trait `Index` and `IndexRef`.
//!
//! [`MultiVector`]: struct.MultiVector.html

use std::fmt::Debug;

#[doc(hidden)]
pub use std::marker;

/// A factory trait used to bind lifetime to Ref implementations.
///
/// Vector/ArrayView-like classes cannot be directly implemented over the
/// structs since that binds lifetime too early. Instead this generic factory
/// and Higher-Rank-Trait-Bounds are used to emulate higher-kinded-generics.
pub trait Struct: Debug {
    /// Create a new struct
    ///
    /// # Safety
    /// If the struct is not self-contained (NoOverlap),
    /// and there is no directly subsequent structure in memory,
    /// then the resulting instance's data must not be accessed
    unsafe fn create_unchecked() -> Self;

    /// Size of an object of this type in bytes.
    const SIZE_IN_BYTES: usize;
    /// Whether this structs requires data of the next instance
    const IS_OVERLAPPING_WITH_NEXT: bool;
}

/// Marks structs that can be used stand-alone, e.g. no range
///
/// # Safety
///
/// Compiler can't guarantee that the struct does not overlap in the storage (memory)
pub unsafe trait NoOverlap {}
/// Marks structs that cannot be used stand-alone, e.g. no range
pub trait Overlap {}

/// A specialized Struct factory producing Index items.
/// Used primarily by the MultiVector/MultiArrayView.
pub trait IndexStruct: Struct {
    /// Provide getter for index
    fn range(&self) -> std::ops::Range<usize>;

    /// Provide setter for index
    fn set_index(&mut self, value: usize);
}

/// Index specifying a variadic type of `MultiArrayView`.
pub type TypeIndex = u8;

/// A type used as element of `MultiArrayView`.
///
/// Implemented by an enum type.
pub trait VariadicRef: Clone + Debug + PartialEq {
    /// Returns size in bytes of the current variant type.
    ///
    /// Since a variadic struct can contain types of different sized, this is a
    /// method based on the current value type.
    fn size_in_bytes(&self) -> usize;
}

/// A type used as element of 'MultiArrayView'.
///
/// Provides the index type that should be used in the container.
pub trait VariadicIndex {
    /// Index type
    type Index: IndexStruct;
}

/// A type used to create VariadicStructs.
///
/// Vector/ArrayView-like classes cannot be directly implemented over the
/// structs since that binds lifetime too early. Instead this generic factory
/// and Higher-Rank-Trait-Bounds are used to emulate higher-kinded-generics.
pub trait VariadicStruct<'a>: Clone {
    /// Reader type
    type Item: VariadicRef;

    /// Creates a reader for specific type of data.
    fn create(data: TypeIndex, _: &'a [u8]) -> Self::Item;

    /// Associated type used for building an item in `MultiVector` based on
    /// this variadic type.
    ///
    /// The builder is returned by
    /// [`MultiVector::grow`](struct.MultiVector.html#method.grow)
    /// method. It provides convenient methods `add_{variant_name}` for each
    /// enum variant.
    type ItemMut;

    /// Creates a builder for a list of VariadicRef.
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut;
}

/// Shortcut trait for VariadicStructs that are able to produce references of
/// any given lifetime
///
/// Equivalent to ```for<'a> VariadicStruct<'a>'''
pub trait VariadicRefFactory: for<'a> VariadicStruct<'a> + VariadicIndex {}

impl<T> VariadicRefFactory for T
where
    T: for<'a> VariadicStruct<'a>,
    T: VariadicIndex,
{
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::{A, R};

    #[test]
    fn test_debug() {
        let a = A::new();
        let output = format!("{:?}", a);
        assert_eq!(output, "A { x: 0, y: 0, e: Value }");
    }

    #[test]
    #[allow(clippy::assertions_on_constants)]
    fn test_range() {
        assert!(<R as Struct>::IS_OVERLAPPING_WITH_NEXT);
    }
}
