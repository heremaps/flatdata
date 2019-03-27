'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from collections import namedtuple

import pandas as pd

from .errors import MissingResourceError, SchemaMismatchError

ResourceSignature = namedtuple("ResourceSignature",
                               ["container", "initializer", "schema", "is_optional", "doc"])

def _is_archive_signature(resource_signature):
    return resource_signature.container == Archive

_SCHEMA_EXT = ".schema"


class Archive:
    """
    Archive class. Entry point to Flatdata.
    Provides access to flatdata resources and verifies archive/resource schemas on opening.
    """

    def __init__(self, resource_storage):
        """
        Opens archive from a given resource storage.
        :raises flatdata.errors.CorruptArchiveError
        :raises flatdata.errors.SchemaMismatchError
        :param resource_storage: Resource storage to use.
        """
        self._resource_storage = resource_storage
        self._loaded_resources = {}

        # Preload resources and check their schemas
        for name, _ in sorted(list(self._RESOURCES.items())):
            self.__getattr__(name)

    def __getattr__(self, name):
        if name not in list(self._RESOURCES.keys()):
            raise AttributeError("Resource %s not defined in archive." % name)
        if name not in list(self._loaded_resources.keys()):
            self._loaded_resources[name] = self._open_resource(name)
        return self._loaded_resources[name]

    def __dir__(self):
        return list(self._RESOURCES.keys()) + ['schema']

    def __repr__(self):
        return self.to_data_frame().__repr__()

    def to_data_frame(self):
        result = []
        for name, signature in self._RESOURCES.items():
            resource = self.__getattr__(name)
            result.append([name, signature.container.__name__, signature.is_optional,
                           resource.size_in_bytes() if resource else None,
                           len(resource) if resource else None])
        return pd.DataFrame(data=result,
                            columns=["Name", "Type", "Optional", "SizeInBytes", "Size"])

    @classmethod
    def name(cls):
        return cls._NAME

    @classmethod
    def schema(cls):
        return cls._SCHEMA

    @classmethod
    def resource_schema(cls, resource):
        return cls._RESOURCES[resource].schema

    @classmethod
    def open(cls, storage, name, initializer, is_optional=False):
        nested_storage = storage.get(name, is_optional)
        assert nested_storage is not None or is_optional
        if nested_storage is None:
            return None
        return initializer(nested_storage)

    def size_in_bytes(self):
        return sum(resource_value.size_in_bytes() for resource_value in
                   (self.__getattr__(resource) for resource in self._RESOURCES.keys())
                   if resource_value)

    def __len__(self):
        return len(self._RESOURCES)

    def _schema_validated_resource_signature(self, name):
        resource_signature = self._RESOURCES[name]
        # We check only schema for non-subarchives, since the subarchives schema is checked,
        # when it is initialized.
        if not _is_archive_signature(resource_signature):
            storage = self._resource_storage.get(name + _SCHEMA_EXT, resource_signature.is_optional)
            if storage:
                Archive._check_non_subarchive_schema(name, resource_signature, storage)
            elif not resource_signature.is_optional:
                raise MissingResourceError(name)
            else:
                return None
        return resource_signature

    def _open_resource(self, name):
        resource_signature = self._schema_validated_resource_signature(name)
        if resource_signature:
            resource = resource_signature.container.open(storage=self._resource_storage,
                                                         name=name,
                                                         initializer=resource_signature.initializer,
                                                         is_optional=resource_signature.is_optional)
            if resource:
                resource.__doc__ = resource_signature.doc
                return resource
        return None

    @staticmethod
    def _check_non_subarchive_schema(name, resource_signature, storage):
        actual_schema = storage.read().decode()
        if actual_schema != resource_signature.schema:
            raise SchemaMismatchError(
                name, resource_signature.schema.splitlines(), actual_schema.splitlines())
