//! Implementation of [heremaps/flatdata] in Rust.
//!
//! Flatdata is a library providing data structures for convenient creation,
//! storage and access of packed memory-mappable structures with minimal
//! overhead.
//!
//! The idea is, that the user defines a schema of the data format using
//! flatdata's very simple schema language supporting plain structs, vectors
//! and multivectors. The schema is then used to generate builders and readers
//! for serialization and deserialization of the data. The data is serialized
//! in a portable way which allows zero-overhead random access to it by using
//! memory mapped storage. Memory mapped approach makes it possible to use the
//! operating system facilities for loading, caching and paging of the data,
//! and most important, accessing it as if it were in memory. Read more in
//! "[Why flatdata?]".
//!
//! This create provides:
//!
//! * macros for generated code (prefixed with `create_`),
//! * macros for zero-cost serialization [`read_bytes`] and deserialization
//! [`write_bytes`], * in-memory [`MemoryResourceStorage`] and memory-mapped
//! [`FileResourceStorage`] storages, * data structures for writing data:
//! [`StructBuf`], [`Vector`], [`ExternalVector`],   [`MultiVector`],
//! * data structures for reading data: [`ArrayView`], [`MultiArrayView`].
//!
//! The generator is part of the main [heremaps/flatdata] repository.
//!
//! For a comprehensive example, cf. coappearances [generated code] and the
//! corresponding [usage].
//!
//! [heremaps/flatdata]: https://github.com/heremaps/flatdata
//! [generated code]: https://github.com/boxdot/flatdata-rs/blob/master/tests/coappearances/generated.rs
//! [usage]: https://github.com/boxdot/flatdata-rs/blob/master/tests/coappearances_test.rs
//! [Why flatdata?]: https://github.com/heremaps/flatdata/blob/master/docs/src/why-flatdata.rst
//! [`read_bytes`]: macro.read_bytes.html
//! [`write_bytes`]: macro.write_bytes.html
//! [`MemoryResourceStorage`]: struct.MemoryResourceStorage.html
//! [`FileResourceStorage`]: struct.FileResourceStorage.html
//! [`StructBuf`]: struct.StructBuf.html
//! [`Vector`]: struct.Vector.html
//! [`ExternalVector`]: struct.ExternalVector.html
//! [`MultiVector`]: struct.MultiVector.html
//! [`ArrayView`]: struct.ArrayView.html
//! [`MultiArrayView`]: struct.MultiArrayView.html

#![deny(missing_docs, missing_debug_implementations, warnings)]
// #![allow(intra_doc_link_resolution_failure)]

extern crate diff;
extern crate memmap;

/// Number of elements in `ArrayView`, `MultiArrayView`, and `Vector` to show
/// in Debug output.
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
