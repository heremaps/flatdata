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

    def get(self, key, is_subarchive):
        filename = os.path.join(self.path, key)
        if os.path.exists(filename):
            raise ResourceExistsError(key)
        
        if is_subarchive:
            return FileResourceWriter(filename)
        
        return open(filename, 'wb')


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

    @classmethod
    def name(cls):
        return cls._NAME

    @classmethod
    def schema(cls):
        return cls._SCHEMA

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
        # TODO: fail if trying to set subarchive?
        fout = self._resource_writer.get(name, False)
        if self._RESOURCES[name].container is Instance:
            initializer = self._RESOURCES[name].initializer
            for key in initializer._FIELD_KEYS:
                if key not in value:
                    raise FieldMissingError(key, name)
            for key in value.keys():
                if key not in initializer._FIELD_KEYS:
                    raise UnknownFieldError(key, name)
            
            fout.write(int(initializer._SIZE_IN_BYTES).to_bytes(8, byteorder="little"))
            bout = bytearray(initializer._SIZE_IN_BYTES) 
            for (key, field) in initializer._FIELDS.items():
                write_value(bout, field.offset, field.width, field.is_signed, value[key])

            fout.write(bout)

        elif self._RESOURCES[name].container is Vector:
            # TODO: write vector; re-use writing instance code
            NotImplementedError
        elif self._RESOURCES[name].container is Multivector:
            # TODO: write multi-vector
            NotImplementedError   
        elif self._RESOURCES[name].container is RawData:
            # TODO: should we do some checks here? what checks?
            fout.write(value)
        else:
            NotImplementedError

        fout.write(b"\x00\x00\x00\x00\x00\x00\x00\x00")
        fout.close()

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
