'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from collections import namedtuple
import os

from .errors import *
from .resources import *
from .data_access import write_value

_SCHEMA_EXT = ".schema"

ResourceSignature = namedtuple("ResourceSignature",
                               ["container", "initializer", "schema", "is_optional", "doc"])

class FileResourceWriter:
    def __init__(self, path):
        self.path = path

        if os.path.exists(path):
            raise DirExistsError(path)

        os.mkdir(path)

    def get(self, key, is_subarchive):
        filename = os.path.join(self.path, key)
        if os.path.exists(filename):
            raise FileExistsError(key)
        
        if is_subarchive:
            return FileResourceWriter(filename)
        
        return open(filename, 'wb')

class IndexWriter:
    def __init__(self, name, resource_writer, size):
        self._resource_writer = resource_writer
        self._name = name
        self._index_size = size
        self._size = 0

        self._fout = self._resource_writer.get(f'{self._name}_index', False)
        self._fout.write(b"\x00" * 8)

    def add(self, index):
        index_bytes = int(index).to_bytes(self._index_size, byteorder="little", signed=False)
        self._fout.write(index_bytes)
        self._size += (1 * self._index_size)

    def finish(self):
        # hack to write size
        #self._fout.close()
        #self._fout = self._resource_writer.get(f"{self._name}.index", False)

        self._fout.write(b"\x00" * 8)
        self._fout.close()


class ArchiveBuilder:
    """
    Archive class. Entry point to Flatdata.
    Provides access to flatdata resources and verifies archive/resource schemas on opening.
    """

    def __init__(self, resource_writer):
        """
        Opens archive from a given resource writer.
        :param resource_writer: Resource writer to use.
        """
        self._resource_writer = resource_writer
        self._write_archive_signature()
        self._write_archive_schema()
        self._resources_written = [f"{self._NAME}.archive"]

    @classmethod
    def name(cls):
        return cls._NAME

    @classmethod
    def schema(cls):
        return cls._SCHEMA

    def _write_schema(self, name):
        fout = self._resource_writer.get(f"{name}.schema", False)
        fout.write(self._RESOURCES[name].schema)
        fout.close()
    
    def _write_archive_signature(self):
        fout = self._resource_writer.get(f"{self._NAME}.archive", False)
        # archive signature is 16 zero-ed bytes
        fout.write(bytearray(16))
        fout.close()
        
    def _write_archive_schema(self):
        fout = self._resource_writer.get(f"{self._NAME}.archive.schema", False)
        fout.write(self._SCHEMA)
        fout.close()

    def subarchive(self, name):
        """
        Returns an archive builder for the sub-archive `name`.
        :raises $name_not_subarchive_error
        :param name: name of the sub-archive
        """
        NotImplemented
    @classmethod
    def validate_structure_fields(cls, name, struct, initializer):
        for key in initializer._FIELD_KEYS:
            if key not in struct:
                raise FieldMissingError(key, name)
        for key in struct.keys():
            if key not in initializer._FIELD_KEYS:
                raise UnknownFieldError(key, name)

    def set(self, name, value):
        """
        Write a resource for this archive at once.
        Can only be done once. `set` and `start` can't be used for the same resource.
        :raises $already_set_error
        :raises $already_start_error
        :raises $unknown_resource_error
        :param name: name of the resource
        :param value: value to write
        """
        self._write_schema(name)

        fout = self._resource_writer.get(name, False)
        if self._RESOURCES[name].container is Instance:
            initializer = self._RESOURCES[name].initializer
            ArchiveBuilder.validate_structure_fields(name, value, initializer)
            
            fout.write(int(initializer._SIZE_IN_BYTES).to_bytes(8, byteorder="little"))
            bout = bytearray(initializer._SIZE_IN_BYTES) 
            for (key, field) in initializer._FIELDS.items():
                write_value(bout, field.offset, field.width, field.is_signed, value[key])

            fout.write(bout)

        elif self._RESOURCES[name].container is Vector:
            # TODO: refactor to use less copy-pasta
            initializer = self._RESOURCES[name].initializer
            for v in value:
                ArchiveBuilder.validate_structure_fields(name, v, initializer)

            fout.write(int(initializer._SIZE_IN_BYTES * len(value)).to_bytes(8, byteorder="little"))
            for v in value:
                bout = bytearray(initializer._SIZE_IN_BYTES) 
                for (key, field) in initializer._FIELDS.items():
                    write_value(bout, field.offset, field.width, field.is_signed, v[key])

                fout.write(bout)

        elif self._RESOURCES[name].container is Multivector:
            # TODO: write multi-vector
            initializer_list = self._RESOURCES[name].initializer
            size_in_bits = initializer_list[0]._SIZE_IN_BITS

            def valid_structure_name(_obj):
                return _obj['name'] in [_initializer._NAME for _initializer in initializer_list[1:]]

            def validate_fields(_obj):
                matched_obj_list = [_initializer for _initializer in initializer_list[1:] if _initializer._NAME == _obj['name']]
                if len(matched_obj_list) == 1:
                    ArchiveBuilder.validate_structure_fields(name, _obj['attributes'], matched_obj_list[0])

            for sub_list in value:
                for obj in sub_list:
                    if not valid_structure_name(obj):
                        """
                        Validate if passed structure is part of Mulitvector definition
                        """
                        raise UnknownStructureError(obj['name'])
                    validate_fields(obj)

            #Write placeholder for size
            #TODO - Write correct length writing
            fout.write(int(0).to_bytes(8, byteorder="little"))
            #TODO: Find out how to write data points
            index_writer = IndexWriter(name, self._resource_writer, initializer_list[0]._SIZE_IN_BYTES)
            # Write 
            data_point = 0
            data_size = 0

            for sub_list in value:
                index_writer.add(data_point)

                if len(sub_list) > 0:
                    for obj in sub_list:
                        # find out correct initializer
                        type_index, matched_initializer = [(index, element) for index, element in enumerate(initializer_list[1:]) if element._NAME == obj['name']][0]

                        data_size += matched_initializer._SIZE_IN_BYTES +1
                        data_point += matched_initializer._SIZE_IN_BYTES +1

                        bout = bytearray(matched_initializer._SIZE_IN_BYTES + 1)
                        bout[0] = int(type_index).to_bytes(1, byteorder="little", signed=False)[0]
                        for (key, field) in matched_initializer._FIELDS.items():
                            write_value(bout, field.offset + 1 * 8, field.width, field.is_signed, obj['attributes'][key])

                        fout.write(bout)

            # Add sentinel data point
            index_writer.add(data_point)
            index_writer.finish()
            self._resources_written.append(f'{name}.index')

        elif self._RESOURCES[name].container is RawData:
            # TODO: should we do some checks here? what checks?
            fout.write(value)
        else:
            NotImplementedError

        # write trailing 8 zero bytes
        fout.write(b"\x00\x00\x00\x00\x00\x00\x00\x00")
        fout.close()

        self._resources_written.append(name)

    def start(self, name):
        """
        Start writing a resource for this archive incrementally.
        Can only be started once. `set` and `start` can't be used for the same resource.
        :raises $already_set_error
        :raises $already_start_error
        :raises $unknown_resource_error
        :param name: name of the resource
        """
        NotImplementedError

    def finish(self):
        """
        Checks that all required resources are created.
        :raises RuntimeError
        """
        for (name, resource) in self._RESOURCES.items():
            if not resource.is_optional and name not in self._resources_written:
                raise RuntimeError(f'Required resource "{name}" not created. Finished archive is invalid.')
