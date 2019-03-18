'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import argparse
import logging
import os.path
import sys

# check that requirements are installed here
try:
    # pylint: disable=unused-import
    import pyparsing
    import jinja2
except ModuleNotFoundError as exc:
    print("Cannot import `%s`, you probably need to install it. See `generator/requirements.txt` or `README.md`." % exc.name, file=sys.stderr)
    sys.exit(2)

from flatdata.generator.engine import Engine
from flatdata.generator.tree.errors import FlatdataSyntaxError


def _parse_command_line():
    parser = argparse.ArgumentParser(
        description="Generates code for a given flatdata schema file.")
    parser.add_argument("-s", "--schema", type=str, required=True,
                        help="Path to the flatdata schema file")
    parser.add_argument("-g", "--gen", type=str, required=True,
                        help="Language to generate bindings for. Supported values: %s" %
                        (', '.join(Engine.available_generators())))
    parser.add_argument("-O", "--output-file", type=str, required=True,
                        default=None,
                        help="Destination file. Forces all output to be stored in one file")
    parser.add_argument("-v", "--verbose", action="store_true",
                        help="Enable verbose mode")
    parser.add_argument("--debug", action="store_true",
                        help="Enable debug output")
    return parser.parse_args()


def _setup_logging(args):
    level = logging.WARNING
    if args.debug:
        level = logging.DEBUG
    elif args.verbose:
        level = logging.INFO

    logging.basicConfig(
        format="%(asctime)s - %(levelname)s - %(module)s:%(lineno)d - %(message)s",
        datefmt="%H:%M:%S",
        level=level)


def _check_args(args):
    if not os.path.isfile(args.schema):
        logging.fatal("Cannot find schema file at %s", args.schema)
        sys.exit(1)


def _run(args):
    _setup_logging(args)
    _check_args(args)

    with open(args.schema, 'r') as input_file:
        schema = input_file.read()
        try:
            engine = Engine(schema)
            logging.debug("Tree: %s", engine.tree)
        except FlatdataSyntaxError as ex:
            logging.fatal("Error reading schema: %s ", ex)
            sys.exit(1)

    try:
        logging.info("Generating %s...", args.gen)
        output_content = engine.render(args.gen)
    except ValueError as ex:
        logging.fatal("%s", ex)
        sys.exit(1)

    dirname = os.path.dirname(os.path.abspath(args.output_file))
    if not os.path.exists(dirname):
        os.makedirs(dirname)
    with open(args.output_file, "w") as output:
        output.write(output_content)
        logging.info("Code for %s is written to %s", args.gen, args.output_file)


def main():
    """Entrypoint"""
    _run(_parse_command_line())