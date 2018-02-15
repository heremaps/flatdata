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

pub trait VariadicArchiveType: convert::From<(u8, StreamType)> {
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

        impl ArchiveType for $name {
            const SIZE_IN_BYTES: usize = $size_in_bytes;
        }

        impl convert::From<StreamType> for $name {
            fn from(data: StreamType) -> Self {
                Self { data: data }
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f,
                    concat!("{} {{ ",
                        intersperse!($(concat!( stringify!($field), ": {}")), *), " }}"),
                    stringify!($name), $(self.$field(),)*)
            }
        }
    }
}
