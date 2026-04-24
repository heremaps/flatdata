'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.nodes.trivial import Field
from . import BaseGenerator

from jinja2 import Environment

SCOPE_SEPARATOR = "__"
DECORATION_BOUND = "__bound__"


class DotGenerator(BaseGenerator):
    """Flatdata to DOT (graph description language) generator"""

    def __init__(self) -> None:
        BaseGenerator.__init__(self, "dot/dot.jinja2")

    def _populate_environment(self, env: Environment) -> None:
        env.autoescape = True

        def _field_value_type(field: Field) -> str:
            assert field.type is not None
            assert field.parent is not None
            assert field.parent.parent is not None
            type_name = str(field.type.name).replace("@@", ".").replace("@", ".")
            namespace_name = str(field.parent.parent.path)
            if type_name.startswith(namespace_name):
                type_name = type_name[len(namespace_name):]
            if type_name.startswith("."):
                type_name = type_name[1:]

            return type_name

        env.filters["field_value_type"] = _field_value_type

    def supported_nodes(self) -> list[type]:
        return [Archive]
