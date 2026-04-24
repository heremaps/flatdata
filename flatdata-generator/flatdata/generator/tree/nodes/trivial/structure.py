from flatdata.generator.tree.nodes.node import Node
from .field import Field

from typing import Any


class Structure(Node):
    def __init__(self, name: str, properties: Any = None) -> None:
        """
        Use to instantiate empty structure.
        No special properties are evaluated.

        :param name: name
        :param properties: properties. can be missing.
        """
        super().__init__(name=name, properties=properties)

    @staticmethod
    def create(properties: Any, definition: Any) -> 'Structure':
        result = Structure(name=properties.name, properties=properties)

        for field in properties.fields:
            result.insert(Field.create(properties=field))
        return result

    @property
    def has_range(self) -> bool:
        return any(f for f in self.fields if f.range)

    @property
    def doc(self) -> Any:
        return self._properties.doc

    @property
    def size_in_bits(self) -> int:
        return self._size_in_bits

    @size_in_bits.setter
    def size_in_bits(self, value: int) -> None:
        self._size_in_bits = value

    @property
    def size_in_bytes(self) -> int:
        return (self._size_in_bits + 7) // 8

    @property
    def fields(self) -> list[Field]:
        return self.children_like(Field)
