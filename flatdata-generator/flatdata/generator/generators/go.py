'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.resources import Instance, Vector, Multivector, RawData
from flatdata.generator.tree.nodes.resources.archive import Archive as ArchiveResource
from flatdata.generator.tree.nodes.trivial import Structure, Constant

from . import BaseGenerator


class GoGenerator(BaseGenerator):
    """Flatdata to Go generator"""

    def __init__(self):
        BaseGenerator.__init__(self, "go/go.jinja2")

    def supported_nodes(self):
        return [Structure, Archive, Constant]

    def _populate_environment(self, env):
        def _decorate_archive_type(value):
            assert isinstance(value, Node)
            return value.name

        def to_go_doc(value):
            lines = value.doc.splitlines()
            return '\n'.join(["// " + s for s in lines if len(s) != 0])

        def type_mapping(flatdata_type, _struct):
            if is_bool(flatdata_type):
                return "uint8"
            return go_mapping(flatdata_type)

        def type_mapping_with_bool(flatdata_type):
            if is_bool(flatdata_type):
                return "bool"
            return go_mapping(flatdata_type)

        def go_mapping(flatdata_type):
            return {
                "i8": "int8",
                "u8": "uint8",
                "u16": "uint16",
                "i16": "int16",
                "u32": "uint32",
                "i32": "int32",
                "u64": "uint64",
                "i64": "int64"
            }[flatdata_type]

        def is_bool(flatdata_type):
            return flatdata_type == "bool"

        def to_go_case(name, exported=True):
            if "_" in name:
                name = "".join(part.title() for part in name.split("_"))
            return (str.upper if exported else str.lower)(str(name[0])) + str(name[1:])

        def to_initializer(resource, tree):
            if isinstance(resource, Instance):
                return _decorate_archive_type(resource.referenced_structures[0].node)
            if isinstance(resource, Vector):
                return _decorate_archive_type(resource.referenced_structures[0].node)
            if isinstance(resource, Multivector):
                return "[{}]".format(','.join(
                    [_decorate_archive_type(t.node)
                     for t in resource.referenced_structures]
                ))
            if isinstance(resource, ArchiveResource):
                return _decorate_archive_type(resource.children[0].node)
            if isinstance(resource, RawData):
                return "None"
            raise ValueError("Unknown resource type: %s" % (resource.__class__))

        def get_types_for_multivector(resource, _tree):
            return [_decorate_archive_type(t.node) for t in resource.referenced_structures]

        def contains_archive_resource(tree):
            for child in tree.root.children[0].children:
                for res in child.children:
                    if isinstance(res, ArchiveResource):
                        return True
            return False

        env.filters['to_go_doc'] = to_go_doc
        env.filters['to_initializer'] = to_initializer
        env.filters['type_mapping'] = type_mapping
        env.filters['type_mapping_with_bool'] = type_mapping_with_bool
        env.filters['to_go_case'] = to_go_case
        env.filters['is_bool'] = is_bool
        env.filters['get_types_for_multivector'] = get_types_for_multivector
        env.filters['contains_archive_resource'] = contains_archive_resource
