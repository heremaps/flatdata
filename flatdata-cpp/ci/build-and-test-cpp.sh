#!/bin/sh
set -ex
mkdir build
cd build
cmake ../flatdata-cpp -DCMAKE_CXX_FLAGS="-Wall -pedantic -Wextra" $EXTRA_CMAKE_ARGS
make -j$(nproc)
make test
