'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import glob

from flatdata.generator.generators.dot import DotGenerator
from .assertions import generate_and_assert_in
from .schemas import schemas_and_expectations

def test_structures_outside_of_archives_are_not_represented():
    unexpected_lines = [
        "_n_S"
    ]
    generate_and_assert_in("""
namespace n{
struct S {
    f : u64 : 3;
}
}
    """, DotGenerator, *[], unexpected_items=unexpected_lines)

def generate_and_compare(test_case):
    with open(test_case[0], 'r') as test_file:
        test = test_file.read()

    expectations = list()
    for file in  glob.glob(test_case[1] + '*'):
        with open(file, 'r') as expectation_file:
            expectations.append(expectation_file.read())

    generate_and_assert_in(test, DotGenerator, *expectations)

def test_against_expectations():
    for x in schemas_and_expectations(generator='dot', extension='dot'):
        yield generate_and_compare, x