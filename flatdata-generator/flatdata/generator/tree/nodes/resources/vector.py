from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.references import StructureReference
from .base import ResourceBase

from pyparsing import ParseResults


class Vector(ResourceBase):
    def __init__(self, name: str, properties: ParseResults | None = None, type: str | None = None) -> None:
        super().__init__(name=name, properties=properties)
        self._type = type

    @staticmethod
    def create(properties: ParseResults) -> 'Vector':
        return Vector(name=properties.name,
                      properties=properties,
                      type=properties.type.vector.type)

    def create_references(self) -> list[Node]:
        assert self._type is not None
        return [StructureReference(name=self._type)]
