//! This module contains traits and macros that are used by generated code to
//! define flatdata's structs, archives and resources.
//!
//! flatdata's code generator translates a flatdata schema to Rust code. The
//! generated code contains all schema definitions embedded as strings, and for
//! each schema element it uses one of the macros `define_struct`,
//! `define_index`, `define_variadic_struct`, and `define_archive` to define
//! the corresponding Rust struct and implement all needed methods and traits.
//!
//! ## Structs
//!
//! A flatdata struct, let's say `SomeData`, is introduced by macro
//! `define_struct` which defines three Rust struct types: `SomeData` and
//! `RefSomeData` and `RefMutSomeData`. The former type is used to to create the
//! latter two. `RefSomeData` is used to read data from a serialized
//! archive, `RefMutSomeData` to write data to archive.
//!
//! ## Indexes and variadic types
//!
//! A `MultiVector` is a heterogeneous container which consists of indexed
//! items, each containing several elements of different types (cf.
//! `MultiVector`). Macros `define_index` and `define_variadic_struct` are used
//! to introduce types used with `MultiVector`. `define_index` introduces a
//! struct with a single field `value` used as an index for items.
//! `define_variadic_struct` bounds multiple structs as into a single enum
//! type, which is used for reading. For writing, the macro defines a builder
//! type which has corresponding methods to add each struct to the item.

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

/// A specialized Struct factory producing Index items.
/// Used primarily by the MultiVector/MultiArrayView.
pub trait IndexStruct<'a>: Struct<'a> {
    /// Provide getter for index
    fn index(data: Self::Item) -> usize;

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

//
// Generator macros
//

/// Macro used by generator to define a flatdata struct.
#[doc(hidden)]
#[macro_export]
macro_rules! define_struct {
    ($factory:ident, $name:ident, $name_mut:ident, $schema:expr, $size_in_bytes:expr
        $(,($field:ident, $field_setter:ident, $type:path, $primitive_type:tt, $offset:expr, $bit_size:expr))*) =>
    {
        #[derive(Clone, Copy)]
        pub struct $name<'a> {
            data: *const u8,
            _phantom: $crate::marker::PhantomData<&'a u8>,
        }

        #[derive(Clone, Debug)]
        pub struct $factory{}

        impl<'a> $crate::Struct<'a> for $factory
        {
            const SCHEMA: &'static str = $schema;
            const SIZE_IN_BYTES: usize = $size_in_bytes;

            type Item = $name<'a>;

            #[inline]
            fn create(data : &'a[u8]) -> Self::Item
            {
                Self::Item{ data : data.as_ptr(), _phantom : $crate::marker::PhantomData }
            }

            type ItemMut = $name_mut<'a>;

            #[inline]
            fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
            {
                Self::ItemMut{ data : data.as_mut_ptr(), _phantom : $crate::marker::PhantomData }
            }
        }

        impl<'a> $name<'a> {
            #[inline]
            $(pub fn $field(&self) -> $type {
                let value = read_bytes!($primitive_type, self.data, $offset, $bit_size);
                unsafe { ::std::mem::transmute::<$primitive_type, $type>(value) }
            })*

            #[inline]
            pub fn as_ptr(&self) -> *const u8 {
                self.data
            }
        }

        impl<'a> ::std::fmt::Debug for $name<'a> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f,
                    concat!(stringify!($factory), " {{ ",
                        intersperse!($(concat!( stringify!($field), ": {:?}")), *), " }}"),
                    $(self.$field(),)*)
            }
        }

        impl<'a> ::std::cmp::PartialEq for $name<'a> {
            #[inline]
            fn eq(&self, other: &$name) -> bool {
                $(self.$field() == other.$field()) && *
            }
        }

        impl<'a> $crate::Ref for $name<'a> {}

        pub struct $name_mut<'a> {
            data: *mut u8,
            _phantom: $crate::marker::PhantomData<&'a u8>,
        }

        impl<'a> $name_mut<'a> {
            #[inline]
            $(pub fn $field(&self) -> $type {
                let value = read_bytes!($primitive_type, self.data, $offset, $bit_size);
                unsafe { ::std::mem::transmute::<$primitive_type, $type>(value) }
            })*

            #[inline]
            $(pub fn $field_setter(&mut self, value: $type) {
                let buffer = unsafe {
                    ::std::slice::from_raw_parts_mut(self.data, $size_in_bytes)
                };
                write_bytes!($primitive_type; value, buffer, $offset, $bit_size)
            })*

            #[inline]
            pub fn fill_from(&mut self, other: &$name) {
                $(self.$field_setter(other.$field());)*
            }

            #[inline]
            pub fn into_ref(self) -> $name<'a> {
                $name{ data : self.data, _phantom : $crate::marker::PhantomData }
            }

            #[inline]
            pub fn as_ptr(&self) -> *const u8 {
                self.data
            }

            #[inline]
            pub fn as_mut_ptr(&self) -> *mut u8 {
                self.data
            }
        }

        impl<'a> ::std::fmt::Debug for $name_mut<'a> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $name{ data : self.data, _phantom : $crate::marker::PhantomData }.fmt( f )
            }
        }

        impl<'a> $crate::RefMut for $name_mut<'a> {}
    };
}

/// Macro used by generator to define a flatdata index.
#[doc(hidden)]
#[macro_export]
macro_rules! define_index {
    ($factory:ident,$name:ident, $name_mut:ident, $schema:expr, $size_in_bytes:expr, $size_in_bits:expr) => {
        // TODO: Find a way to put this definition into an internal submodule.
        define_struct!(
            $factory,
            $name,
            $name_mut,
            $schema,
            $size_in_bytes,
            (value, set_value, u64, u64, 0, $size_in_bits)
        );

        impl<'a> $crate::IndexStruct<'a> for $factory {
            #[inline]
            fn index(data: Self::Item) -> usize {
                data.value() as usize
            }

            #[inline]
            fn set_index(mut data: Self::ItemMut, value: usize) {
                data.set_value(value as u64);
            }
        }
    };
}

/// Macro used by generator to define a flatdata variant used in `MultiVector`
/// and `MultiArrayView`.
#[doc(hidden)]
#[macro_export]
macro_rules! define_variadic_struct {
    ($factory:ident, $name:ident, $item_builder_name:ident, $index_type:path,
        $($type_index:expr => ($type_name:ident, $inner_type:path, $add_type_fn:ident)),+) =>
    {
        #[derive(Clone, PartialEq)]
        pub enum $name<'a> {
            $($type_name(<$inner_type as $crate::Struct<'a>>::Item),)*
        }

        impl<'a> ::std::fmt::Debug for $name<'a> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($name::$type_name(ref inner) => write!(f, "{:?}", inner)),+
                }
            }
        }

        impl<'a> $crate::VariadicRef for $name<'a> {
            #[inline]
            fn size_in_bytes(&self) -> usize {
                match *self {
                    $($name::$type_name(_) => <$inner_type as $crate::Struct<'a>>::SIZE_IN_BYTES),+
                }
            }
        }

        pub struct $item_builder_name<'a> {
            data: &'a mut Vec<u8>
        }

        impl<'a> $item_builder_name<'a> {
            #[inline]
            $(pub fn $add_type_fn<'b>(&'b mut self) -> <$inner_type as $crate::Struct<'b>>::ItemMut {
                let old_len = self.data.len();
                let increment = 1 + <$inner_type as $crate::Struct<'b>>::SIZE_IN_BYTES;
                self.data.resize(old_len + increment, 0);
                self.data[old_len - $crate::PADDING_SIZE] = $type_index;
                <$inner_type as $crate::Struct<'b>>::create_mut(
                    &mut self.data[1 + old_len - $crate::PADDING_SIZE..]
                )
            })*
        }

        #[derive(Clone)]
        pub struct $factory{}

        impl<'a> $crate::VariadicStruct<'a> for $factory {
            type Index = $index_type;

            type Item = $name<'a>;

            #[inline]
            fn create(index : $crate::TypeIndex, data : &'a [u8]) -> Self::Item
            {
                match index {
                    $($type_index => $name::$type_name(<$inner_type as $crate::Struct<'a>>::create(data))),+,
                    _ => panic!(concat!(
                        "invalid type index {} for type ", stringify!($name)), index),
                }
            }

            type ItemMut = $item_builder_name<'a>;

            #[inline]
            fn create_mut(data : &'a mut Vec<u8>) -> Self::ItemMut
            {
                $item_builder_name{data}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::{helper::Int, structbuf::StructBuf};

    #[test]
    #[allow(dead_code)]
    fn test_debug() {
        #[derive(Debug, PartialEq, Eq)]
        #[repr(u32)]
        pub enum MyEnum {
            Value,
        }
        define_struct!(
            A,
            RefA,
            RefMutA,
            "no_schema",
            4,
            (x, set_x, u32, u32, 0, 16),
            (y, set_y, u32, u32, 16, 16),
            (e, set_e, MyEnum, u32, 32, 16)
        );
        let a = StructBuf::<A>::new();
        let output = format!("{:?}", a);
        assert_eq!(output, "StructBuf { resource: A { x: 0, y: 0, e: Value } }");
    }

    macro_rules! define_enum_test {
        ($test_name:ident, $type:tt, $is_signed:expr, $val1:expr, $val2:expr) => {
            #[test]
            #[allow(dead_code)]
            fn $test_name() {
                #[derive(Debug, PartialEq, Eq)]
                #[repr($type)]
                pub enum Variant {
                    X = $val1,
                    Y = $val2,
                }

                impl Int for Variant {
                    const IS_SIGNED: bool = $is_signed;
                }

                define_struct!(
                    A,
                    RefA,
                    RefMutA,
                    "no_schema",
                    1,
                    (x, set_x, Variant, $type, 0, 2)
                );
                let mut a = StructBuf::<A>::new();
                let output = format!("{:?}", a);
                assert_eq!(output, "StructBuf { resource: A { x: X } }");

                a.get_mut().set_x(Variant::Y);
                let output = format!("{:?}", a);
                assert_eq!(output, "StructBuf { resource: A { x: Y } }");
            }
        };
    }

    define_enum_test!(test_enum_u8_1, u8, false, 0, 1);
    define_enum_test!(test_enum_u8_2, u8, false, 0, 2);
    define_enum_test!(test_enum_u16_1, u16, false, 0, 1);
    define_enum_test!(test_enum_u16_2, u16, false, 0, 2);
    define_enum_test!(test_enum_u32_1, u32, false, 0, 1);
    define_enum_test!(test_enum_u32_2, u32, false, 0, 2);
    define_enum_test!(test_enum_u64_1, u64, false, 0, 1);
    define_enum_test!(test_enum_u64_2, u64, false, 0, 2);

    // Note: Right now, there a regression bug for binary enums with underlying
    // type i8: https://github.com/rust-lang/rust/issues/51582
    //
    // Until it is backported into stable release, we have to disable this test.
    //
    // define_enum_test!(test_enum_i8, i8, true, 0, 1);
    // define_enum_test!(test_enum_i8, i8, true, 0, -1);
    define_enum_test!(test_enum_i16_1, i16, true, 0, 1);
    define_enum_test!(test_enum_i16_2, i16, true, 0, -1);
    define_enum_test!(test_enum_i32_1, i32, true, 0, 1);
    define_enum_test!(test_enum_i32_2, i32, true, 0, -1);
    define_enum_test!(test_enum_i64_1, i64, true, 0, 1);
    define_enum_test!(test_enum_i64_2, i64, true, 0, -1);
}
