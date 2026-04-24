
from abc import ABC, abstractmethod

from typing import Any

from flatdata.generator.tree.nodes.explicit_reference import ExplicitReference
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.references import BuiltinStructureReference, StructureReference


class ResourceBase(Node, ABC):
    def __init__(self, name: str, properties: Any = None) -> None:
        super().__init__(name=name, properties=properties)
        self._decorations: list[Any] = []
        self._max_size: int | None = None
        if properties is not None and 'decorations' in properties:
            self._decorations = properties.decorations
            for decoration in self._decorations:
                if 'explicit_reference' in decoration:
                    self.insert(ExplicitReference.create(properties=decoration.explicit_reference))

    @abstractmethod
    def create_references(self) -> list[Node]:
        pass

    @property
    def optional(self) -> bool:
        return any(['optional' in d for d in self.decorations])

    @property
    def doc(self) -> Any:
        return self._properties.doc

    @property
    def decorations(self) -> list[Any]:
        return self._decorations

    @property
    def explicit_references(self) -> list[ExplicitReference]:
        return self.children_like(ExplicitReference)

    @property
    def referenced_structures(self) -> list[Any]:
        return self.children_like(BuiltinStructureReference) + self.children_like(StructureReference)

    @property
    def max_size(self) -> int | None:
        return self._max_size

    @max_size.setter
    def max_size(self, value: int | None) -> None:
        self._max_size = value
