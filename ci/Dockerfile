# CI image for building with gcc and clang
FROM alpine:latest

ENV GOPATH /go
ENV PATH $PATH:$GOPATH/bin

RUN apk --no-cache add clang cmake make binutils musl-dev g++ boost-dev gtest gtest-dev gfortran python3-dev go git && \
    pip3 install jinja2 pyparsing pandas nose && \
    go get -u github.com/golang/lint/golint && \
    go get -u github.com/golang/dep/cmd/dep && \
    mkdir -p $GOPATH/src/github.com/heremaps/flatdata
