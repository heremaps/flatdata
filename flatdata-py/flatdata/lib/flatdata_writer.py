'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from __future__ import annotations

from typing import Any, TYPE_CHECKING

from flatdata.generator.engine import Engine
from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.errors import FlatdataSyntaxError

from .resource_storage import ResourceStorage
from .file_resource_writer import FileResourceWriter

if TYPE_CHECKING:
    from .archive_builder import ArchiveBuilder


class Writer:
    '''
    This class is responsible for generating python module based on provided schema
    and use it create flatdata files. This class expects data in json format.

    It does not support incremental data. All the data has to provided once for creating
    flatdata.
    '''

    def __init__(self, archive_schema: str, path: str, archive_name: str = "") -> None:
        '''
        Creates instance of Writer class from a schema string.
        The schema must be self-contained (no import statements).

        :param archive_schema(str): flatdata schema
        :param path(str): file path where flatdata files are created
        :param archive_name(str): name of the archive (inferred if empty)
        '''
        try:
            engine = Engine(archive_schema)
            self._init_from_engine(engine, path, archive_name)
        except FlatdataSyntaxError as err:
            raise RuntimeError(
                "Error in generating modules from provided schema: %s " % err) from err

    @classmethod
    def from_file(cls, schema_path: str, path: str, archive_name: str = "") -> 'Writer':
        '''
        Creates instance of Writer class from a schema file, resolving imports.

        :param schema_path(str): path to the flatdata schema file
        :param path(str): file path where flatdata files are created
        :param archive_name(str): name of the archive (inferred if empty)
        '''
        writer = cls.__new__(cls)
        try:
            engine = Engine.from_file(schema_path)
            writer._init_from_engine(engine, path, archive_name)
        except FlatdataSyntaxError as err:
            raise RuntimeError(
                "Error in generating modules from provided schema: %s " % err) from err
        return writer

    def _init_from_engine(self, engine: Engine, path: str, archive_name: str) -> None:
        '''Shared initialization from an Engine instance.'''
        if not archive_name:
            archive_name = Writer._find_archive_name(engine)
        module, archive_type = engine.render_python_module(
            archive_name=archive_name)
        builder_type = getattr(module, archive_type.__name__ + "Builder")
        self.builder: ArchiveBuilder = builder_type(
            ResourceStorage(FileResourceWriter(), path))

    def set(self, resource_name: str, resource_data: Any) -> None:
        '''
        It is the setter for flatdata creation. Expects data in JSON format. Caller has to provide
        resource name which is the flatdata schema.

        :param resource_name(str): name of resource
        :param resource_data(str): JSON data in str
        '''
        self.builder.set(resource_name, resource_data)

    def finish(self) -> None:
        '''Completes flatdata creation'''
        self.builder.finish()

    @staticmethod
    def _find_archive_name(engine: Engine) -> str:
        '''
        Finds the archive name from the AST, preferring local archives.

        :raises RuntimeError: if no archive or multiple ambiguous archives found
        '''
        all_archives = list(engine.tree.root.iterate(Archive))
        local_archives = [a for a in all_archives if a.is_local]

        # Prefer local archives when imports are present
        candidates = local_archives if local_archives else all_archives

        if len(candidates) == 0:
            raise RuntimeError("No archive found in schema")
        if len(candidates) > 1:
            raise RuntimeError(
                "Schema contains multiple archives, please specify archive name explicitly")
        return candidates[0].name
