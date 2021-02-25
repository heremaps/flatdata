'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from collections import namedtuple

_SCHEMA_EXT = ".schema"

ResourceSignature = namedtuple("ResourceSignature",
                               ["container", "initializer", "schema", "is_optional", "doc"])


class ArchiveBuilder:
    """
    Archive class. Entry point to Flatdata.
    Provides access to flatdata resources and verifies archive/resource schemas on opening.
    """

    def __init__(self, resource_storage):
        """
        Opens archive from a given resource storage.
        :param resource_storage: Resource storage to use.
        """
        self._resource_storage = resource_storage

    @classmethod
    def name(cls):
        return cls._NAME

    @classmethod
    def schema(cls):
        return cls._SCHEMA

    def set(self, name, value):
        """
        Write a resource for this archive.
        Can only be done once. `set` and `start` can't be used for the same resource.
        :raises $already_set_error
        :raises $already_start_error
        :raises $unknown_resource_error
        :param name: name of the resource
        :param value: value to write
        """
        NotImplemented

    def start(self, name):
        NotImplemented