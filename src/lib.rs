extern crate memmap;

#[macro_use]
mod bytereader;
#[macro_use]
mod archive;
mod arrayview;
mod filestorage;
mod memstorage;
mod storage;

pub use archive::*;
pub use arrayview::ArrayView;
pub use bytereader::StreamType;
pub use filestorage::FileResourceStorage;
pub use memstorage::MemoryResourceStorage;
pub use storage::{MemoryDescriptor, ResourceStorage};
