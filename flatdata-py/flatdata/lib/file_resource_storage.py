'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import mmap
import os

from .errors import MissingResourceError


class FileResourceStorage:
    """
    Resource storage based on memory-mapped files.
    """

    @staticmethod
    def memory_map(filename, read_only):
        """
        Memory maps given file. Introduced to be able to swap mmap implementations.
        :param filename:
        :return: file-like object for memory mapped file.
        """
        if read_only:
            opened_file = open(filename, 'r')
            return mmap.mmap(opened_file.fileno(), 0, access=mmap.ACCESS_READ)
        else:
            opened_file = open(filename, 'wb')
            return mmap.mmap(opened_file.fileno(), 0, access=mmap.ACCESS_WRITE)

    def __init__(self, path, read_only=True):
        self.path = path
        self.read_only = read_only

    def get(self, key, is_optional=False):
        filename = os.path.join(self.path, key)
        if not os.path.exists(filename):
            if not is_optional:
                raise MissingResourceError(key)
            else:
                return None

        if os.path.isfile(filename):
            return self.memory_map(filename, self.read_only)

        return FileResourceStorage(filename)
