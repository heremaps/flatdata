'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import glob

from generator.generators.python import PythonGenerator
from .assertions import generate_and_assert_in
from .schemas import schemas_and_expectations

from nose.plugins.skip import SkipTest

def generate_and_compare(test_case):
    with open(test_case[0], 'r') as test_file:
        test = test_file.read()

    expectations = list()
    for file in  glob.glob(test_case[1] + '*'):
        with open(file, 'r') as expectation_file:
            expectations.append(expectation_file.read())

    generate_and_assert_in(test, PythonGenerator, *expectations)

def skip(test_case):
    raise SkipTest("Test %s is skipped" % test_case[0])

def test_against_expectations():
    for x in schemas_and_expectations(generator='py', extension='py'):
        # Python does not yet support enums or constants, skip those tests
        if "enums" not in x[0] and "constants" not in x[0]:
            yield generate_and_compare, x
        else:
            yield skip, x