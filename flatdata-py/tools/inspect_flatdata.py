#!/usr/bin/env python3
'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import argparse
import inspect
import os.path

import pandas as pd
import sys

parent_path = os.path.join(
    os.path.dirname(os.path.abspath(inspect.getfile(inspect.currentframe()))), os.pardir)
sys.path.insert(0, parent_path)
sys.path.insert(0, os.path.join(parent_path, 'flatdata-py'))

from inline import open_archive

DESCRIPTION = \
    """Flatdata Interactive Shell.

    Archive at {path}
    Data is available via `archive`. Try following:
     - `archive`
     - `archive.resource`
     - `archive.resource[N]`
     - `archive.resource[N:M:S]`
     - `archive.resource[N:M:S].field`
    """


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
