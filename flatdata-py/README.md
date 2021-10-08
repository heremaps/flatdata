# flatdata-py

[![Build Status](https://api.travis-ci.com/heremaps/flatdata.svg?branch=master)](https://travis-ci.com/heremaps/flatdata/)

Python 3 implementation of [flatdata](https://github.com/heremaps/flatdata).

## Running the tests

```sh
python3 -m nose
```

## Basic usage

Once you have [created a flatdata schema file](../README.md#creating-a-schema), you can generate a Python module to read your existing `flatdata` archive:

```sh
flatdata-generator --gen py --schema locations.flatdata --output-file locations.py
```

## Using the inspector

`flatdata-py` comes with a handy tool called the `flatdata-inspector` to inspect the contents of an archive:

* from the `flatdata-py` source directory:

```sh
./inspector.py
# or
python3 -m flatdata.lib.inspector
```

* if you want to install `flatdata-py`:

```sh
pip3 install flatdata-py[inspector]  # the inspector feature requires IPython
flatdata-inspector -p /path/to/my/flatdata.archive
```

## Using the writer

`flatdata-writer` is an addition to `flatdata-py` that can create flatdata archives from a flatdata schema, with the following limitations:
* does not allow adding additional sub-archives to an existing archive
* supports only bulk-writing (no streaming)
* not optimized for performance

* from the `flatdata-py` source directory

```sh
./writer.py --schema archive.flatdata --output-dir testdir --json-file data.json --resource-name resourcename
#or
python3 -m flatdata.lib.writer --schema archive.flatdata --output-dir testdir --json-file data.json --resource-name resourcename
```

Note that the `flatdata-writer` CLI tool can only write one resource at a time. For archives that have multiple non-optional
resources, the tool has to be executed separately for each resource. Only after all resources have been written can the archive be opened.

* if you want to install flatdata-py:

```sh
pip3 install flatdata-py[writer]
flatdata-writer --schema archive.flatdata --output-dir testdir --json-file data.json --resource-name resourcename
```
