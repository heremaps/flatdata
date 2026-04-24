'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import os
from typing import IO

from flatdata.lib.errors import ArchivePathNotProvidedError, FileNameNotProvided

class FileResourceWriter:
    '''
    This is a factory class which will create instance of FileResourceWriter for
    resource. This class directly writes to disc on a file.
    '''
    def __init__(self) -> None:
        '''Create instance of FileResourceWriter'''
        self._file: IO[bytes] | None = None

    @staticmethod
    def create_instance() -> 'FileResourceWriter':
        '''Static method to create instances and gives this class a factory like behaviour'''
        return FileResourceWriter()

    def open(self, name: str, file_path: str) -> None:
        '''
        Opens a file for writing. It will also create directory if it is not present.

        :raises FileNameNotProvided
        :raises ArchivePathNotProvidedError
        :param name(str): name of file
        :param file_path(str): file path
        '''
        if not name:
            raise FileNameNotProvided()

        if not file_path:
            raise ArchivePathNotProvidedError()

        file_name = os.path.join(file_path, name)
        if not os.path.exists(file_path):
            os.mkdir(file_path)

        self._file = open(file_name, 'wb')

    def write(self, data: bytes | bytearray) -> None:
        '''Write data to file'''
        if data:
            assert self._file is not None, "write() called before open()"
            self._file.write(data)

    def close(self) -> None:
        '''Flush data and close file'''
        assert self._file is not None, "close() called before open()"
        self._file.flush()
        self._file.close()
