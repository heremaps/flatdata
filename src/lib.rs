extern crate diff;
extern crate memmap;

#[macro_use]
mod bytereader;
#[macro_use]
mod archive;
mod arrayview;
mod error;
mod filestorage;
mod memstorage;
mod multiarrayview;
mod storage;

pub use archive::*;
pub use arrayview::ArrayView;
pub use bytereader::StreamType;
pub use error::*;
pub use filestorage::FileResourceStorage;
pub use memstorage::MemoryResourceStorage;
pub use multiarrayview::MultiArrayView;
pub use storage::{MemoryDescriptor, ResourceStorage};
