'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import argparse
import fnmatch
import os
import sys

import pandas as pd

from .file_resource_storage import FileResourceStorage
from flatdata.generator.engine import Engine
from flatdata.generator.tree.errors import FlatdataSyntaxError


DESCRIPTION = \
    """Flatdata Interactive Shell.

    Archive at {path}
    Data is available via `archive`. Try the following:
     - `archive`
     - `archive.resource`
     - `archive.resource[N]`
     - `archive.resource[N:M:S]`
     - `archive.resource[N:M:S].field`
    """


def open_archive(path, archive=None, module_name=None):
    """
    Opens archive at a given path.
    Archive schema is read and python bindings are generated on the fly.

    :param path: Path to archive
    :param archive: Archive name to open (in case multiple archives reside in one directory)
                    if None, will be implied. If cannot be implied, RuntimeError is raised.
    :param module_name: Module name to create. If None, will match the highest-level namespace.
    :return: tuple archive, module
    """
    if not os.path.exists(path):
        raise RuntimeError("Specified non-existent path %s" % path)

    archive_path = path if os.path.isdir(path) else os.path.dirname(path)
    signatures = [p for p in os.listdir(
        archive_path) if fnmatch.fnmatch(p, "*.archive")]

    if not signatures:
        raise RuntimeError("No archives located at path %s" % path)

    if len(signatures) > 1 and archive is None:
        raise RuntimeError(
            "Multiple archives found at given path %s\nPlease specify archive name. Found: %s" %
            (path, signatures))

    matching = 0
    if archive is not None:
        try:
            matching = signatures.index(archive + ".archive")
        except ValueError:
            raise RuntimeError("Specified archive not found at path.")

    archive_name, _ = signatures[matching].rsplit('.', 1)
    schema_filename = os.path.join(
        archive_path, signatures[matching] + ".schema")

    with open(schema_filename) as input_file:
        try:
            module, archive_type = \
                Engine(input_file.read()).render_python_module(module_name=module_name,
                                                               archive_name=archive_name)
        except FlatdataSyntaxError as err:
            raise RuntimeError("Error reading schema: %s " % err)
    archive = archive_type(FileResourceStorage(archive_path))
    return archive, module


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("-p", "--path", type=str, dest="path", required=True,
                        help="Path to archive")
    parser.add_argument("-a", "--archive", type=str, dest="archive", required=False, default=None,
                        help="Name of the archive")
    parser.add_argument("--non-interactive", type=str, dest="non_interactive", required=False,
                        default=None,
                        help="Python code to execute in non-interactive mode")
    args = parser.parse_args()

    archive, _ = open_archive(args.path, args.archive)

    pd.set_option('display.max_rows', 30)
    pd.set_option('expand_frame_repr', False)

    if args.non_interactive:
        # pylint: disable=exec-used
        exec(args.non_interactive, globals(), locals())
        sys.exit(0)

    import IPython
    IPython.embed(
        locals_ns={"archive": archive},
        global_ns={"archive": archive},
        banner1=DESCRIPTION.format(path=args.path),
        display_banner=True)


if __name__ == "__main__":
    main()
