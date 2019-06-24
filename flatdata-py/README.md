# flatdata-py

[![Build Status](https://api.travis-ci.com/heremaps/flatdata.svg?branch=master)](https://travis-ci.com/heremaps/flatdata/)

**Read-only** Python 3 implementation of [flatdata](https://github.com/heremaps/flatdata).

## Basic usage


Once you have [created a flatdata schema file](../README.md#creating-a-schema), you can generate a Python module to read your existing `flatdata` archive:

```sh
flatdata-generator --gen python --schema locations.flatdata --output-file locations.py
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

