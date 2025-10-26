'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import glob
import pytest

from flatdata.generator.generators.rust import RustGenerator
from .assertions import generate_and_assert_in
from .schemas import schemas_and_expectations

def test_format_numeric_literals():
    assert RustGenerator._format_numeric_literal(1) == "1"
    assert RustGenerator._format_numeric_literal(123) == "123"
    assert RustGenerator._format_numeric_literal(-123) == "-123"
    assert RustGenerator._format_numeric_literal(1) == "1"
    assert RustGenerator._format_numeric_literal(10) == "10"
    assert RustGenerator._format_numeric_literal(100) == "100"
    assert RustGenerator._format_numeric_literal(1000) == "1_000"
    assert RustGenerator._format_numeric_literal(10000) == "10_000"
    assert RustGenerator._format_numeric_literal(100000) == "100_000"
    assert RustGenerator._format_numeric_literal(1000000) == "1_000_000"
    assert RustGenerator._format_numeric_literal(-1000000) == "-1_000_000"
    assert RustGenerator._format_numeric_literal(2147483647) == "2_147_483_647"
    assert RustGenerator._format_numeric_literal("hello") == "hello"
    assert RustGenerator._format_numeric_literal("hello1234") == "hello1234"
    assert RustGenerator._format_numeric_literal("1234hello") == "1234hello"

def generate_and_compare(test_case):
    with open(test_case[0], 'r') as test_file:
        test = test_file.read()

    expectations = list()
    for file in  glob.glob(test_case[1] + '*'):
        with open(file, 'r') as expectation_file:
            expectations.append(expectation_file.read())

    generate_and_assert_in(test, RustGenerator, *expectations)

def get_test_cases():
    test_cases = []
    for x in schemas_and_expectations(generator='rust', extension='rs'):
        test_cases.append(x)
    return test_cases

@pytest.mark.parametrize("test_case", get_test_cases())
def test_against_expectations(test_case):
    generate_and_compare(test_case)
