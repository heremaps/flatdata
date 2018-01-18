'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import sys

sys.path.insert(0, "..")
from nose.tools import *
import generator.tree.nodes.references as refs


def test_reference_name_is_at_prefixed_and_at_separated():
    assert_equal("@foo@bar@baz", refs.TypeReference(name="foo.bar.baz").name)
