'''
 Copyright (c) 2018 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.generators.FlatdataGenerator import FlatdataGenerator
from generator.tree.builder import SyntaxTreeBuilder

from nose.tools import assert_equal

from .schemas import *

def generate_and_compare(test_case):
    with open(test_case[0], 'r') as test_file:
        test = test_file.read()
    with open(test_case[1], 'r') as expectation_file:
        expectation = expectation_file.read()
    tree = SyntaxTreeBuilder.build(definition=test)
    contents = FlatdataGenerator().render(tree)
    assert_equal.__self__.maxDiff = None
    assert_equal(expectation, contents, test_case);

def test_against_expectations():
    for x in schemas_and_expectations(generator='flatdata', extension='flatdata'):
        generate_and_compare(x)

def test_normalization_is_fixed_point():
    for x in schemas_and_expectations(generator='flatdata', extension='flatdata'):
        generate_and_compare((x[1], x[1]))