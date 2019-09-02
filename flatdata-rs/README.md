# flatdata-rs [![latest version]][crates.io] [![docs]][docs.rs]

_Write-once, read-many, minimal overhead binary structured file format._

Flatdata is a library providing data structures for convenient creation,
storage and access of packed memory-mappable structures with minimal overhead.

With `flatdata`, the user defines a schema of the data format using a very
simple schema language that supports plain structs, vectors and multivectors.
The schema is then used to generate builders and readers for serialization and
deserialization of the data to an archive of files on disk.

The data is serialized in a portable way which allows zero-overhead random
access to it by using memory mapped storage: the operating system facilities
are used for loading, caching and paging of the data, and most important,
accessing it as if it were in memory.

## Example

The folder [tests/coappearances] contains a graph of character coappearances in
Tolstoi's _Anna Karenina_ as flatdata [archive schema] together with the
serialized data. The standalone tests read and write all different types of
data and check that the data was de/serialized correctly.

## Getting started

We recommend that you use a `build.rs` to automatically generate the code from your `flatdata` schema.

```rust
// build.rs
fn main() {
    flatdata::generate(
        "path/to/my_schema.flatdata",
        &std::env::var("OUT_DIR").unwrap(),
    )
    .expect("generator failed");
}
```

Then add a module to your crate and include the generated code:

```rust
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "path/to/my_schema.rs"));
// re-export if desired
pub use my_schema::*;
```

See the [documentation of generator.rs] for a more detailed explaination.

[travis]: https://travis-ci.org/heremaps/flatdata-rs
[travis status]: https://travis-ci.org/heremaps/flatdata-rs.svg?branch=master
[latest version]: https://img.shields.io/crates/v/flatdata.svg
[crates.io]: https://crates.io/crates/flatdata
[docs]: https://docs.rs/flatdata/badge.svg
[docs.rs]: https://docs.rs/flatdata/
[archive schema]: tests/coappearances/assets/coappearances.flatdata
[tests/coappearances]: tests/coappearances
[documentation of generator.rs]: lib/src/generator.rs
