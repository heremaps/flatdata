from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.references import StructureReference
from .base import ResourceBase

from pyparsing import ParseResults


class Instance(ResourceBase):
    def __init__(self, name: str, properties: ParseResults | None = None, resource_type: str | None = None) -> None:
        super().__init__(name=name, properties=properties)
        self._type = resource_type

    @staticmethod
    def create(properties: ParseResults) -> 'Instance':
        return Instance(name=properties.name,
                        properties=properties,
                        resource_type=properties.type.object.type)

    def create_references(self) -> list[Node]:
        assert self._type is not None
        return [StructureReference(name=self._type)]

    @property
    def type(self) -> str | None:
        return self._type
