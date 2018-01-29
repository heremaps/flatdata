# CI image for building with gcc and clang
FROM alpine:latest

RUN apk --no-cache add clang cmake make binutils musl-dev g++ boost-dev gtest gtest-dev gfortran python3-dev go && \
    pip3 install jinja2 pyparsing pandas nose && \
    go get github.com/golang/lint/golint &&\
    go get github.com/golang/dep/cmd/dep

