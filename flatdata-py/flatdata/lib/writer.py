'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import argparse
import os
import json

from .flatdata_writer import Writer


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-p", "--output-path", type=str, dest="path", required=True,
                        help="Path to archive")
    parser.add_argument("-s", "--schema", type=str, dest="schema", required=True,
                        help="Archive schema file")
    parser.add_argument("-r", "--resource-name", type=str, dest="resource_name", required=True,
                        help="Archive's schema file")
    parser.add_argument("-d", "--json-file", type=str, dest="json_file", required=True,
                        help="File containing data in json format")
    args = parser.parse_args()

    if not os.path.exists(args.schema):
        raise RuntimeError(
            "Specified schema file %s doesn't exists" % args.schema)

    if not os.path.exists(args.json_file):
        raise RuntimeError(
            "Specified json file %s doesn't exists" % args.json_file)

    with open(args.schema, 'r') as schema_file:
        schema = schema_file.read()

    with open(args.json_file, 'r') as json_file:
        data = json.load(json_file)

    archive_writer = Writer(schema, args.path)
    archive_writer.set(args.resource_name, data)
    archive_writer.finish()


if __name__ == "__main__":
    main()
