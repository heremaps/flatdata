'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from pyparsing import ParseResults

from flatdata.generator.tree.nodes.node import Node


class Namespace(Node):
    def __init__(self, name: str, properties: ParseResults | None = None) -> None:
        super().__init__(name=name, properties=properties)
