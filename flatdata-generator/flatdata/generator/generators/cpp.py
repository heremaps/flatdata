'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from flatdata.generator.tree.nodes.resources import Vector, Multivector, Instance, RawData, BoundResource, \
    ResourceBase, Archive as ArchiveResource
from flatdata.generator.tree.nodes.trivial import Structure, Enumeration, Constant, Field
from flatdata.generator.tree.nodes.archive import Archive
from . import BaseGenerator


class CppGenerator(BaseGenerator):
    """Flatdata to C++ header file generator"""

    def __init__(self):
        BaseGenerator.__init__(self, "cpp/cpp.jinja2")

    def supported_nodes(self):
        return [Structure, Archive, Constant, Enumeration]

    def _populate_environment(self, env):
        env.filters["cpp_doc"] = lambda value: value

        def _safe_cpp_string_line(value):
            return value.replace('\\', '\\\\').replace('"', r'\"')

        env.filters["safe_cpp_string_line"] = _safe_cpp_string_line

        def _cpp_base_type(flatdata_type):
            type_map = {
                "bool": "bool",
                "i8": "int8_t",
                "u8": "uint8_t",
                "u16": "uint16_t",
                "i16": "int16_t",
                "u32": "uint32_t",
                "i32": "int32_t",
                "u64": "uint64_t",
                "i64": "int64_t"
            }
            if flatdata_type.name in type_map:
                return type_map[flatdata_type.name]
            return flatdata_type.name.replace("@@", "::").replace("@", "::")

        env.filters["cpp_base_type"] = _cpp_base_type

        def _to_type_params(refs):
            return ', '.join([ref.node.path_with("::") for ref in refs])

        env.filters["to_type_params"] = _to_type_params

        def _typedef_name(entity, extra_suffix=""):
            assert isinstance(entity, (Field, ResourceBase)), "Got: %s" % entity.__class__
            return "".join([c.title() for c in entity.name.split('_')]) + extra_suffix + "Type"

        env.filters["typedef_name"] = _typedef_name

        def _optional_typedef_usage(resource, extra_suffix=""):
            def _wrap_in_optional(declaration):
                if resource.optional:
                    return "boost::optional< %s >" % declaration
                return declaration

            return _wrap_in_optional(_typedef_name(resource, extra_suffix))

        env.filters["archive_typedef_usage"] = _optional_typedef_usage

        def _resource_provides_incremental_builder(resource):
            assert isinstance(resource, ResourceBase)
            if isinstance(resource, Instance):
                return False
            if isinstance(resource, Vector):
                return True
            if isinstance(resource, Multivector):
                return True
            if isinstance(resource, RawData):
                return False
            raise ValueError("Unknown resource type %s" % (resource.__class__))

        env.filters[
            "resource_provides_incremental_builder"] = _resource_provides_incremental_builder

        def provides_setter(resource):
            assert isinstance(resource, ResourceBase)
            if isinstance(resource, Instance):
                return True
            if isinstance(resource, Vector):
                return True
            if isinstance(resource, Multivector):
                return False
            if isinstance(resource, RawData):
                return True
            raise ValueError("Unknown resource type %s" % (resource.__class__))

        env.filters["provides_setter"] = provides_setter

        env.filters["supported_resources"] = lambda l: [x for x in l if
                                                        not isinstance(x, BoundResource)]
        env.filters["simple_resources"] = lambda l: [x for x in l if
                                                     not isinstance(x, BoundResource) and
                                                     not isinstance(x, ArchiveResource)]
        env.filters["archive_resources"] = lambda l: [x for x in l if
                                                      isinstance(x, ArchiveResource)]
