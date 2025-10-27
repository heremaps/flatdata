'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import pytest

from flatdata.generator.generators.flatdata import FlatdataGenerator
from flatdata.generator.tree.builder import build_ast
from .schemas import schemas_and_expectations


def generate_and_compare(test_case):
    with open(test_case[0], 'r') as test_file:
        test = test_file.read()
    with open(test_case[1], 'r') as expectation_file:
        expectation = expectation_file.read()
    tree = build_ast(definition=test)
    contents = FlatdataGenerator().render(tree)
    assert expectation == contents

def get_test_cases():
    test_cases = []
    for x in schemas_and_expectations(generator='flatdata', extension='flatdata'):
        test_cases.append(x)
    return test_cases

@pytest.mark.parametrize("case", get_test_cases())
def test_against_expectations(case):
    generate_and_compare(case)

@pytest.mark.parametrize("case", get_test_cases())
def test_normalization_is_fixed_point(case):
    generate_and_compare((case[1], case[1]))
