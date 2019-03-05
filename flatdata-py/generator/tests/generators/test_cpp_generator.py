'''
 Copyright (c) 2019 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.generators.cpp import CppGenerator
import glob;

from .assertions import *
from .schemas import *

def generate_and_compare(test_case):
    with open(test_case[0], 'r') as test_file:
        test = test_file.read()

    expectations = list()
    for file in  glob.glob(test_case[1] + '*'):
        with open(file, 'r') as expectation_file:
            expectations.append(expectation_file.read())

    generate_and_assert_in(test, CppGenerator, *expectations)

def test_against_expectations():
    for x in schemas_and_expectations(generator='cpp', extension='h'):
        generate_and_compare(x)
