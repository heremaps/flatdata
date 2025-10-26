'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import glob
import pytest

from flatdata.generator.generators.go import GoGenerator
from .assertions import generate_and_assert_in
from .schemas import schemas_and_expectations


def generate_and_compare(test_case):
    with open(test_case[0], 'r') as test_file:
        test = test_file.read()

    expectations = list()
    for file in glob.glob(test_case[1] + '*'):
        with open(file, 'r') as expectation_file:
            expectations.append(expectation_file.read())

    generate_and_assert_in(test, GoGenerator, *expectations)


def skip(test_case):
    raise pytest.skip("Test %s is skipped" % test_case[0])

def get_test_cases():
    test_cases = []
    for x in schemas_and_expectations(generator='go', extension='go'):
        test_cases.append(x)
    return test_cases

@pytest.mark.parametrize("test_case", get_test_cases())
def test_against_expectations(test_case):
    # Go does not yet support namespaces, enums, ranges, or constants, skip those tests
    if "enums" not in test_case[0] and "constants" not in test_case[0] and "namespaces" not in test_case[0] and "ranges" not in test_case[0]:
        generate_and_compare(test_case)
    else:
        skip(test_case)
