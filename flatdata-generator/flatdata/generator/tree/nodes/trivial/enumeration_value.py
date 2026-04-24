from flatdata.generator.tree.nodes.node import Node

from typing import Any


class EnumerationValue(Node):
    def __init__(self, name: str, value: int, auto_generated: bool, properties: Any = None) -> None:
        super().__init__(name=name, properties=properties)
        self._value = value
        self._auto_generated = auto_generated

    @staticmethod
    def create(properties: Any, value: int) -> 'EnumerationValue':
        result = EnumerationValue(name=properties.name, properties=properties, value=value, auto_generated=False)
        return result

    @property
    def doc(self) -> str:
        doc = self._properties.doc
        return str(doc) if doc is not None else ""

    @property
    def value(self) -> int:
        return self._value

    @property
    def auto_generated(self) -> bool:
        return self._auto_generated
