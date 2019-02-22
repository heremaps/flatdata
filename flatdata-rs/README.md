# flatdata-rs [![latest version]][crates.io] [![docs]][docs.rs]

Rust implementation.

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

[travis]: https://travis-ci.org/boxdot/flatdata-rs
[travis status]: https://travis-ci.org/boxdot/flatdata-rs.svg?branch=master
[latest version]: https://img.shields.io/crates/v/flatdata.svg
[crates.io]: https://crates.io/crates/flatdata
[docs]: https://docs.rs/flatdata/badge.svg
[docs.rs]: https://docs.rs/flatdata/
[Why flatdata?]: https://github.com/heremaps/flatdata/blob/master/docs/src/why-flatdata.rst
