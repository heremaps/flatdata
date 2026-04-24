from flatdata.generator.tree.nodes.node import Node
from .base import ResourceBase

from typing import Any


class RawData(ResourceBase):
    def __init__(self, name: str, properties: Any = None) -> None:
        super(RawData, self).__init__(name=name, properties=properties)

    @staticmethod
    def create(properties: Any) -> 'RawData':
        return RawData(name=properties.name, properties=properties)

    def create_references(self) -> list[Node]:
        return []
