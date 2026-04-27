'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from __future__ import annotations

from typing import Protocol

from flatdata.lib.errors import ArchivePathNotProvidedError, MissingResourceName


class ResourceWriter(Protocol):
    def open(self, name: str, file_path: str) -> None: ...
    def write(self, data: bytes | bytearray) -> None: ...
    def close(self) -> None: ...


class ResourceWriterFactory(Protocol):
    def create_instance(self) -> ResourceWriter: ...


class _Resource():
    '''
    _Resource class.

    This class provides the functionality of in memory storage.
    It uses provided writer object to write stored data to file.
    '''
    def __init__(self, name: str, writer: ResourceWriterFactory | None = None, path: str = "", is_subarchive: bool = False) -> None:
        '''
        Creates in memory storage for resource.

        :raises MissingResourceName
        :raises ArchivePathNotProvidedError
        :param name(str): name of resource
        :param writer(object): object of final writer class
        :param path(str): file path where resource is created
        :param is_subarchive(bool): identifies if resource is archive or subarchive
        '''
        if name:
            self.name = name
        else:
            raise MissingResourceName()

        if not path:
            raise ArchivePathNotProvidedError()

        self.data: bytearray | bytes | None = bytearray()
        self._valid: bool = True
        self._resource_writer: ResourceWriter | None = None

        if writer:
            self._resource_writer = writer.create_instance()

        if self._resource_writer:
            self._resource_writer.open(name, path)

    def get_status(self) -> bool:
        '''Returns status of resource. Status is valid if resource is not yet written.'''
        return self._valid

    def write(self, data: bytes | bytearray) -> None:
        '''
        Concatenates passed data to instance member bytearray or bytes.

        :param data(bytearray): bytearray to be added to resource
        '''
        assert self.data is not None, "write() called on closed resource"
        if data and isinstance(data, bytearray) or isinstance(data, bytes):
            self.data += data

    def get_data(self) -> bytearray | bytes | None:
        '''Returns resources data in bytearray, or None if the resource is closed.'''
        return self.data

    def add_size(self) -> None:
        '''Calculate size of stored data and appends it to the begining'''
        assert self.data is not None, "add_size() called on closed resource"
        self.data = int(len(self.data)).to_bytes(
            8, byteorder="little", signed=False) + self.data

    def add_padding(self) -> None:
        '''Add 8 byte zero padding at the end of data'''
        assert self.data is not None, "add_padding() called on closed resource"
        self.data += b'\x00' * 8

    def __str__(self) -> str:
        '''Facilitate print for debugging.

        Uses !r (repr) instead of implicit __format__ because format(bytes, '')
        is deprecated in Python 3.12+ and raises TypeError in 3.14+.
        '''
        return f'{self.data!r}'

    def close(self) -> None:
        '''
        Marks the end of resource. It will invoke actual write to disk and
        mark this resource as already written by setting resource as invalid.
        '''
        if self._resource_writer:
            assert self.data is not None, "close() called on already-closed resource"
            self._resource_writer.write(self.data)
            self.data = None
            self._resource_writer.close()

        self._valid = False


class ResourceStorage:
    '''
    ResourceStorage class is injected to ArchiveBuilder.
    It is responsible for creating and managing all resources available in archive.
    '''

    def __init__(self, writer: ResourceWriterFactory, path: str) -> None:
        '''
        Creates ResourceStorage object.

        :param writer(object): writes data to disc
        :param path(str): file path where resource is created
        '''
        self._store: dict[str, _Resource] = {}
        self._resource_writer: ResourceWriterFactory = writer
        self._path: str = path

    def get(self, resource_name: str, is_subarchive: bool = False) -> _Resource:
        '''
        Returns the instance of _Resource.

        :param resource_name(str): name of resource
        :param is_subarchive(bool): identifies if resource is archive or subarchive
        :return _Resource()
        '''
        self._store[resource_name] = _Resource(
            resource_name, self._resource_writer, self._path, is_subarchive)
        return self._store[resource_name]

    def close(self) -> None:
        '''Try to close _Resource objects which are not written to disc'''
        for key in self._store:
            if self._store[key].get_status():
                self._store[key].close()
