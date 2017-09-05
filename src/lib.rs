extern crate memmap;

#[macro_use]
mod byte_reader;
mod archive;
mod array_view;
mod file_storage;
pub mod storage;

pub use archive::*;
pub use array_view::ArrayView;
pub use byte_reader::StreamType;
pub use file_storage::FileResourceStorage;
pub use storage::ResourceStorage;
