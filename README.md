# flatdata-rs [![Build Status](https://travis-ci.org/boxdot/flatdata-rs.svg?branch=master)](https://travis-ci.org/boxdot/flatdata-rs)

Implementation of [heremaps/flatdata](https://github.com/heremaps/flatdata) in Rust.

## Progress

Reader:

* [x] File resource storage
* [x] Memory resource storage
* [x] Struct reader
* [x] ArrayView
* [x] MultiArrayView
* [x] Raw data view
* [ ] Subarchives

Writer:

* [ ] File resource storage
* [ ] Memory resource storage
* [ ] Struct writer
* [ ] Vector
* [ ] ExternalVector
* [ ] MultiVector
* [ ] Raw data writer
* [ ] Subarchives

Generator:

* [ ] `flatdata`'s schema to Rust generator

Misc:

* [x] Derive partial eq comparsion for all structs.
* [x] Rename `size` of all containers into `len`.
* [x] Rename `index` of containers to `at`.

## Example

The folder [tests/coappearances](tests/coappearances) contains a graph of character coappearances in Tolstoi's Anna Karenina described in `flatdata` together with the serialized data. The example is from the original [flatdata repository](https://github.com/heremaps/flatdata). It also contains a Rust module which implements the schema. Usually, this code would be generated automatically by `flatdata`'s generator. For now, we use it as a proof of concept to develop readers and writers, and also to manually define a desired layout we would generate automatically.

The tests reads all types of different data and checks that it was deserialized correctly. It can be also considered as a reader example. Run it as follows:

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
