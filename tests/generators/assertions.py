'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from nose.tools import *

from generator.tree.builder import SyntaxTreeBuilder
import re


def unify_whitespace(value):
    removed_trailing = re.sub(r"\s+$", "", value)
    return re.sub(r"\s+", " ", removed_trailing)


def assert_in_ignoring_whitespace(member, container):
    assert_in(unify_whitespace(member), unify_whitespace(container))


def assert_not_in_ignoring_whitespace(member, container):
    assert_not_in(unify_whitespace(member), unify_whitespace(container))


def generate_and_assert_in(definition, generator, *expectations, **kwargs):
    tree = SyntaxTreeBuilder.build(definition=definition)
    contents = generator().render(tree)
    if kwargs.get('debug') == True:
        print(contents)
    for expectation in expectations:
        assert_in_ignoring_whitespace(expectation, contents)

    for unexpected_item in kwargs.get('unexpected_items', []):
        assert_not_in_ignoring_whitespace(unexpected_item, contents)
