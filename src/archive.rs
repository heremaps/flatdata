use std::cell::RefCell;
use std::convert::From;
use std::fmt::Debug;
use std::rc::Rc;

use error::ResourceStorageError;
use storage::ResourceStorage;

/// A type in archive.
pub trait Struct: Clone + Debug + PartialEq + From<*const u8> {
    /// Schema of the type. Only for debug and inspection purposes.
    const SCHEMA: &'static str;
    /// Size of an object of this type in bytes.
    const SIZE_IN_BYTES: usize;

    type Mut: StructMut + AsRef<Self>;

    fn as_ptr(&self) -> *const u8;
}

pub trait StructMut: Debug + From<*mut u8> {
    type Const: Struct;

    fn as_mut_ptr(&mut self) -> *mut u8;
}

/// A type in archive used as index of a `MultiArrayView`.
pub trait Index: Struct {
    type IndexMut: IndexMut;
    fn value(&self) -> usize;
}

/// A type in archive used as mutable index of a `MultiVector`.
pub trait IndexMut: StructMut {
    fn set_value(&mut self, value: usize);
}

/// Index specifying a variadic type of `MultiArrayView`.
pub type TypeIndex = u8;

/// A type used as element of `MultiArrayView`.
pub trait VariadicStruct: Clone + Debug + PartialEq + From<(TypeIndex, *const u8)> {
    type ItemBuilder: From<*mut Vec<u8>>;

    fn size_in_bytes(&self) -> usize;
}

/// A flatdata archive representing serialized data.
pub trait Archive: Debug + Clone {
    const NAME: &'static str;
    const SCHEMA: &'static str;

    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError>;
}

/// A flatdata archive builder for serializing data.
pub trait ArchiveBuilder: Clone {
    const NAME: &'static str;
    const SCHEMA: &'static str;

    fn new(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError>;
}

//
// Generator macros
//

#[macro_export]
macro_rules! intersperse {
    ($head:expr) => {$head};
    ($head:expr, $($tail:expr),+) => (concat!($head, ", ", intersperse!($($tail),*)));
}

#[macro_export]
macro_rules! define_struct {
    ($name:ident, $name_mut:ident, $schema:expr, $size_in_bytes:expr
        $(,($field:ident, $field_setter:ident, $type:tt, $offset:expr, $bit_size:expr))*) =>
    {
        // TODO: We cannot store &u8 here, since then we need to annote the type with a lifetime,
        // which would enforce an annotation in the trait, and this would bind the lifetime at the
        // creating of containers as ArrayView, etc... When meta-types are introduced (i.e. when
        // we can express that a container is parametrized over a meta-type with a lifetime bound
        // later), we can refactor this and get rid of Handle and HandleMut.
        #[derive(Clone)]
        pub struct $name {
            data: *const u8,
        }

        impl $name {
            $(pub fn $field(&self) -> $type {
                read_bytes!($type, self.data, $offset, $bit_size)
            })*
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f,
                    concat!("{} {{ ",
                        intersperse!($(concat!( stringify!($field), ": {}")), *), " }}"),
                    stringify!($name), $(self.$field(),)*)
            }
        }

        impl ::std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                $(self.$field() == other.$field()) && *
            }
        }

        impl ::std::convert::From<*const u8> for $name {
            fn from(data: *const u8) -> Self {
                Self { data }
            }
        }

        impl $crate::Struct for $name {
            const SCHEMA: &'static str = $schema;
            const SIZE_IN_BYTES: usize = $size_in_bytes;

            type Mut = $name_mut;

            fn as_ptr(&self) -> *const u8 {
                self.data
            }
        }

        pub struct $name_mut {
            data: *mut u8,
        }

        impl $name_mut {
            $(pub fn $field_setter(&mut self, value: $type) {
                let buffer = unsafe {
                    ::std::slice::from_raw_parts_mut(self.data, $size_in_bytes)
                };
                write_bytes!($type; value, buffer, $offset, $bit_size)
            })*

            pub fn fill_from(&mut self, other: &$name) {
                $(self.$field_setter(other.$field());)*
            }
        }

        impl ::std::fmt::Debug for $name_mut {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.as_ref().fmt(f)
            }
        }

        impl ::std::convert::From<*mut u8> for $name_mut {
            fn from(data: *mut u8) -> Self {
                Self { data }
            }
        }

        impl $crate::StructMut for $name_mut {
            type Const = $name;

            fn as_mut_ptr(&mut self) -> *mut u8 {
                self.data
            }
        }

        impl ::std::convert::AsRef<$name> for $name_mut {
            fn as_ref(&self) -> &$name {
                unsafe { ::std::mem::transmute(&self) }
            }
        }
     }
}

#[macro_export]
macro_rules! define_index {
    ($name: ident, $name_mut: ident, $schema: path, $size_in_bytes: expr, $size_in_bits: expr) => {
        mod internal {
            define_struct!(
                $name,
                $name_mut,
                $schema,
                $size_in_bytes,
                (value, set_value, u64, 0, $size_in_bits)
            );

            impl $crate::Index for $name {
                type IndexMut = $name_mut;
                fn value(&self) -> usize {
                    self.value() as usize
                }
            }

            impl $crate::IndexMut for $name_mut {
                fn set_value(&mut self, value: usize) {
                    self.set_value(value as u64);
                }
            }
        }
    };
}

#[macro_export]
macro_rules! define_variadic_struct {
    ($name:ident, $item_builder_name:ident, $index_type:tt,
        $($type_index:expr => ($type:tt, $add_type_fn:ident)),+) =>
    {
        #[derive(Clone, PartialEq)]
        pub enum $name {
            $($type($type),)*
        }

        impl ::std::convert::From<(
            $crate::TypeIndex, $crate::bytereader::StreamType)> for $name {
            fn from((type_index, data):
                ($crate::TypeIndex, $crate::bytereader::StreamType)) -> Self {
                match type_index {
                    $($type_index => $name::$type($type::from(data))),+,
                    _ => panic!(
                        "invalid type index {} for type {}", type_index, stringify!($name)),
                }
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($name::$type(ref inner) => write!(f, "{:?}", inner)),+
                }
            }
        }

        impl $crate::VariadicStruct for $name {
            type ItemBuilder = $item_builder_name;

            fn size_in_bytes(&self) -> usize {
                match *self {
                    $($name::$type(_) => $type::SIZE_IN_BYTES),+
                }
            }
        }

        pub struct $item_builder_name {
            data: *mut Vec<u8>
        }

        impl $item_builder_name {
            $(pub fn $add_type_fn(&mut self)
                -> $crate::handle::HandleMut<<$type as ::flatdata::Struct>::Mut>
            {
                let data = unsafe { &mut *self.data };
                let old_len = data.len();
                let increment = 1 + $type::SIZE_IN_BYTES;
                data.resize(old_len + increment, 0);
                data[old_len - ::flatdata::memory::PADDING_SIZE] = $type_index;
                $crate::handle::HandleMut::new(<$type as ::flatdata::Struct>::Mut::from(
                    &mut data[1 + old_len - ::flatdata::memory::PADDING_SIZE] as *mut _
                ))
            })*
        }

        impl<'a> ::std::convert::From<*mut Vec<u8>> for $item_builder_name {
            fn from(data: *mut Vec<u8>) -> Self {
                Self { data }
            }
        }
    }
}

#[macro_export]
macro_rules! define_archive {
    ($name:ident, $archive_schema:expr;
        $(($struct_resource:ident, $struct_type:tt, $struct_schema:expr)),*;
        $(($vector_resource:ident, $element_type:tt, $element_schema:expr)),*;
        $(($multivector_resource:ident, $variadic_type:tt, $variadic_type_schema:expr,
            $multivector_resource_index:ident, $index_type:path, $index_schema:expr)),*;
        $(($raw_data_resource:ident, $raw_data_schema:expr)),*
    ) => {

        #[derive(Clone)]
        pub struct $name {
            _storage: ::std::rc::Rc<::std::cell::RefCell<$crate::ResourceStorage>>
            $(,$struct_resource: $crate::MemoryDescriptor)*
            $(,$vector_resource: $crate::MemoryDescriptor)*
            $(,$multivector_resource: ($crate::MemoryDescriptor, $crate::MemoryDescriptor))*
            $(,$raw_data_resource: $crate::MemoryDescriptor)*
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
                storage.read(name, schema).map(R::from)
            }

            $(pub fn $struct_resource(&self) -> ::flatdata::handle::Handle<$struct_type> {
                ::flatdata::handle::Handle::new($struct_type::from(self.$struct_resource.data()))
            })*

            $(pub fn $vector_resource(&self) -> $crate::ArrayView<$element_type> {
                $crate::ArrayView::new(&self.$vector_resource)
            })*

            $(pub fn $multivector_resource(&self)
                    -> $crate::MultiArrayView<$index_type, $variadic_type> {
                $crate::MultiArrayView::new(
                    $crate::ArrayView::new(&self.$multivector_resource.0),
                    &self.$multivector_resource.1,
                )
            })*

            $(pub fn $raw_data_resource(&self) -> &[u8] {
                unsafe {
                    ::std::slice::from_raw_parts(
                        self.$raw_data_resource.data(),
                        self.$raw_data_resource.size_in_bytes())
                }
            })*

            fn signature_name(archive_name: &str) -> String {
                format!("{}.archive", archive_name)
            }
        }

        impl ::std::fmt::Debug for Graph {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f,
                    concat!("{} {{ ",
                        intersperse!(
                            intersperse!($(concat!(stringify!($struct_resource), ": {:?}")),*),
                            intersperse!($(concat!(stringify!($vector_resource), ": {:?}")),*),
                            intersperse!($(concat!(stringify!($multivector_resource), ": {:?}")),*),
                            intersperse!($(concat!(stringify!($raw_data_resource), ": {:?}")),*)
                        ),
                    " }}", ),
                    stringify!($name)
                    $(, self.$struct_resource())*
                    $(, self.$vector_resource())*
                    $(, self.$multivector_resource())*
                    $(, self.$raw_data_resource)*
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

                    $($struct_resource = Self::read_resource(
                        res_storage,
                        stringify!($struct_resource),
                        $struct_schema
                    )?;
                    )*

                    $($vector_resource = Self::read_resource(
                        res_storage,
                        stringify!($vector_resource),
                        $element_schema
                    )?;
                    )*

                    $($multivector_resource_index = Self::read_resource(
                        res_storage,
                        stringify!($multivector_resource_index),
                        $index_schema
                    )?;
                    $multivector_resource = Self::read_resource(
                        res_storage,
                        stringify!($multivector_resource),
                        $variadic_type_schema
                    )?;
                    )*

                    $($raw_data_resource = Self::read_resource(
                        res_storage,
                        stringify!($raw_data_resource),
                        $raw_data_schema)?;
                    )*
                }
                Ok(Self {
                    _storage: storage
                    $(,$struct_resource)*
                    $(,$vector_resource)*
                    $(,$multivector_resource: (
                        $multivector_resource_index,
                        $multivector_resource))*
                    $(,$raw_data_resource)*
                })
            }
        }
    }
}
