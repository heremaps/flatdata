Quick Start Guide
=================

Building ``flatdata``
---------------------

The C++ part of the library depends on Boost. The schema generator and
the Python part of the library require Python 3.

.. code:: shell

    pip3 install -r requirements.txt
    mkdir build && cd build && cmake ..
    make
    make test  # optional

Using ``flatdata``
------------------

Define a flatdata archive, let’s say ``locations.flatdata``:

.. code:: cpp

    namespace loc {
        struct Point {
            x : u32 : 32;
            y : u32 : 32;
        }
        archive Locations {
            pois : vector< Point >;
        }
    }

Generate a C++ header ``locations.hpp`` from it.

.. code:: shell

    ./generator/app.py --gen cpp --schema locations.flatdata --output-file locations.hpp

Serialize some data:

.. code:: cpp

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

And finally, read the serialized data:

.. code:: cpp

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
