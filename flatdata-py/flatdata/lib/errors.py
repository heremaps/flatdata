"""
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
"""

import difflib


class SchemaMismatchError(RuntimeError):
    """
    Schema mismatch: archive does not match software expectations.
    """

    def __init__(self, name, expected_schema, actual_schema):
        diff = '\n'.join([l for l in difflib.unified_diff(expected_schema, actual_schema)])
        message = "Schema mismatch for resource {name}. Expected: \n{expected}\n\nActual:{actual}\n\nDiff:{diff}"
        RuntimeError.__init__(self,
                              message.format(name=name,
                                             expected=expected_schema,
                                             actual=actual_schema, diff=diff))


class CorruptArchiveError(RuntimeError):
    """
    Corrupt archive error: missing resource files or schemas, incorrect archive signature etc.
    """


class CorruptResourceError(CorruptArchiveError):
    """
    Resource is corrupt and cannot be opened.
    """


class MissingResourceError(KeyError, CorruptArchiveError):
    """
    Resource or schema is missing.
    """
    def __init__(self, key):
        super().__init__("Resource {key} not found".format(key=key))
