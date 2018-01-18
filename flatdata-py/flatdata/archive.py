'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from collections import namedtuple

import pandas as pd

from .errors import SchemaMismatchError

ResourceSignature = namedtuple("ResourceSignature",
                               ["container", "initializer", "schema", "is_optional"])

_SCHEMA_EXT = ".schema"


class Archive(object):
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
        for name, signature in sorted(list(self._RESOURCES.items())):
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
        return sum(
            self.__getattr__(resource).size_in_bytes()
            for resource in self._RESOURCES.keys()
        )

    def __len__(self):
        return len(self._RESOURCES)

    def _open_resource(self, name):
        resource = self._RESOURCES[name]
        self._check_non_subarchive_schema(name, resource)
        return resource.container.open(self._resource_storage, name, resource.initializer, resource.is_optional)

    @staticmethod
    def _is_archive():
        """
        Necessary to distinguish between archive and normal resources in a reliable manner.
        isinstance fails to do the check with current module structure.
        https://stackoverflow.com/questions/38514730/isinstance-returns-false-when-the-fully-qualified-object-class-differs-from-th
        """
        return True

    def _check_non_subarchive_schema(self, name, resource):
        if resource.container._is_archive():
            return
        actual_schema = self._resource_storage.get(name + _SCHEMA_EXT).read().decode()
        if actual_schema != resource.schema:
            raise SchemaMismatchError(name, resource.schema.splitlines(), actual_schema.splitlines())
