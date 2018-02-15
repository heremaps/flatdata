use bytereader::StreamType;
use std::cell::RefCell;
use std::convert;
use std::rc::Rc;
use std::fmt;

use storage::ResourceStorage;
use error::ResourceStorageError;

/// A type in archive.
pub trait ArchiveType: convert::From<StreamType> + fmt::Debug {
    const SIZE_IN_BYTES: usize;
}

/// A type in archive used as index of a multivector.
pub trait IndexType: ArchiveType {
    fn value(&self) -> usize;
}

/// Index specifying a variadic type of MultiArrayView.
pub type TypeIndex = u8;

/// A type used as element of MultiArrayView.
pub trait VariadicArchiveType: convert::From<(TypeIndex, StreamType)> {
    fn size_in_bytes(&self) -> usize;
}

/// An archive.
pub trait Archive: fmt::Debug {
    const NAME: &'static str;
    const SCHEMA: &'static str;

    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError>
    where
        Self: Sized;
    fn name(&self) -> &str;
    fn schema(&self) -> &str;
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
macro_rules! define_archive_type {
    ($name:ident, $size_in_bytes:expr
        $(,($field:ident, $type:tt, $offset:expr, $bit_size:expr))*) =>
    {
        pub struct $name {
            data: ::flatdata::StreamType,
        }

        impl $name {
            $(pub fn $field(&self) -> $type {
                read_bytes!($type, self.data, $offset, $bit_size)
            })*
        }

        impl ::flatdata::ArchiveType for $name {
            const SIZE_IN_BYTES: usize = $size_in_bytes;
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
    }
}

#[macro_export]
macro_rules! define_index_type {
    ($name:ident, $size_in_bytes:expr, $size_in_bits:expr) => {
        mod internal {
            define_archive_type!($name, $size_in_bytes, (value, u64, 0, $size_in_bits));

            impl ::flatdata::IndexType for $name {
               fn value(&self) -> usize {
                    self.value() as usize
                }
            }
        }
    }
}

#[macro_export]
macro_rules! define_variadic_archive_type {
    ($name:ident, $index_type:tt, $($type_index:expr => $type:tt),+) =>
    {
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

        impl ::flatdata::VariadicArchiveType for $name {
            fn size_in_bytes(&self) -> usize {
                match *self {
                    $($name::$type(_) => $type::SIZE_IN_BYTES),+
                }
            }
        }
    }
}
