'''
 Copyright (c) 2018 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from jinja2 import Environment

from typing import Any

from flatdata.generator.tree.nodes.resources import BoundResource
from flatdata.generator.tree.nodes.trivial import Structure, Enumeration, Constant
from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.syntax_tree import SyntaxTree
from . import BaseGenerator


class FlatdataGenerator(BaseGenerator):
    """Flatdata to Flatdata generator, used for debugging/testing"""

    def __init__(self) -> None:
        BaseGenerator.__init__(self, "flatdata/flatdata.jinja2")

    def supported_nodes(self) -> list[type]:
        return [Structure, Archive, Constant, Enumeration]

    def _populate_environment(self, env: Environment) -> None:
        def _is_builtin(node: Any) -> bool:
            for namespace in SyntaxTree.namespaces(node):
                if namespace.name == "_builtin":
                    return True
            return False
        env.filters["filter_builtin"] = lambda l: [x for x in l if not _is_builtin(x)]

        def _field_type(flatdata_type: str) -> str:
            return flatdata_type.replace("@@", ".").replace("@", ".")

        env.filters["field_type"] = _field_type

        def _to_type_params(refs: list[Any]) -> str:
            return ', '.join([ref.node.path_with(".") for ref in refs if not _is_builtin(ref.node)])

        env.filters["to_type_params"] = _to_type_params

        env.filters["supported_resources"] = lambda l: [x for x in l if
                                                        not isinstance(x, BoundResource)]

        env.filters["bound_resources"] = lambda l: [x for x in l if
                                                    isinstance(x, BoundResource)]
