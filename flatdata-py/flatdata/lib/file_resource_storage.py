'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from __future__ import annotations

import mmap
import os

from .errors import MissingResourceError


class FileResourceStorage:
    """
    Resource storage based on memory-mapped files.
    """

    @staticmethod
    def memory_map(filename: str) -> mmap.mmap:
        """
        Memory maps given file. Introduced to be able to swap mmap implementations.
        :param filename:
        :return: file-like object for memory mapped file.
        """
        opened_file = open(filename, 'r')
        return mmap.mmap(opened_file.fileno(), 0, access=mmap.ACCESS_READ)

    def __init__(self, path: str) -> None:
        self.path: str = path

    def get(self, key: str, is_optional: bool = False) -> mmap.mmap | 'FileResourceStorage' | None:
        filename = os.path.join(self.path, key)
        if not os.path.exists(filename):
            if not is_optional:
                raise MissingResourceError(key)
            else:
                return None

        if os.path.isfile(filename):
            return self.memory_map(filename)

        return FileResourceStorage(filename)

    def ls(self) -> list[str]:
        return os.listdir(self.path)
