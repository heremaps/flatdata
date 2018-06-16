'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.tree.nodes.resources import Vector, Multivector, Instance, RawData, BoundResource, \
    ResourceBase, Archive as ArchiveResource
from generator.tree.nodes.trivial import Structure, Enumeration, Constant, Field
from generator.tree.nodes.archive import Archive
from generator.tree.syntax_tree import SyntaxTree
from .BaseGenerator import BaseGenerator


class FlatdataGenerator(BaseGenerator):
    def __init__(self):
        BaseGenerator.__init__(self, "flatdata/flatdata.jinja2")

    def _supported_nodes(self):
        return [Structure, Archive, Constant, Enumeration]

    def _populate_environment(self, env):

        def _is_builtin(node):
            for x in SyntaxTree.namespaces(node):
                if x.name == "_builtin":
                    return True;
            return False
        env.filters["filter_builtin"] = lambda l: [x for x in l if not _is_builtin(x)]

        def _safe_cpp_string_line(value):
            return value.replace('\\', '\\\\').replace('"', r'\"')

        env.filters["safe_cpp_string_line"] = _safe_cpp_string_line

        def _field_type(t):
            return t.replace("@@", ".").replace("@", ".")

        env.filters["field_type"] = _field_type

        def _to_type_params(refs):
            return ', '.join([ref.node.path_with(".") for ref in refs if not _is_builtin(ref.node)])

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

        env.filters["supported_resources"] = lambda l: [x for x in l if
                                                        not isinstance(x, BoundResource)]
        env.filters["simple_resources"] = lambda l: [x for x in l if
                                                     not isinstance(x, BoundResource) and
                                                     not isinstance(x, ArchiveResource)]
        env.filters["archive_resources"] = lambda l: [x for x in l if
                                                      isinstance(x, ArchiveResource)]
