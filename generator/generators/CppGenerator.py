'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.tree.nodes.resources import Vector, Multivector, Instance, RawData, BoundResource, \
    ResourceBase, Archive as ArchiveResource
from generator.tree.nodes.trivial import Structure, Enumeration, Constant, Field
from generator.tree.nodes.archive import Archive
from .BaseGenerator import BaseGenerator


class CppGenerator(BaseGenerator):
    def __init__(self):
        BaseGenerator.__init__(self, "cpp/cpp.jinja2")

    def _supported_nodes(self):
        return [Structure, Archive, Constant, Enumeration]

    def _populate_environment(self, env):
        env.filters["cpp_doc"] = lambda value: value

        def _safe_cpp_string_line(value):
            return value.replace('\\', '\\\\').replace('"', r'\"')

        env.filters["safe_cpp_string_line"] = _safe_cpp_string_line

        def _cpp_base_type(t):
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
            if t.name in type_map:
                return type_map[t.name]
            return t.name.replace("@@", "::").replace("@", "::")

        env.filters["cpp_base_type"] = _cpp_base_type

        def _to_type_params(refs):
            return ', '.join([ref.node.path_with("::") for ref in refs])

        env.filters["to_type_params"] = _to_type_params

        def _typedef_name(entity, extra_suffix=""):
            assert isinstance(entity, Field) or isinstance(entity,
                                                           ResourceBase), "Got: %s" % \
                                                                          entity.__class__
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
            elif isinstance(resource, Vector):
                return True
            elif isinstance(resource, Multivector):
                return True
            elif isinstance(resource, RawData):
                return False
            assert False, "Unknown resource type %s" % (resource.__class__)

        env.filters[
            "resource_provides_incremental_builder"] = _resource_provides_incremental_builder

        def provides_setter(resource):
            assert isinstance(resource, ResourceBase)
            if isinstance(resource, Instance):
                return True
            elif isinstance(resource, Vector):
                return True
            elif isinstance(resource, Multivector):
                return False
            elif isinstance(resource, RawData):
                return True
            assert False, "Unknown resource type %s" % (resource.__class__)

        env.filters["provides_setter"] = provides_setter

        env.filters["supported_resources"] = lambda l: [x for x in l if
                                                        not isinstance(x, BoundResource)]
        env.filters["simple_resources"] = lambda l: [x for x in l if
                                                     not isinstance(x, BoundResource) and
                                                     not isinstance(x, ArchiveResource)]
        env.filters["archive_resources"] = lambda l: [x for x in l if
                                                      isinstance(x, ArchiveResource)]
