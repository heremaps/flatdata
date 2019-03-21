# flatdata-py

Python 3 implementation

## Basic usage

Once you have [created a flatdata schema file](../README.md#creating-a-schema), you can generate a Python module:

```sh
flatdata-generator --gen python --schema locations.flatdata --output-file locations.py
```

TODO: Add basic python usages

## Using the inspector

`flatdata-py` comes with a handy tool called the `inspector`, you can install it or run it like so:

```sh
pip3 install .

flatdata-inspector -p /path/to/my/flatdata.archive
```

or from the source code:

```sh
pip3 install -r requirements.txt

python3 -m flatdata.inspector
```

## Local development

If you wish to modify both `flatdata-generator` and `flatdata-py`, you need to run tests or scripts here
while overriding the `PYTHONPATH`:

`export PYTHONPATH=$PWD/../flatdata-generator`
