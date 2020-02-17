//! Test structures and archives used in unit tests of the library.
//!
//! The test structs are generated from the underlying flatdata file.
//! However, to make it easier to understand the unit tests, the
//! generated code is checked in and is not generated on the fly.
//! Whenever the generator changes this code should be updated manually
//! and checked in.
//!
//! Generate it by using the following command from the `flatdata-rs`
//! folder:
//!
//! ```shell
//! ../generator -g rust -s lib/src/test/test.flatdata -O lib/src/test/test_generated.rs && \
//! sed -i.bak -e s/flatdata::/crate::/g lib/src/test/test_generated.rs
//! ```
//!

// TODO: Implement missing docs
// TODO: Implement missing debugs
#![allow(dead_code, missing_debug_implementations, missing_docs)]

include!("test_generated.rs");

pub use test::*;
