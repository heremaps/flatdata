'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from flatdata.generator.engine import Engine
from flatdata.generator.tree.errors import FlatdataSyntaxError

from .resource_storage import ResourceStorage
from .file_resource_writer import FileResourceWriter


class Writer:
    '''
    This class is responsible for generating python module based on provided schema
    and use it create flatdata files. This class expects data in json format.

    It does not support incremental data. All the data has to provided once for creating
    flatdata.
    '''

    def __init__(self, archive_schema, path, archive_name=""):
        '''
        Creates instance or Writer class. Archive module is rendered by engine
        using provided schema.

        :param archive_schema(str): flatdata schema
        :param path(str): file path where flatdata files are created
        '''
        try:
            if not archive_name:
                archive_name = Writer._get_archive_name(
                    archive_schema)
            _, archive_type = Engine(archive_schema).render_python_module(
                archive_name=archive_name + "Builder")
        except FlatdataSyntaxError as err:
            raise RuntimeError(
                "Error in generating modules from provided schema: %s " % err)

        self.builder = archive_type(
            ResourceStorage(FileResourceWriter(), path))

    def set(self, resource_name, resource_data):
        '''
        It is the setter for flatdata creation. Expects data in JSON format. Caller has to provide
        resource name which is the flatdata schema.

        :param resource_name(str): name of resource
        :param resource_data(str): JSON data in str
        '''
        self.builder.set(resource_name, resource_data)

    def finish(self):
        '''Completes flatdata creation'''
        self.builder.finish()

    @classmethod
    def _get_archive_name(cls, archive_schema):
        '''
        Returns name of archive from flatdata schema.

        :param archive_schema(str): flatdata schema in str
        '''
        if not archive_schema:
            raise RuntimeError("Archive schema is required")

        archive_keyword = "archive"
        index = archive_schema.find(archive_keyword) + len(archive_keyword)
        if archive_schema[index:].find(archive_keyword) >= 0:
            raise RuntimeError(
                "Schema contains multiple archives, please specify archive name explicitly")
        return archive_schema[index:index+archive_schema[index:].find("{")].strip()
