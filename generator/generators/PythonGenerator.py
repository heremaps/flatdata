'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
from generator.tree.nodes.resources import Instance, Vector, Multivector, RawData
from generator.tree.nodes.resources.archive import Archive as ArchiveResource
from generator.tree.nodes.trivial import Structure
from generator.tree.nodes.node import Node
from generator.tree.nodes.archive import Archive
from .BaseGenerator import BaseGenerator


class PythonGenerator(BaseGenerator):
    def __init__(self):
        BaseGenerator.__init__(self, "py/python.jinja2")

    def _supported_nodes(self):
        return [Structure, Archive]

    def _populate_environment(self, env):
        def _decorate_archive_type(tree, value):
            assert isinstance(value, Node)
            return tree.namespace_path(value, "_") + "_" + value.name

        def to_python_doc(value):
            return '\n'.join(
                ["# " + line.replace('/**', '', 1).replace('*/', '', 1).replace(" *", '',
                                                                                1).replace("//", "",
                                                                                           1) for
                 line in value.splitlines()])

        def to_container(resource):
            if isinstance(resource, Instance):
                return "flatdata.resources.Instance"
            elif isinstance(resource, Vector):
                return "flatdata.resources.Vector"
            elif isinstance(resource, Multivector):
                return "flatdata.resources.Multivector"
            elif isinstance(resource, RawData):
                return "flatdata.resources.RawData"
            elif isinstance(resource, ArchiveResource):
                return "flatdata.archive.Archive"
            assert False, "Unknown resource type: %s" % (resource.__class__)

        def to_initializer(resource, tree):
            if isinstance(resource, Instance):
                return _decorate_archive_type(tree, resource.referenced_structures[0].node)
            elif isinstance(resource, Vector):
                return _decorate_archive_type(tree, resource.referenced_structures[0].node)
            elif isinstance(resource, Multivector):
                return "[{}]".format(
                    ','.join([_decorate_archive_type(tree, t.node) for t in resource.referenced_structures]))
            elif isinstance(resource, ArchiveResource):
                return _decorate_archive_type(tree, resource.children[0].node)
            elif isinstance(resource, RawData):
                return "None"
            assert False, "Unknown resource type: %s" % (resource.__class__)

        def _safe_py_string_line(value):
            return value.replace('\\', '\\\\').replace('"', r'\"')

        env.filters["safe_py_string_line"] = _safe_py_string_line
        env.filters['to_python_doc'] = to_python_doc
        env.filters['to_container'] = to_container
        env.filters['to_initializer'] = to_initializer

