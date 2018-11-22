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
//!
//! # Archive
//!
//! A flatdata archive is introduced by `define_archive`. It defines two types
//! `ArchiveName` and `ArchiveNameBuilder` for reading resp. writing data.

use error::ResourceStorageError;
use storage::ResourceStorage;

use std::cell::RefCell;

use std::fmt::Debug;
use std::rc::Rc;

pub use std::marker;

/// A type in flatdata used for reading data.
///
/// Each struct reference in generated code implements this trait.
pub trait Ref: Clone + Debug + PartialEq {}

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
    fn create(&'a [u8]) -> Self::Item;

    /// Item this factory will produce.
    type ItemMut: RefMut;

    /// Creates a new item from a slice.
    fn create_mut(&'a mut [u8]) -> Self::ItemMut;
}

/// A specialized Struct factory producing Index items.
/// Used primarily by the MultiVector/MultiArrayView.
pub trait IndexStruct<'a>: Struct<'a> {
    /// Provide getter for index
    fn index(data: Self::Item) -> usize;

    /// Provide setter for index
    fn set_index(data: Self::ItemMut, value: usize);
}

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
    /// Reader type
    type Item: VariadicRef;

    /// Creates a reader for specific type of data.
    fn create(TypeIndex, &'a [u8]) -> Self::Item;

    /// Associated type used for building an item in `MultiVector` based on
    /// this variadic type.
    ///
    /// The builder is returned by
    /// [`MultiVector::grow`](struct.MultiVector.html#method.grow)
    /// method. It provides convenient methods `add_{variant_name}` for each
    /// enum variant.
    type ItemMut;

    /// Creates a builder for a list of VariadicRef.
    fn create_mut(&'a mut Vec<u8>) -> Self::ItemMut;
}

/// A flatdata archive representing serialized data.
///
/// Each archive in generated code implements this trait.
pub trait Archive: Debug + Clone {
    /// Name of the archive.
    const NAME: &'static str;
    /// Schema of the archive.
    ///
    /// Used for verifying the integrity of the archive when opening.
    const SCHEMA: &'static str;

    /// Opens the archive with name `NAME` and schema `SCHEMA` in the given
    /// storage for reading.
    ///
    /// When opening the archive, the schema of the archive and the schema
    /// stored in the storage are compared as strings. If there is a
    /// difference, an Error [`ResourceStorageError::WrongSignature`](enum.
    /// ResourceStorageError.html) is returned containing a detailed diff
    /// of both schemata.
    ///
    /// All resources are in the archive are also opened and their schemata are
    /// verified. If any non-optional resource is missing or has a wrong
    /// signature (unexpected schema), the operation will fail. Therefore,
    /// it is not possible to open partially written archive.
    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError>;
}

/// A flatdata archive builder for serializing data.
///
/// For each archive in generated code there is a corresponding archive builder
/// which implements this trait.
pub trait ArchiveBuilder: Clone {
    /// Name of the archive associated with this archive builder.
    const NAME: &'static str;
    /// Schema of the archive associated with this archive builder.
    ///
    /// Used only for debug and inspection purposes.
    const SCHEMA: &'static str;

    /// Creates an archive with name `NAME` and schema `SCHEMA` in the given
    /// storage for writing.
    ///
    /// If the archive is successfully created, the storage will contain the
    /// archive and archives schema. Archive's resources need to be written
    /// separately by using the corresponding generated methods:
    ///
    /// * `set_struct`
    /// * `set_vector`
    /// * `start_vector`/`finish_vector`
    /// * `start_multivector`/`finish_multivector`.
    ///
    /// For more information about how to write resources, cf. the
    /// [coappearances] example.
    ///
    /// [coappearances]: https://github.com/boxdot/flatdata-rs/blob/master/tests/coappearances_test.rs#L159
    fn new(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError>;
}

//
// Generator macros
//

/// Macro used by generator to define a flatdata struct.
#[macro_export]
macro_rules! define_struct {

    // Simpler case where type and primitive_type coincide.
    ($factory:ident, $name:ident, $name_mut:ident, $schema:expr, $size_in_bytes:expr
        $(,($field:ident, $field_setter:ident, $type:tt, $offset:expr, $bit_size:expr))*) => {
        define_struct!($factory, $name, $name_mut, $schema, $size_in_bytes
            $(,($field, $field_setter, $type: $type, $offset, $bit_size))*
        );
    };
    ($factory:ident, $name:ident, $name_mut:ident, $schema:expr, $size_in_bytes:expr
        $(,($field:ident, $field_setter:ident, $type:tt: $primitive_type:tt, $offset:expr, $bit_size:expr))*) =>
    {
        #[derive(Clone)]
        pub struct $name<'a> {
            data: *const u8,
            _phantom: $crate::marker::PhantomData<&'a u8>,
        }

        #[derive(Clone)]
        pub struct $factory{}

        impl<'a> $crate::Struct<'a> for $factory
        {
            const SCHEMA: &'static str = $schema;
            const SIZE_IN_BYTES: usize = $size_in_bytes;

            type Item = $name<'a>;
            fn create(data : &'a[u8]) -> Self::Item
            {
                Self::Item{ data : data.as_ptr(), _phantom : $crate::marker::PhantomData }
            }

            type ItemMut = $name_mut<'a>;
            fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
            {
                Self::ItemMut{ data : data.as_mut_ptr(), _phantom : $crate::marker::PhantomData }
            }
        }

        impl<'a> $name<'a> {
            $(pub fn $field(&self) -> $type {
                let value = read_bytes!($primitive_type, self.data, $offset, $bit_size);
                unsafe { ::std::mem::transmute::<$primitive_type, $type>(value) }
            })*
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
            $(pub fn $field(&self) -> $type {
                let value = read_bytes!($primitive_type, self.data, $offset, $bit_size);
                unsafe { ::std::mem::transmute::<$primitive_type, $type>(value) }
            })*

            $(pub fn $field_setter(&mut self, value: $type) {
                let buffer = unsafe {
                    ::std::slice::from_raw_parts_mut(self.data, $size_in_bytes)
                };
                write_bytes!($type; value, buffer, $offset, $bit_size)
            })*

            pub fn fill_from(&mut self, other: &$name) {
                $(self.$field_setter(other.$field());)*
            }

            pub fn into_ref(self) -> $name<'a> {
                $name{ data : self.data, _phantom : $crate::marker::PhantomData }
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
            (value, set_value, u64, 0, $size_in_bits)
        );

        impl<'a> $crate::IndexStruct<'a> for $factory {
            fn index(data: Self::Item) -> usize {
                data.value() as usize
            }

            fn set_index(mut data: Self::ItemMut, value: usize) {
                data.set_value(value as u64);
            }
        }
    };
}

/// Macro used by generator to define a flatdata variant used in `MultiVector`
/// and `MultiArrayView`.
#[macro_export]
macro_rules! define_variadic_struct {
    ($factory:ident, $name:ident, $item_builder_name:ident, $index_type:path,
        $($type_index:expr => ($type:tt, $add_type_fn:ident)),+) =>
    {
        #[derive(Clone, PartialEq)]
        pub enum $name<'a> {
            $($type(<$type as $crate::Struct<'a>>::Item),)*
        }

        impl<'a> ::std::fmt::Debug for $name<'a> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($name::$type(ref inner) => write!(f, "{:?}", inner)),+
                }
            }
        }

        impl<'a> $crate::VariadicRef for $name<'a> {
            fn size_in_bytes(&self) -> usize {
                match *self {
                    $($name::$type(_) => <$type as $crate::Struct<'a>>::SIZE_IN_BYTES),+
                }
            }
        }

        pub struct $item_builder_name<'a> {
            data: &'a mut Vec<u8>
        }

        impl<'a> $item_builder_name<'a> {
            $(pub fn $add_type_fn<'b>(&'b mut self) -> <$type as $crate::Struct<'b>>::ItemMut {
                let old_len = self.data.len();
                let increment = 1 + <$type as $crate::Struct<'b>>::SIZE_IN_BYTES;
                self.data.resize(old_len + increment, 0);
                self.data[old_len - $crate::PADDING_SIZE] = $type_index;
                <$type as $crate::Struct<'b>>::create_mut(
                    &mut self.data[1 + old_len - $crate::PADDING_SIZE..]
                )
            })*
        }

        #[derive(Clone)]
        pub struct $factory{}

        impl<'a> $crate::VariadicStruct<'a> for $factory {
            type Item = $name<'a>;

            fn create(index : $crate::TypeIndex, data : &'a [u8]) -> Self::Item
            {
                match index {
                    $($type_index => $name::$type(<$type as $crate::Struct<'a>>::create(data))),+,
                    _ => panic!(concat!(
                        "invalid type index {} for type ", stringify!($name)), index),
                }
            }

            type ItemMut = $item_builder_name<'a>;

            fn create_mut(data : &'a mut Vec<u8>) -> Self::ItemMut
            {
                $item_builder_name{data}
            }
        }
    }
}

/// Depending on the provided flag returns the type or wrap it in `Option`.
///
/// The flag is `true`, if the resource is optional.
#[doc(hidden)]
#[macro_export]
macro_rules! opt {
    ($type:ty, false) => {
        $type
    };
    ($type:ty, true) => {
        Option<$type>
    };
}

/// Depending on whether the first argument is `true` or `false` returns the
/// first block or the second, resp.
#[doc(hidden)]
#[macro_export]
macro_rules! static_if {
    (true, $true_block:block, $false_block:block) => {
        $true_block
    };
    (false, $true_block:block, $false_block:block) => {
        $false_block
    };
}

/// Depending on the provided flag returns the result or make it an `Option`.
///
/// The flag is `true`, if the resource is optional.
#[doc(hidden)]
#[macro_export]
macro_rules! check_resource {
    ($res:expr,false) => {
        $res?
    };
    ($res:expr,true) => {
        $res.ok()
    };
}

/// Macro used by generator to define a flatdata archive and corresponding
/// archive builder.
#[macro_export]
macro_rules! define_archive {
    ($name:ident, $builder_name:ident, $archive_schema:expr;
        // struct resources
        $(($struct_resource:ident, $struct_setter:ident, $struct_type:tt, $struct_schema:expr,
            $is_optional_struct:ident)),*;
        // vector resources
        $(($vector_resource:ident, $vector_setter:ident, $vector_start:ident,
            $element_type:tt, $element_schema:expr, $is_optional_vector:ident)),*;
        // multivector resources
        $(($multivector_resource:ident,
            $multivector_start:ident,
            $variadic_type:tt, $variadic_type_schema:expr,
            $multivector_resource_index:ident, $index_type:path,
            $is_optional_multivector:ident)),*;
        // raw data resources
        $(($raw_data_resource:ident, $raw_data_resource_setter:ident,
            $raw_data_schema:expr, $is_optional_raw_data:ident)),*;
        // subarchive resources
        $(($subarchive_resource:ident,
            $subarchive_type:tt, $subarchive_builder_type:tt, $subarchive_schema:expr,
            $is_optional_subarchive:ident)),*
    ) => {

        #[derive(Clone)]
        pub struct $name {
            _storage: ::std::rc::Rc<::std::cell::RefCell<$crate::ResourceStorage>>
            $(,$struct_resource: opt!($crate::MemoryDescriptor, $is_optional_struct))*
            $(,$vector_resource: opt!($crate::MemoryDescriptor, $is_optional_vector))*
            $(,$multivector_resource: (
                opt!($crate::MemoryDescriptor, $is_optional_multivector),
                opt!($crate::MemoryDescriptor, $is_optional_multivector)))*
            $(,$raw_data_resource: opt!($crate::MemoryDescriptor, $is_optional_raw_data))*
            $(,$subarchive_resource: opt!($subarchive_type, $is_optional_subarchive))*
        }

        impl $name {
            fn read_resource<R>(
                storage: &mut $crate::ResourceStorage,
                name: &str,
                schema: &str,
            ) -> Result<R, $crate::ResourceStorageError>
            where
                R: From<$crate::MemoryDescriptor>,
            {
                storage.read(name, schema).map(|x|R::from($crate::MemoryDescriptor::new(&x)))
            }

            $(pub fn $struct_resource(&self) -> opt!(
                <$struct_type as $crate::Struct>::Item, $is_optional_struct)
            {
                static_if!($is_optional_struct, {
                    self.$struct_resource.as_ref().map(|mem_desc| {
                        <$struct_type as $crate::Struct>::create(&unsafe{mem_desc.as_bytes()})
                    })
                }, {
                    <$struct_type as $crate::Struct>::create(&unsafe{self.$struct_resource.as_bytes()})
                })
            })*

            $(pub fn $vector_resource(&self) -> opt!(
                $crate::ArrayView<$element_type>, $is_optional_vector)
            {
                static_if!($is_optional_vector, {
                    self.$vector_resource.as_ref().map(|x|$crate::ArrayView::new(unsafe{x.as_bytes()}))
                }, {
                    $crate::ArrayView::new(&unsafe{self.$vector_resource.as_bytes()})
                })
            })*

            $(pub fn $multivector_resource(&self) -> opt!(
                $crate::MultiArrayView<$index_type, $variadic_type>, $is_optional_multivector)
            {
                static_if!($is_optional_multivector, {
                    let index_mem_desc = &self.$multivector_resource.0.as_ref();
                    let res_mem_desc = &self.$multivector_resource.1.as_ref();
                    index_mem_desc
                        .map(|x|$crate::ArrayView::new(unsafe{x.as_bytes()}))
                        .and_then(move |index| {
                            res_mem_desc.map(move |mem_desc| {
                                $crate::MultiArrayView::new(index, unsafe{mem_desc.as_bytes()})
                            })
                        })
                }, {
                    $crate::MultiArrayView::new(
                        $crate::ArrayView::new(&unsafe{self.$multivector_resource.0.as_bytes()}),
                        &unsafe{self.$multivector_resource.1.as_bytes()},
                    )
                })
            })*

            $(pub fn $raw_data_resource(&self) -> opt!(&[u8], $is_optional_raw_data) {
                static_if!($is_optional_raw_data, {
                    self.$raw_data_resource.as_ref().map(|mem_desc| {
                        unsafe { mem_desc.as_bytes() }
                    })
                }, {
                    unsafe {
                        self.$raw_data_resource.as_bytes()
                    }
                })
            })*

            $(pub fn $subarchive_resource(&self) -> &opt!(
                $subarchive_type, $is_optional_subarchive)
            {
                &self.$subarchive_resource
            })*

            fn signature_name(archive_name: &str) -> String {
                format!("{}.archive", archive_name)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f,
                    concat!(stringify!($name), " {{ ",
                        intersperse!(""
                            $(, concat!(stringify!($struct_resource), ": {:?}"))*
                            $(, concat!(stringify!($vector_resource), ": {:?}"))*
                            $(, concat!(stringify!($multivector_resource), ": {:?}"))*
                            $(, concat!(stringify!($raw_data_resource), ": {:?}"))*
                            $(, concat!(stringify!($subarchive_resource), ": {:?}"))*
                        ),
                    " }}"),
                    $(self.$struct_resource(), )*
                    $(self.$vector_resource(), )*
                    $(self.$multivector_resource(), )*
                    $(self.$raw_data_resource, )*
                    $(self.$subarchive_resource, )*
                )
            }
        }

        impl $crate::Archive for $name {
            const NAME: &'static str = stringify!($name);
            const SCHEMA: &'static str = $archive_schema;

            fn open(storage: ::std::rc::Rc<::std::cell::RefCell<$crate::ResourceStorage>>)
                -> ::std::result::Result<Self, $crate::ResourceStorageError>
            {
                $(let $struct_resource;)*
                $(let $vector_resource;)*
                $(let $multivector_resource_index;
                  let $multivector_resource;)*
                $(let $raw_data_resource;)*
                {
                    let res_storage = &mut *storage.borrow_mut();
                    res_storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

                    $($struct_resource = check_resource!(
                        Self::read_resource(
                            res_storage,
                            stringify!($struct_resource),
                            $struct_schema
                        ), $is_optional_struct);
                    )*

                    $($vector_resource = check_resource!(
                        Self::read_resource(
                            res_storage,
                            stringify!($vector_resource),
                            $element_schema), $is_optional_vector);
                    )*

                    $($multivector_resource_index = check_resource!(
                        Self::read_resource(
                            res_storage,
                            stringify!($multivector_resource_index),
                            &format!("index({})", $variadic_type_schema)),
                        $is_optional_multivector);
                    $multivector_resource = check_resource!(
                        Self::read_resource(
                            res_storage,
                            stringify!($multivector_resource),
                            $variadic_type_schema), $is_optional_multivector);
                    )*

                    $($raw_data_resource = check_resource!(
                        Self::read_resource(
                            res_storage,
                            stringify!($raw_data_resource),
                            $raw_data_schema), $is_optional_raw_data);
                    )*
                }
                $(
                let $subarchive_resource = check_resource!(
                    $subarchive_type::open(
                        storage.borrow().subdir(&stringify!($subarchive_resource))),
                    $is_optional_subarchive
                );)*
                Ok(Self {
                    _storage: storage
                    $(,$struct_resource)*
                    $(,$vector_resource)*
                    $(,$multivector_resource: (
                        $multivector_resource_index,
                        $multivector_resource))*
                    $(,$raw_data_resource)*
                    $(,$subarchive_resource)*
                })
            }
        }

        #[derive(Clone)]
        pub struct $builder_name {
            storage: ::std::rc::Rc<::std::cell::RefCell<$crate::ResourceStorage>>
        }

        impl $builder_name {
            $(pub fn $struct_setter(
                &mut self,
                resource: <$struct_type as $crate::Struct>::Item,
            ) -> ::std::io::Result<()> {
                let data = unsafe {
                    ::std::slice::from_raw_parts(resource.data, <$struct_type as $crate::Struct>::SIZE_IN_BYTES)
                };
                self.storage
                    .borrow_mut()
                    .write(stringify!($struct_resource), $struct_schema, data)
            })*

            $(pub fn $vector_setter(
                &mut self,
                vector: &$crate::ArrayView<$element_type>,
            ) -> ::std::io::Result<()> {
                self.storage
                    .borrow_mut()
                    .write(stringify!($vector_resource), $element_schema, vector.as_ref())
            }

            pub fn $vector_start(
                &mut self,
            ) -> ::std::io::Result<$crate::ExternalVector<$element_type>> {
                $crate::create_external_vector(
                    &mut *self.storage.borrow_mut(),
                    stringify!($vector_resource),
                    $element_schema,
                )
            })*

            $(pub fn $multivector_start(
                &mut self,
            ) -> ::std::io::Result<
                $crate::MultiVector<$index_type, $variadic_type>
            > {
                $crate::create_multi_vector(
                    &mut *self.storage.borrow_mut(),
                    stringify!($multivector_resource),
                    $variadic_type_schema,
                )
            })*

            $(pub fn $raw_data_resource_setter(&mut self, data: &[u8]) -> ::std::io::Result<()> {
                self.storage.borrow_mut().write(
                    stringify!($raw_data_resource),
                    $raw_data_schema,
                    data,
                )
            })*

            $(pub fn $subarchive_resource(
                &mut self,
            ) -> Result<$subarchive_builder_type, $crate::ResourceStorageError> {
                use $crate::ArchiveBuilder;
                let storage = self.storage.borrow().subdir(stringify!($subarchive_resource));
                $subarchive_builder_type::new(storage)
            }
            )*
        }

        impl $crate::ArchiveBuilder for $builder_name {
            const NAME: &'static str = stringify!($name);
            const SCHEMA: &'static str = $archive_schema;

            fn new(
                storage: ::std::rc::Rc<::std::cell::RefCell<$crate::ResourceStorage>>,
            ) -> Result<Self, $crate::ResourceStorageError> {
                $crate::create_archive::<Self>(&storage)?;
                Ok(Self { storage })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::helper::Int;
    use super::super::structbuf::StructBuf;

    #[test]
    #[allow(dead_code)]
    fn test_debug() {
        define_struct!(
            A,
            RefA,
            RefMutA,
            "no_schema",
            4,
            (x, set_x, u32, 0, 16),
            (y, set_y, u32, 16, 16)
        );
        let a = StructBuf::<A>::new();
        let output = format!("{:?}", a);
        assert_eq!(output, "StructBuf { resource: A { x: 0, y: 0 } }");
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
                    (x, set_x, Variant: $type, 0, 2)
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

    #[test]
    #[allow(warnings)]
    fn test_archive_compilation() {
        // This test checks that the archive definition below compiles.
        use super::Ref;

        define_struct!(
            A,
            RefA,
            RefMutA,
            "no_schema",
            4,
            (x, set_x, u32, 0, 16),
            (y, set_y, u32, 16, 16)
        );

        define_struct!(
            B,
            RefB,
            RefMutB,
            "no_schema",
            4,
            (x, set_x, u32, 0, 16),
            (y, set_y, u32, 16, 16)
        );

        define_index!(
            IndexType32,
            RefIndexType32,
            RefMutIndexType32,
            "IndexType32 schema",
            4,
            32
        );

        define_variadic_struct!(Ts, RefTs, BuilderTs, IndexType32,
            0 => (A, add_a),
            1 => (B, add_b));

        define_archive!(SubArch, SubArchBuilder, "SubArch schema";
            ;  // struct resources
            ;  // vector resources
            ;  // multivector resources
            (raw, set_raw, "raw schema", false) // raw data resources
            ; // subarchive resources
        );

        define_archive!(Arch, ArchBuilder, "Arch schema";
            // struct resources
            (a, set_a, A, "a schema", false),
            (b, set_b, A, "b schema", true);
            // vector resources
            (v, set_v, start_v, A, "v schema", false),
            (w, set_w, start_w, A, "w schema", true);
            // multivector resources
            (mv, start_mv, Ts, "mv schema", mv_index, IndexType32, false),
            (mw, start_mw, Ts, "mw schema", mw_index, IndexType32, true);
            // raw data resources
            (r, set_r, "r schema", false),
            (s, set_s, "s schema", true);
            // subarchive resources
            (arch, SubArch, SubArchBuilder, "arch schema", false),
            (opt_arch, SubArch, SubArchBuilder, "opt_arch schema", true)
        );
    }
}
