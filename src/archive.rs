use bytereader::StreamType;
use std::cell::RefCell;
use std::convert;
use std::rc::Rc;

use storage::ResourceStorage;

/// Element in an Archive, which can be a struct or a (sub)archive itself.
pub trait ArchiveElement {
    const NAME: &'static str;
    const SCHEMA: &'static str;
}

/// A type in archive.
pub trait ArchiveType: ArchiveElement + convert::From<StreamType> {
    const SIZE_IN_BYTES: usize;
}

/// An archive.
pub trait Archive: ArchiveElement {
    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Self;
    fn is_open(&self) -> bool;
    fn describe(&self) -> String;
}

//
// Generator macros
//

#[macro_export]
macro_rules! intersperse {
    ($head:expr) => {$head};
    ($head:expr, $($tail:expr),+) => (concat!($head, ", ", $($tail),*));
}

#[macro_export]
macro_rules! define_resource_type {
    ($name:ident, $name_str:expr, $schema_str:expr, $size_in_bytes:expr
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

        impl ArchiveElement for $name {
            const NAME: &'static str = $name_str;
            const SCHEMA: &'static str = $schema_str;
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
