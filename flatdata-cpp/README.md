# flatdata-cpp

C++ 11 implementation of `flatdata`

## Building

First, install the requirements of the generator and build `flatdata-cpp`.

```shell
pip3 install -r flatdata-generator/requirements.txt
cd flatdata-cpp
mkdir build && cd build
cmake ..
make
make test # optional
```

## Usage & basic example

Once you have [created a flatdata schema file](../README.md#creating-a-schema), you can generate a C++ header:

```shell
./generator --gen cpp --schema locations.flatdata --output-file locations.hpp
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