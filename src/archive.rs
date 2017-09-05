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
