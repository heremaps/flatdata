#!/usr/bin/env python3

'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import argparse
import inspect
import logging
import os.path
import sys

sys.path.insert(0, os.path.join(
    os.path.dirname(os.path.abspath(inspect.getfile(inspect.currentframe()))), os.pardir))

from generator.engine import Engine
from generator.tree.errors import FlatdataSyntaxError


def _parse_command_line():
    parser = argparse.ArgumentParser(description="Generates code for a given flatdata schema file.")
    parser.add_argument("-s", "--schema", type=str, dest="schema", required=True,
                        help="Path to the flatdata schema file")
    parser.add_argument("-g", "--gen", type=str, nargs="+", dest="gen", required=True,
                        help="Language to generate bindings for. Supported values: %s" % (', '.join(Engine.available_generators())))
    parser.add_argument("-O", "--output-file", type=str, dest="output_file", required=True,
                        default=None,
                        help="Destination file. Forces all output to be stored in one file")
    parser.add_argument("-v", "--verbose", dest="verbose", action="store_true",
                        help="Enable verbose mode")
    parser.add_argument("--debug", dest="debug", action="store_true",
                        help="Enable debug output")
    return parser.parse_args()


class App(object):
    @staticmethod
    def _setup_logging(args):
        level = logging.WARNING
        if args.debug:
            level = logging.DEBUG
        elif args.verbose:
            level = logging.INFO

        logging.basicConfig(format="%(asctime)s - %(levelname)s - %(module)s:%(lineno)d - %(message)s",
                            datefmt="%H:%M:%S",
                            level=level)

    @staticmethod
    def _check_args(args):
        if not os.path.isfile(args.schema):
            logging.fatal("Cannot find schema file at %s" % args.schema)
            sys.exit(1)

    @classmethod
    def run(cls, args):
        cls._setup_logging(args)
        cls._check_args(args)

        with open(args.schema, 'r') as input_file:
            schema = input_file.read()
            try:
                engine = Engine(schema)
                logging.debug("Tree: %s" % engine.tree)
            except FlatdataSyntaxError as e:
                logging.fatal("Error reading schema: %s " % e)
                sys.exit(1)

        for gen in args.gen:
            try:
                logging.info("Generating %s..." % gen)
                output_content = engine.render(gen)
            except Engine.GeneratorNotDefined:
                logging.fatal("Generator %s not implemented. Available options: %s" %
                              (gen, ', '.join(Engine.available_generators())))
                sys.exit(1)

        dirname = os.path.dirname(os.path.abspath(args.output_file))
        if not os.path.exists(dirname):
            os.makedirs(dirname)
        with open(args.output_file, "w") as output:
            output.write(output_content)
            logging.info("Code for %s is written to %s" % (gen, args.output_file))


if __name__ == "__main__":
    App.run(_parse_command_line())
