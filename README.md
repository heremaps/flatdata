# flatdata [![Build Status](https://travis-ci.org/heremaps/flatdata.svg?branch=master)](https://travis-ci.org/heremaps/flatdata)

_Write once, read-many, minimal overhead binary structured file format._

Flatdata is a library providing data structures for convenient creation, storage and access of packed memory-mappable structures with minimal overhead.

With `flatdata`, the user defines a schema of the data format using a very simple schema language that supports plain structs, vectors and multivectors. The schema is then used to generate builders and readers for serialization and deserialization of the data to an archive of files on disk.

The data is serialized in a portable way which allows zero-overhead random access to it by using memory mapped storage: the operating system facilities for loading, caching and paging of the data, and most important, accessing it as if it were in memory.

## Why `flatdata`?

Flatdata helps creating efficient datasets:

* Zero overhead random access
* Support for bit and byte packing
* Structuring data using a schema definition
* Optimized for large read-only datasets
* Portable, with support for multiple languages

Flatdata _doesn't_ provide:

* Backwards compatible schema evolution
* Support for mutable datasets
* Portable floating point serialization

For more details read [why flatdata](docs/src/why-flatdata.rst).

## Using `flatdata`

### Creating a schema

Define a flatdata archive, let's say `locations.flatdata`:
```cpp
namespace loc {
    struct Point {
        x : u32 : 32;
        y : u32 : 32;
    }
    archive Locations {
        pois : vector< Point >;
    }
}
```

### Generating a module

Flatdata relies on a generator that takes a `.flatdata` file as an input and
generates a module for one of the supported languages.

The following languages are supported:

  * First-class citizen implementations:
    * **[C++](./flatdata-cpp)** - used extensively, tested excessively normally receives features first
    * **[Rust](./flatdata-rs)** - the newest addition to the family
  * Read-only implementations:
    * **[Python](./flatdata-py)** - used mostly for debugging purposes
    * **[Dot](./flatdata-dot)** - used to generate diagrams of the schema
    * **[Go](./flatdata-go)** - beta implementation.


To use the generator:

```sh
# install all the required dependencies in a virtualenv
python3 -m virtualenv .venv
.venv/bin/activate
pip3 install -r generator/requirements.txt

# generate a rust module
./generator/app.py -s locations.flatdata -g rust -O location
```

## License

Copyright (c) 2017-2019 HERE Europe B.V.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this document by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
