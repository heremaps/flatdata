extern crate memmap;

#[macro_use]
mod byte_reader;
pub mod storage;
mod file_storage;
mod archive;

pub use storage::ResourceStorage;
pub use file_storage::FileResourceStorage;
pub use archive::Archive;
