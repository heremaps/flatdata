use bytereader;
use std::cell::RefCell;
use std::convert;
use std::rc::Rc;
use std::fmt;

use storage::ResourceStorage;
use error::ResourceStorageError;

/// A type in archive.
pub trait Struct: fmt::Debug + PartialEq {
    /// Schema of the type. Only for debug and inspection purposes.
    const SCHEMA: &'static str;
    /// Size of an object of this type in bytes.
    const SIZE_IN_BYTES: usize;
}

/// A type in archive used as index of a multivector.
pub trait Index: Struct {
    fn value(&self) -> usize;
}

/// Index specifying a variadic type of `MultiArrayView`.
pub type TypeIndex = u8;

/// A type used as element of `MultiArrayView`.
pub trait VariadicStruct
    : convert::From<(TypeIndex, bytereader::StreamType)> + fmt::Debug + PartialEq
    {
    fn size_in_bytes(&self) -> usize;
}

/// An archive.
pub trait Archive: fmt::Debug + Clone {
    const NAME: &'static str;
    const SCHEMA: &'static str;

    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError>;
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
    ($name:ident, $schema:expr, $size_in_bytes:expr
        $(,($field:ident, $field_setter:ident, $type:tt, $offset:expr, $bit_size:expr))*) =>
    {
        #[repr(C)]
        pub struct $name {
            first_byte: u8,
        }

        impl $name {
            $(pub fn $field(&self) -> $type {
                read_bytes!($type, &self.first_byte, $offset, $bit_size)
            })*

            $(pub fn $field_setter(&mut self, value: $type) {
                let buffer = unsafe {
                    ::std::slice::from_raw_parts_mut(&mut self.first_byte, $size_in_bytes)
                };
                write_bytes!($type; value, buffer, $offset, $bit_size)
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

        impl $crate::Struct for $name {
            const SCHEMA: &'static str = $schema;
            const SIZE_IN_BYTES: usize = $size_in_bytes;
        }
    }
}

#[macro_export]
macro_rules! define_index {
    ($name:ident, $schema:path, $size_in_bytes:expr, $size_in_bits:expr) => {
        mod internal {
            define_struct!(
                $name, $schema, $size_in_bytes,
                (value, set_value, u64, 0, $size_in_bits));

            impl $crate::Index for $name {
               fn value(&self) -> usize {
                    self.value() as usize
                }
            }
        }
    }
}

#[macro_export]
macro_rules! define_variadic_struct {
    ($name:ident, $index_type:tt, $($type_index:expr => $type:tt),+) =>
    {
        #[derive(PartialEq)]
        pub enum $name<'a> {
            $($type(&'a $type),)*
        }

        impl<'a> ::std::convert::From<(
            $crate::TypeIndex, $crate::bytereader::StreamType)> for $name<'a> {
            fn from((type_index, data):
                ($crate::TypeIndex, $crate::bytereader::StreamType)) -> Self {
                match type_index {
                    $($type_index => unsafe {
                        $name::$type(&*(data as *const $type))
                    }),+,
                    _ => panic!(
                        "invalid type index {} for type {}", type_index, stringify!($name)),
                }
            }
        }

        impl<'a> ::std::fmt::Debug for $name<'a> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($name::$type(ref inner) => write!(f, "{:?}", inner)),+
                }
            }
        }

        impl<'a> $crate::VariadicStruct for $name<'a> {
            fn size_in_bytes(&self) -> usize {
                match *self {
                    $($name::$type(_) => $type::SIZE_IN_BYTES),+
                }
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

            $(pub fn $struct_resource(&self) -> &$struct_type {
                unsafe {
                    &*(self.$struct_resource.data() as *const $struct_type)
                }
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
