'''
 Copyright (c) 2019 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import glob
from nose.tools import eq_

from generator.generators.rust import RustGenerator
from .assertions import generate_and_assert_in
from .schemas import schemas_and_expectations

def test_format_numeric_literals():
    eq_(RustGenerator._format_numeric_literal(1), "1")
    eq_(RustGenerator._format_numeric_literal(123), "123")
    eq_(RustGenerator._format_numeric_literal(-123), "-123")
    eq_(RustGenerator._format_numeric_literal(1), "1")
    eq_(RustGenerator._format_numeric_literal(10), "10")
    eq_(RustGenerator._format_numeric_literal(100), "100")
    eq_(RustGenerator._format_numeric_literal(1000), "1_000")
    eq_(RustGenerator._format_numeric_literal(10000), "10_000")
    eq_(RustGenerator._format_numeric_literal(100000), "100_000")
    eq_(RustGenerator._format_numeric_literal(1000000), "1_000_000")
    eq_(RustGenerator._format_numeric_literal(-1000000), "-1_000_000")
    eq_(RustGenerator._format_numeric_literal(2147483647), "2_147_483_647")
    eq_(RustGenerator._format_numeric_literal("hello"), "hello")
    eq_(RustGenerator._format_numeric_literal("hello1234"), "hello1234")
    eq_(RustGenerator._format_numeric_literal("1234hello"), "1234hello")

def generate_and_compare(test_case):
    with open(test_case[0], 'r') as test_file:
        test = test_file.read()

    expectations = list()
    for file in  glob.glob(test_case[1] + '*'):
        with open(file, 'r') as expectation_file:
            expectations.append(expectation_file.read())

    generate_and_assert_in(test, RustGenerator, *expectations)

def test_against_expectations():
    for x in schemas_and_expectations(generator='rust', extension='rs'):
        generate_and_compare(x)
