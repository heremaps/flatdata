use bytereader::StreamType;
use std::cell::RefCell;
use std::convert;
use std::rc::Rc;
use std::fmt;

use storage::ResourceStorage;
use error::ResourceStorageError;

/// A type in archive.
pub trait Struct: convert::From<StreamType> + fmt::Debug + Clone + PartialEq {
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
    : convert::From<(TypeIndex, StreamType)> + fmt::Debug + Clone + PartialEq {
    fn size_in_bytes(&self) -> usize;
}

/// An archive.
pub trait Archive: fmt::Debug + Clone {
    const NAME: &'static str;
    const SCHEMA: &'static str;

    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError>
    where
        Self: Sized;
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
        $(,($field:ident, $type:tt, $offset:expr, $bit_size:expr))*) =>
    {
        #[derive(Clone)]
        pub struct $name {
            data: ::flatdata::StreamType,
        }

        impl $name {
            $(pub fn $field(&self) -> $type {
                read_bytes!($type, self.data, $offset, $bit_size)
            })*
        }

        impl ::std::convert::From<::flatdata::StreamType> for $name {
            fn from(data: ::flatdata::StreamType) -> Self {
                Self { data: data }
            }
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

        impl ::flatdata::Struct for $name {
            const SCHEMA: &'static str = $schema;
            const SIZE_IN_BYTES: usize = $size_in_bytes;
        }
    }
}

#[macro_export]
macro_rules! define_index {
    ($name:ident, $schema:path, $size_in_bytes:expr, $size_in_bits:expr) => {
        mod internal {
            define_struct!($name, $schema, $size_in_bytes, (value, u64, 0, $size_in_bits));

            impl ::flatdata::Index for $name {
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
        #[derive(Clone, PartialEq)]
        pub enum $name {
            $($type($type),)*
        }

        impl ::std::convert::From<(::flatdata::TypeIndex, ::flatdata::StreamType)> for $name {
            fn from((type_index, data): (::flatdata::TypeIndex, ::flatdata::StreamType)) -> Self {
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

        impl ::flatdata::VariadicStruct for $name {
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
            _storage: ::std::rc::Rc<::std::cell::RefCell<::flatdata::ResourceStorage>>
            $(,$struct_resource: $struct_type)*
            $(,$vector_resource: ::flatdata::ArrayView<$element_type>)*
            $(,$multivector_resource: ::flatdata::MultiArrayView<$index_type, $variadic_type>)*
            $(,$raw_data_resource: ::flatdata::MemoryDescriptor)*
        }

        impl $name {
            fn read_resource<R>(
                storage: &mut ::flatdata::ResourceStorage,
                name: &str,
                schema: &str,
            ) -> Result<R, ::flatdata::ResourceStorageError>
            where
                R: From<::flatdata::MemoryDescriptor>,
            {
                storage.read(name, schema).map(R::from)
            }

            $(pub fn $struct_resource(&self) -> &$struct_type {
                &self.$struct_resource
            })*

            // TODO: It should be ArrayView annotated with a life-time and not a ref to an
            // ArrayView.
            $(pub fn $vector_resource(&self) -> &::flatdata::ArrayView<$element_type> {
                &self.$vector_resource
            })*

            // TODO: It should be MultiArrayView annotated with a life-time and not a ref to an
            // MultiArrayView.
            $(pub fn $multivector_resource(&self)
                    -> &::flatdata::MultiArrayView<$index_type, $variadic_type> {
                &self.$multivector_resource
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

        impl ::flatdata::Archive for $name {
            const NAME: &'static str = stringify!($name);
            const SCHEMA: &'static str = $archive_schema;

            fn open(storage: ::std::rc::Rc<::std::cell::RefCell<::flatdata::ResourceStorage>>)
                -> ::std::result::Result<Self, ::flatdata::ResourceStorageError>
            {
                $(let $struct_resource;)*
                $(let $vector_resource;)*
                $(let $multivector_resource;)*
                $(let $raw_data_resource;)*
                {
                    let res_storage = &mut *storage.borrow_mut();
                    res_storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

                    $($struct_resource = Self::read_resource(
                        res_storage,
                        stringify!($struct_resource),
                        $struct_schema
                    ).map(|mem: ::flatdata::MemoryDescriptor| $struct_type::from(mem.data()))?;
                    )*

                    $($vector_resource = Self::read_resource(
                        res_storage,
                        stringify!($vector_resource),
                        $element_schema
                    ).map(|mem| ::flatdata::ArrayView::new(&mem))?;
                    )*

                    $(let $multivector_resource_index = Self::read_resource(
                        res_storage,
                        stringify!($multivector_resource_index),
                        $index_schema
                    ).map(|mem| ::flatdata::ArrayView::new(&mem))?;
                    $multivector_resource = Self::read_resource(
                        res_storage,
                        stringify!($multivector_resource),
                        $variadic_type_schema
                    ).map(|mem| ::flatdata::MultiArrayView::new(
                        $multivector_resource_index, &mem))?;
                    )*

                    $($raw_data_resource = Self::read_resource(
                        res_storage,
                        stringify!($raw_data_resource),
                        $raw_data_schema)?;
                    )*
                }
                Ok(Self {
                    _storage: storage
                    $(,$struct_resource: $struct_resource)*
                    $(,$vector_resource: $vector_resource)*
                    $(,$multivector_resource: $multivector_resource)*
                    $(,$raw_data_resource: $raw_data_resource)*
                })
            }
        }
    }
}
