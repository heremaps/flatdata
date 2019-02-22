# flatdata [![Build Status](https://travis-ci.org/heremaps/flatdata.svg?branch=master)](https://travis-ci.org/heremaps/flatdata)

_Write once, read-many, zero-overhead binary structured file format._

Flatdata is a library providing data structures for convenient creation, storage and access of packed memory-mappable structures with minimal overhead. Library consists of schema language, code generator for C++, Python and Go, and target language libraries.

* [Why Flatdata](#why-flatdata)
* [Building Flatdata](#building-flatdata)
* [Using Flatdata](#using-flatdata)
* [Library Layout](#library-layout)
* [License](#license)

## Why `flatdata`?

Flatdata helps creating efficient datasets:

* Zero overhead random access
* Support for bit and byte packing
* Structuring data using a schema definition
* Optimized for large read-only datasets
* Portable with support for multiple languages

Flatdata _doesn't_ provide:

* Backwards compatible schema evolution
* Support for mutable datasets
* Portable floating point serialization

For more details read [why flatdata](docs/src/why-flatdata.rst).

## Using `flatdata`

### Generator

To use the generator, you need Python 3 and the dependencies listed in `requirements.txt`

```shell
pip3 install -r requirements.txt
generator/app.py
```

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


## Library Layout

Library is organized as follows:

   * `generator` includes sources of the flatdata code generator.
       * `generator/app.py` executable script. Use it to generate code in target language.
   * `flatdata-cpp` includes C++ library sources. Client application needs to include and
                      link against this library.
   * `flatdata-py` includes python library sources. Client application needs to have this
                     folder in PYTHON_PATH.
   * `flatdata-go`  includes Go library sources. Client application needs to have `flatdata-go/flatdata`
                     folder in GOPATH.
   * `tools` contains tools to work with flatdata archives.
       * `tools/inspect_flatdata.py` provides interactive python interpreter loaded with a specified
           archive.

At the moment following languages are supported:

   * *C++*. Main development target and first-class citizen. Used extensively, tested excessively,
       normally receives features first. Use `cpp` generator.
   * *Python*. Used mostly for debugging and testing purposes, thus it is always a bit late.
       Use `py` generator.
   * *Dot*. Used to generate diagrams of the schema. Normally, it is up to date.
       Use `dot` generator.
   * *Go*. Beta implementation for reader. No guarantees of backward compatibility at the moment! Use `go` generator.

For more details see the [documentation](docs/src/index.rst).

## License

Copyright (c) 2017 HERE Europe B.V.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
