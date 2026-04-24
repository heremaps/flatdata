'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
from jinja2 import Environment

from typing import Any

from flatdata.generator.tree.nodes.resources import Instance, Vector, Multivector, RawData
from flatdata.generator.tree.nodes.resources.archive import Archive as ArchiveResource
from flatdata.generator.tree.nodes.trivial import Structure
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.archive import Archive
from . import BaseGenerator


class PythonGenerator(BaseGenerator):
    """Flatdata to Python generator"""

    def __init__(self) -> None:
        BaseGenerator.__init__(self, "py/python.jinja2")

    def supported_nodes(self) -> list[type]:
        return [Structure, Archive]

    def _populate_environment(self, env: Environment) -> None:
        def _decorate_archive_type(tree: Any, value: Node) -> str:
            assert isinstance(value, Node)
            return str(tree.namespace_path(value, "_") + "_" + value.name)

        def to_python_doc(value: str) -> str:
            return '\n'.join(
                ["# " + line.replace('/**', '', 1).replace('*/', '', 1).replace(" *", '',
                                                                                1).replace("//", "",
                                                                                           1) for
                 line in value.splitlines()])

        def to_container(resource: Any) -> str:
            if isinstance(resource, Instance):
                return "flatdata.resources.Instance"
            if isinstance(resource, Vector):
                return "flatdata.resources.Vector"
            if isinstance(resource, Multivector):
                return "flatdata.resources.Multivector"
            if isinstance(resource, RawData):
                return "flatdata.resources.RawData"
            if isinstance(resource, ArchiveResource):
                return "flatdata.archive.Archive"
            raise ValueError("Unknown resource type: %s" % (resource.__class__))

        def to_initializer(resource: Any, tree: Any) -> str:
            if isinstance(resource, Instance):
                return str(_decorate_archive_type(tree, resource.referenced_structures[0].node))
            if isinstance(resource, Vector):
                return str(_decorate_archive_type(tree, resource.referenced_structures[0].node))
            if isinstance(resource, Multivector):
                return "[{}]".format(
                    ','.join([_decorate_archive_type(tree, t.node) for t in
                              resource.referenced_structures]))
            if isinstance(resource, ArchiveResource):
                return str(_decorate_archive_type(tree, resource.children[0].node))  # type: ignore[attr-defined]  # child is an ArchiveReference which has .node
            if isinstance(resource, RawData):
                return "None"
            raise ValueError("Unknown resource type: %s" % (resource.__class__))

        def to_dtype(field: Any) -> str:
            type_map = {
                "bool": "?",
                "i8": "b",
                "u8": "B",
                "u16": "u2",
                "i16": "i2",
                "u32": "u4",
                "i32": "i4",
                "u64": "u8",
                "i64": "i8"
            }
            if field.type.name in type_map:
                return type_map[field.type.name]
            return str(type_map[field.type_reference.node.type.name])

        def _safe_py_string_line(value: str) -> str:
            return value.replace('\\', '\\\\').replace('"', r'\"')

        env.filters["safe_py_string_line"] = _safe_py_string_line
        env.filters['to_python_doc'] = to_python_doc
        env.filters['to_container'] = to_container
        env.filters['to_initializer'] = to_initializer
        env.filters['to_dtype'] = to_dtype
