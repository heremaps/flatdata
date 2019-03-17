# flatdata-py

Python 3 implementation

## Pre-requisites

Install `flatdata-generator` as it's used directly by this implementation.

```sh
# for now, directly from this repository
pip3 install ../flatdata-generator
```

## Basic usage

Once you have [created a flatdata schema file](../README.md#creating-a-schema), you can generate a Python module:

```sh
flatdata-generator --gen python --schema locations.flatdata --output-file locations.py
```

TODO: Add basic python usages