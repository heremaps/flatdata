#!/bin/sh
set -ex
mkdir build
cd build
cmake ../flatdata-cpp -DCMAKE_CXX_FLAGS="-Wall -pedantic -Wextra"
make -j$(nproc)
make test
