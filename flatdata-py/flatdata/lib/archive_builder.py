'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from collections import namedtuple
import os

from .errors import IndexWriterError, MissingFieldError, UnknownFieldError, \
    UnknownStructureError, UnknownResourceError, ResourceAlreadySetError

from .resources import Instance, Vector, Multivector, RawData
from .data_access import write_value

_SCHEMA_EXT = ".schema"

ResourceSignature = namedtuple("ResourceSignature",
                               ["container", "initializer", "schema", "is_optional", "doc"])


class IndexWriter:
    """
    IndexWriter class. Only applicable when multivector is present in archive schema.
    """

    def __init__(self, name, size, resource_storage):
        """
        Create IndexWriter class.

        All arguments are required.
        """
        if not (name and resource_storage and size):
            raise IndexWriterError(
                f"Either ResourceStorage: {resource_storage} or name: {name} or size:"
                "{size} not provided.")

        self._name = name
        self._index_size = size
        self._fout = resource_storage.get(f'{self._name}_index', False)

    def add(self, index):
        """
        Convert index(number) to bytearray and add to in memory store
        """
        index_bytes = int(index).to_bytes(self._index_size,
                                          byteorder="little", signed=False)
        self._fout.write(index_bytes)

    def finish(self):
        """
        Complete index resource by adding size and padding followed by writing to file
        """
        self._fout.add_size()
        self._fout.add_padding()
        self._fout.close()


class ArchiveBuilder:
    """
    ArchiveBuilder class. Entry point to writing Flatdata.
    Provides methods to create flatdata archives.
    """

    def __init__(self, resource_storage, path=""):
        """
        Opens archive from a given resource writer.
        :param resource_storage: storage manager to store and write to disc
        :param path: file path where archive is created
        """
        self._path = os.path.join(path, self._NAME)
        self._resource_storage = resource_storage
        self._write_archive_signature()
        self._write_archive_schema()
        self._resources_written = [f"{self._NAME}.archive"]

    @classmethod
    def name(cls):
        '''Returns archive name'''
        return cls._NAME

    @classmethod
    def schema(cls):
        '''Returns archive schema'''
        return cls._SCHEMA

    def _write_raw_data(self, name, data):
        '''
        Helper function to write data

        :param name(str): resource name
        :param data(bytearray): data to be written to disc
        '''
        storage = self._resource_storage.get(name)
        storage.write(data)
        storage.close()

    def _write_schema(self, name):
        '''
        Writes resource schema

        :param name: name of resource
        '''
        self._write_raw_data(f"{name}.schema", bytes(
            self._RESOURCES[name].schema, 'utf-8'))

    def _write_archive_signature(self):
        '''Writes archive's signature'''
        self._write_raw_data(f"{self._NAME}.archive", b'\x00' * 16)

    def _write_archive_schema(self):
        '''Writes archive schema'''
        self._write_raw_data(
            f"{self._NAME}.archive.schema", bytes(self._SCHEMA, 'utf-8'))

    def _write_index_schema(self, resource_name, schema):
        self._write_raw_data(
            f"{resource_name}_index.schema", bytes(schema, 'utf-8'))

    def subarchive(self, name):
        """
        Returns an archive builder for the sub-archive `name`.
        :raises $name_not_subarchive_error
        :param name: name of the sub-archive
        """
        NotImplemented

    @classmethod
    def __validate_structure_fields(cls, name, struct, initializer):
        '''
        Validates whether passed object has all required fields

        :raises MissingFieldError
        :raises UnknownFieldError
        :param name(str): name of object(struct)
        :param struct(object): object to validate
        :param initializer(object): provided field keys to validate from
        '''
        for key in initializer._FIELD_KEYS:
            if key not in struct:
                raise MissingFieldError(key, name)
        for key in struct.keys():
            if key not in initializer._FIELD_KEYS:
                raise UnknownFieldError(key, name)

    def __set_instance(self, storage, name, value):
        '''
        Creates and writes instance type resource

        :param storage(object): handles storage and writing to disc
        :param name(str): instance name
        :param value(dict): instance object replicates struct
        '''
        initializer = self._RESOURCES[name].initializer
        ArchiveBuilder.__validate_structure_fields(name, value, initializer)

        bout = bytearray(initializer._SIZE_IN_BYTES)
        for (key, field) in initializer._FIELDS.items():
            write_value(bout, field.offset, field.width,
                        field.is_signed, value[key])

        storage.write(bout)

    def __set_vector(self, storage, name, vector):
        '''
        Creates and writes vector resource

        :param storage(object): handles storage and writing to disc
        :param name(str): resource name
        :param vector(list): vector, provided as list of dict ie [{},{}]
        '''
        initializer = self._RESOURCES[name].initializer
        for value in vector:
            ArchiveBuilder.__validate_structure_fields(
                name, value, initializer)
        for value in vector:
            bout = bytearray(initializer._SIZE_IN_BYTES)
            for (key, field) in initializer._FIELDS.items():
                write_value(bout, field.offset, field.width,
                            field.is_signed, value[key])
            storage.write(bout)

    def __set_multivector(self, storage, name, value):
        '''
        Creates and writes multivector resource

        :param storage(object): handles storage and writing to disc
        :param name(str): resource name
        :param value(list): mulitvector, provided as list of list of dict ie [[{},{}],[]]
        '''
        initializer_list = self._RESOURCES[name].initializer

        initializers = {}
        for index, obj_type in enumerate(initializer_list[1:]):
            initializers[obj_type._NAME] = (index, obj_type)

        def valid_structure_name(_obj):
            return _obj['name'] in [_initializer._NAME for _initializer in initializer_list[1:]]

        def validate_fields(_obj):
            matched_obj_list = [
                _initializer for _initializer in initializer_list[1:] \
                    if _initializer._NAME == _obj['name']]
            if len(matched_obj_list) == 1:
                ArchiveBuilder.__validate_structure_fields(
                    name, _obj['attributes'], matched_obj_list[0])

        for sub_list in value:
            for obj in sub_list:
                if not valid_structure_name(obj):
                    raise UnknownStructureError(obj['name'])
                validate_fields(obj)

        index_data_points = []
        data_point = 0
        data_size = 0

        for sub_list in value:
            index_data_points.append(data_point)
            if sub_list:
                for obj in sub_list:
                    # find out correct initializer
                    type_index, matched_initializer = initializers[obj['name']]

                    size = matched_initializer._SIZE_IN_BYTES + 1
                    data_size += size
                    data_point += size
                    bout = bytearray(size)
                    bout[0] = int(type_index).to_bytes(
                        1, byteorder="little", signed=False)[0]

                    for (key, field) in matched_initializer._FIELDS.items():
                        write_value(bout, field.offset + 1 * 8, field.width,
                                    field.is_signed, obj['attributes'][key])

                    storage.write(bout)

        index_data_points.append(data_point)

        index_writer = IndexWriter(
            name, initializer_list[0]._SIZE_IN_BYTES, self._resource_storage)

        for index in index_data_points:
            index_writer.add(index)
        index_writer.finish()
        # Write index schema
        self._write_index_schema(
            name, f'index({self._RESOURCES[name].schema})')
        self._resources_written.append(name)
        self._resources_written.append(f'{name}_index')

    def set(self, name, value):
        """
        Write a resource for this archive at once.
        Can only be done once. `set` and `start` can't be used for the same resource.
        :raises $already_set_error
        :raises $unknown_resource_error
        :param name: name of the resource
        :param value: value to write
        """
        if name not in self._RESOURCES:
            raise UnknownResourceError(name)

        if not self._resources_written.count(name):
            self._write_schema(name)
        else:
            raise ResourceAlreadySetError()

        storage = self._resource_storage.get(name, False)

        if self._RESOURCES[name].container is Instance:
            self.__set_instance(storage, name, value)
        elif self._RESOURCES[name].container is Vector:
            self.__set_vector(storage, name, value)
        elif self._RESOURCES[name].container is Multivector:
            self.__set_multivector(storage, name, value)
        elif self._RESOURCES[name].container is RawData:
            storage.write(value)
        else:
            NotImplementedError

        storage.add_size()
        storage.add_padding()
        storage.close()

        self._resources_written.append(name)

    def finish(self):
        """
        Closes the storage manager
        """
        self._resource_storage.close()
