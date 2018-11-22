# Dummy change

# flatdata [![Build Status](https://travis-ci.org/heremaps/flatdata.svg?branch=master)](https://travis-ci.org/heremaps/flatdata)

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

Flatdata doesn't provide:

* Backwards compatible schema evolution
* Support for mutable datasets
* Portable floating point serialization

For more details read [why flatdata](docs/src/why-flatdata.rst).

## Building `flatdata`

The C++ part of the library depends on Boost. The schema generator and the Python part of the
library require Python 3.

```shell
pip3 install -r requirements.txt
mkdir build && cd build && cmake ..
make
make test  # optional
```

## Using `flatdata`

*Note: Until the release APIs are expected to be undergoing (potentially breaking) changes. Binary data layout is stable, though.*

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

Generate a C++ header `locations.hpp` from it.
```shell
./generator/app.py --gen cpp --schema locations.flatdata --output-file locations.hpp
```

Serialize some data:
```cpp
// Compile with: c++ -std=c++11 writer.cpp -Iflatdata-cpp/include -Lbuild/flatdata-cpp -lflatdata -lboost_system -lboost_filesystem -o writer
#include "locations.hpp"
int main() {
  auto storage = flatdata::FileResourceStorage::create("locations.archive");  // create storage
  auto builder = loc::LocationsBuilder::open(std::move(storage));             // create builder
  auto pois = builder.start_pois();

  uint32_t x, y;
  while(std::cin >> x >> y) {
      loc::PointMutator poi = pois.grow();
      poi.x = x;
      poi.y = y;
  }
  pois.close();  // flush not yet written data to disk
}
```

And finally, read the serialized data:

```cpp
// Compile with: c++ -std=c++11 reader.cpp -Iflatdata-cpp/include -Lbuild/flatdata-cpp -lflatdata -lboost_system -lboost_filesystem -o reader
#include "locations.hpp"
#include <iostream>
int main() {
    auto storage = flatdata::FileResourceStorage::create("locations.archive");  // open storage
    auto archive = loc::Locations::open(std::move(storage));              // create archive
    for (loc::Point point : archive.pois()) {                             // iterate through pois
        std::cout << point.to_string() << std::endl;
    }
    return 0;
}
```

For more examples cf. the [examples](examples) directory.

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
