'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from flatdata.lib.errors import ArchivePathNotProvidedError, MissingResourceName


class _Resource():
    '''
    _Resource class.

    This class provides the functionality of in memory storage.
    It uses provided writer object to write stored data to file.
    '''
    def __init__(self, name, writer=None, path="", is_subarchive=False):
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

        self.data = bytearray()
        self._valid = True
        self._resource_writer = None

        if writer:
            self._resource_writer = writer.create_instance()

        if self._resource_writer:
            self._resource_writer.open(name, path)

    def get_status(self):
        '''Returns status of resource. Status is valid if resource is not yet written.'''
        return self._valid

    def write(self, data):
        '''
        Concatenates passed data to instance member bytearray or bytes.

        :param data(bytearray): bytearray to be added to resource
        '''
        if data and isinstance(data, bytearray) or isinstance(data, bytes):
            self.data += data

    def get_data(self):
        '''Returns resources data in bytearray'''
        return self.data

    def add_size(self):
        '''Calculate size of stored data and appends it to the begining'''
        self.data = int(len(self.data)).to_bytes(
            8, byteorder="little", signed=False) + self.data

    def add_padding(self):
        '''Add 8 byte zero padding at the end of data'''
        self.data += b'\x00' * 8

    def __str__(self):
        '''Facilitate print for debugging'''
        return f'{self.data}'

    def close(self):
        '''
        Marks the end of resource. It will invoke actual write to disk and
        mark this resource as already written by setting resource as invalid.
        '''
        if self._resource_writer:
            self._resource_writer.write(self.data)
            self.data = None
            self._resource_writer.close()

        self._valid = False


class ResourceStorage:
    '''
    ResourceStorage class is injected to ArchiveBuilder.
    It is responsible for creating and managing all resources available in archive.
    '''

    def __init__(self, writer, path):
        '''
        Creates ResourceStorage object.

        :param writer(object): writes data to disc
        :param path(str): file path where resource is created
        '''
        self._store = {}
        self._resource_writer = writer
        self._path = path

    def get(self, resource_name, is_subarchive=False):
        '''
        Returns the instance of _Resource.

        :param resource_name(str): name of resource
        :param is_subarchive(bool): identifies if resource is archive or subarchive
        :return _Resource()
        '''
        self._store[resource_name] = _Resource(
            resource_name, self._resource_writer, self._path, is_subarchive)
        return self._store[resource_name]

    def close(self):
        '''Try to close _Resource objects which are not written to disc'''
        for key in self._store:
            if self._store[key].get_status():
                self._store[key].close()
