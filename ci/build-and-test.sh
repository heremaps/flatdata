#!/bin/sh
set -ex

cmake ../src \
    -DCMAKE_C_COMPILER=${C_COMPILER} \
    -DCMAKE_CXX_COMPILER=${CXX_COMPILER} \
    -DCMAKE_CXX_FLAGS="-Wall -pedantic -Wextra"
make
make test

cd /src/flatdata-go
make setup-ci
cd $GOPATH/src/github.com/heremaps/flatdata/flatdata-go
make run-ci

cd /src/flatdata-go-backward-compatibility
make setup-ci
cd $GOPATH/src/github.com/heremaps/flatdata/flatdata-go-backward-compatibility
make run-ci
