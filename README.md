# flatdata-rs [![Build Status](https://travis-ci.org/boxdot/flatdata-rs.svg?branch=master)](https://travis-ci.org/boxdot/flatdata-rs)

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
character coappearances in Tolstoi's Anna Karenina described in `flatdata`
together with the serialized data. The example is taken from the original
[flatdata repository](https://github.com/heremaps/flatdata). It also contains a
Rust module which implements the schema. Usually, this code would be generated
automatically by `flatdata`'s generator.

The standalone tests read and write all different types of data and check that
the data was de/serialized correctly. Run them simply with:

```shell
cargo test
```

Implementation of [heremaps/flatdata](https://github.com/heremaps/flatdata) in Rust.

## TODO

Generator:

* [ ] `flatdata`'s schema to Rust generator (implemented, but the code is not
  yet merge into the `flatdata` main repository).
* [ ] Support for namespaces.
* [ ] Support for constants.

## License

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT License ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
