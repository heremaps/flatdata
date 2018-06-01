# flatdata-rs [![travis status]][travis] [![latest version]][crates.io] [![docs]][docs.rs]

Implementation of [heremaps/flatdata](https://github.com/heremaps/flatdata) in
Rust.

Flatdata is a library providing data structures for convenient creation,
storage and access of packed memory-mappable structures with minimal overhead.

The idea is, that the user defines a schema of the data format using flatdata's
very simple schema language supporting plain structs, vectors, and
multivectors. The schema is then used to generate builders and readers for
serialization and deserialization of the data. The data is serialized in a
portable way which allows zero-overhead random access to it by using memory
mapped storage. Memory mapped approach makes it possible to use the operating
system facilities for loading, caching and paging of the data, and most
important, accessing it as if it were in memory. Read more in "[Why
flatdata?]".

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

## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[travis]: https://travis-ci.org/boxdot/flatdata-rs
[travis status]: https://travis-ci.org/boxdot/flatdata-rs.svg?branch=master
[latest version]: https://img.shields.io/crates/v/flatdata.svg
[crates.io]: https://crates.io/crates/flatdata
[docs]: https://docs.rs/flatdata/badge.svg
[docs.rs]: https://docs.rs/flatdata/
[Why flatdata?]: https://github.com/heremaps/flatdata/blob/master/docs/src/why-flatdata.rst
