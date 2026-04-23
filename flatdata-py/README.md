# flatdata-py

[![Build Status](https://api.travis-ci.com/heremaps/flatdata.svg?branch=master)](https://travis-ci.com/heremaps/flatdata/)

Python 3 implementation of [flatdata](https://github.com/heremaps/flatdata).

## Running the tests

```sh
python3 -m pytest
```

## Basic usage

Once you have [created a flatdata schema file](../README.md#creating-a-schema), you can generate a Python module to read your existing `flatdata` archive:

```sh
flatdata-generator --gen py --schema locations.flatdata --output-file locations.py
```

## Performance tips

`flatdata-py` supports two data access patterns with very different performance characteristics on large archives.

Iterating over a vector yields one Python object per element. Each field access unpacks bits from the underlying memory-mapped data. This is fine for accessing individual elements or small ranges, but has significant per-element overhead for bulk operations:

```python
count = sum(1 for x in archive.links if x.speed_limit > 100)
```

For bulk operations, use the vectorized access methods that read fields directly into NumPy arrays:

```python
# single column access, returns a pandas DataFrame
df = archive.links.speed_limit
count = len(df[df['speed_limit'] > 100])

# full NumPy structured array with all fields
arr = archive.links.to_numpy()
count = int(np.sum(arr['speed_limit'] > 100))

# slices work too
arr = archive.links[1000:2000].to_numpy()
df = archive.links[::10].to_data_frame()
```

* Use `vector.field_name` (column access) when you only need one or a few fields.
* Use `vector.to_numpy()` or `vector.to_data_frame()` when you need all fields at once.
* Use `vector[i].field` for random access to individual elements.
* The underlying data is memory-mapped; the OS pages it from disk on demand. Vectorized results are materialized as NumPy arrays in RAM.

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
