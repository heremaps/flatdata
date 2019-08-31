# flatdata-rs [![latest version]][crates.io] [![docs]][docs.rs]

Rust implementation of [flatdata](https://github.com/heremaps/flatdata),
a write-once, read-many, minimal overhead binary structured file format.

## Example

The folder [tests/coappearances](tests/coappearances) contains a graph of
character coappearances in Tolstoi's _Anna Karenina_ as flatdata archive schema
together with the serialized data. The example is taken from the original
[flatdata repository](https://github.com/heremaps/flatdata). It also contains a
Rust module which implements the schema. Usually, this code would be generated
automatically by flatdata's generator.

The standalone tests read and write all different types of data and check that
the data was de/serialized correctly. Run them simply with:

```shell
cargo test
```

## Getting started

We recommend that you use a `build.rs` to automatically generate the code from your `flatdata` schema.

```rust
fn main() {
    flatdata::generate("path/to/my_schema.flatdata").unwrap();
}
```

```rust
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "path/to/my_schema.rs"));
///
// re-export if desired
pub use my_schema::*;
```

See the [documentation of generator.rs](lib/generator.rs) for a more detailed explaination.

[travis]: https://travis-ci.org/heremaps/flatdata-rs
[travis status]: https://travis-ci.org/heremaps/flatdata-rs.svg?branch=master
[latest version]: https://img.shields.io/crates/v/flatdata.svg
[crates.io]: https://crates.io/crates/flatdata
[docs]: https://docs.rs/flatdata/badge.svg
[docs.rs]: https://docs.rs/flatdata/
[Why flatdata?]: https://github.com/heremaps/flatdata/blob/master/docs/why-flatdata.md
