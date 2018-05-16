extern crate diff;
extern crate memmap;

/// Number of elements in `ArrayView`, `MultiArrayView`, and `Vector` to show in Debug output.
const DEBUG_PREVIEW_LEN: usize = 10;

#[macro_use]
pub mod helper;
#[macro_use]
mod bytereader;
#[macro_use]
mod bytewriter;
#[macro_use]
mod archive;
mod arrayview;
mod error;
mod filestorage;
mod handle;
mod memory;
mod memstorage;
mod multiarrayview;
mod multivector;
mod storage;
mod structbuf;
mod vector;

mod test_structs;

pub use archive::*;
pub use arrayview::ArrayView;
pub use error::*;
pub use filestorage::FileResourceStorage;
pub use handle::*;
pub use memory::PADDING_SIZE;
pub use memstorage::MemoryResourceStorage;
pub use multiarrayview::MultiArrayView;
pub use multivector::MultiVector;
pub use storage::{
    create_archive, create_external_vector, create_multi_vector, MemoryDescriptor, ResourceStorage,
};
pub use structbuf::StructBuf;
pub use vector::*;
