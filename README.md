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
* [x] Subarchives

Writer:

* [x] File resource storage
* [x] Memory resource storage
* [x] Struct writer
* [x] Vector
* [x] ExternalVector
* [x] MultiVector
* [x] Raw data writer
* [x] Subarchives

Generator:

* [ ] `flatdata`'s schema to Rust generator

## Example

The folder [tests/coappearances](tests/coappearances) contains a graph of character coappearances in Tolstoi's Anna Karenina described in `flatdata` together with the serialized data. The example is taken from the original [flatdata repository](https://github.com/heremaps/flatdata). It also contains a Rust module which implements the schema. Usually, this code would be generated automatically by `flatdata`'s generator. For now, we use it as a proof of concept to develop readers and writers, and also to manually define a desired layout we would generate automatically otherwise.

The standalone tests read and write all different types of data and check that the data was de/serialized correctly. Run them as follows:

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
