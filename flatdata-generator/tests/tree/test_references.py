'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import sys
import pytest

sys.path.insert(0, "..")
import flatdata.generator.tree.nodes.references as refs


def test_reference_name_is_at_prefixed_and_at_separated():
    assert "@foo@bar@baz" == refs.TypeReference(name="foo.bar.baz").name
