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

/// A type in flatdata used for reading data.
///
/// Each struct reference in generated code implements this trait.
pub trait Ref: Clone + Copy + Debug + PartialEq {}

/// A mutable type in flatdata used for writing data.
///
/// Each struct reference in generated code has a corresponding type with suffix
/// `Mut` which implements this trait.
pub trait RefMut: Debug {}

/// A factory trait used to bind lifetime to Ref implementations.
///
/// Vector/ArrayView-like classes cannot be directly implemented over the
/// structs since that binds lifetime too early. Instead this generic factory
/// and Higher-Rank-Trait-Bounds are used to emulate higher-kinded-generics.
pub trait Struct<'a>: Clone {
    /// Schema of the type. Used only for debug and inspection purposes.
    const SCHEMA: &'static str;
    /// Size of an object of this type in bytes.
    const SIZE_IN_BYTES: usize;
    /// Whether this structs requires data of the next instance
    const IS_OVERLAPPING_WITH_NEXT: bool;

    /// Item this factory will produce.
    type Item: Ref;

    /// Creates a new item from a slice.
    fn create(data: &'a [u8]) -> Self::Item;

    /// Item this factory will produce.
    type ItemMut: RefMut;

    /// Creates a new item from a slice.
    fn create_mut(data: &'a mut [u8]) -> Self::ItemMut;
}

/// Shortcut trait for Structs that are able to produce references of any given
/// lifetime
///
/// Equivalent to ```for<'a> Struct<'a>'''
pub trait RefFactory: for<'a> Struct<'a> {}
impl<T> RefFactory for T where T: for<'a> Struct<'a> {}

/// Marks structs that can be used stand-alone, e.g. no range
pub trait NoOverlap {}

/// A specialized Struct factory producing Index items.
/// Used primarily by the MultiVector/MultiArrayView.
pub trait IndexStruct<'a>: Struct<'a> {
    /// Provide getter for index
    fn range(data: Self::Item) -> std::ops::Range<usize>;

    /// Provide setter for index
    fn set_index(data: Self::ItemMut, value: usize);
}

/// Shortcut trait for IndexStructs that are able to produce references of any
/// given lifetime
///
/// Equivalent to ```for<'a> IndexStruct<'a>'''
pub trait IndexRefFactory: for<'a> IndexStruct<'a> {}
impl<T> IndexRefFactory for T where T: for<'a> IndexStruct<'a> {}

/// A type in archive used as index of a `MultiArrayView`.
pub trait Index: Ref {}

/// A type in archive used as mutable index of a `MultiVector`.
pub trait IndexMut: RefMut {}

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

/// A type used to create VariadicStructs.
///
/// Vector/ArrayView-like classes cannot be directly implemented over the
/// structs since that binds lifetime too early. Instead this generic factory
/// and Higher-Rank-Trait-Bounds are used to emulate higher-kinded-generics.
pub trait VariadicStruct<'a>: Clone {
    /// Index type
    type Index: IndexRefFactory;

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
pub trait VariadicRefFactory: for<'a> VariadicStruct<'a> {}
impl<T> VariadicRefFactory for T where T: for<'a> VariadicStruct<'a> {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::{A, R};
    use crate::StructBuf;

    #[test]
    fn test_debug() {
        let a = StructBuf::<A>::new();
        let output = format!("{:?}", a);
        assert_eq!(output, "StructBuf { resource: A { x: 0, y: 0, e: Value } }");
    }

    #[test]
    fn test_range() {
        assert_eq!(<R as Struct>::IS_OVERLAPPING_WITH_NEXT, true);
    }
}
