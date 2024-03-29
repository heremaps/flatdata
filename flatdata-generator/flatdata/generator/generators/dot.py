'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from flatdata.generator.tree.nodes.archive import Archive
from . import BaseGenerator

SCOPE_SEPARATOR = "__"
DECORATION_BOUND = "__bound__"


class DotGenerator(BaseGenerator):
    """Flatdata to DOT (graph description language) generator"""

    def __init__(self):
        BaseGenerator.__init__(self, "dot/dot.jinja2")

    def _populate_environment(self, env):
        env.autoescape = True

        def _field_value_type(field):
            type_name = field.type.name.replace("@@", ".").replace("@", ".")
            namespace_name = field.parent.parent.path
            if type_name.startswith(namespace_name):
                type_name = type_name[len(namespace_name):]
            if type_name.startswith("."):
                type_name = type_name[1:]

            return type_name

        env.filters["field_value_type"] = _field_value_type

    def supported_nodes(self):
        return [Archive]
