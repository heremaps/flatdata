'''
 Copyright (c) 2018 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
from nose.tools import assert_equal

from generator.generators.flatdata import FlatdataGenerator
from generator.tree.builder import build_ast
from .schemas import schemas_and_expectations

def generate_and_compare(test_case):
    with open(test_case[0], 'r') as test_file:
        test = test_file.read()
    with open(test_case[1], 'r') as expectation_file:
        expectation = expectation_file.read()
    tree = build_ast(definition=test)
    contents = FlatdataGenerator().render(tree)
    assert_equal.__self__.maxDiff = None
    assert_equal(expectation, contents, test_case)

def test_against_expectations():
    for i in schemas_and_expectations(generator='flatdata', extension='flatdata'):
        yield generate_and_compare, i

def test_normalization_is_fixed_point():
    for i in schemas_and_expectations(generator='flatdata', extension='flatdata'):
        yield generate_and_compare, (i[1], i[1])
