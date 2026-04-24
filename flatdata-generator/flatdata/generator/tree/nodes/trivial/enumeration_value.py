from flatdata.generator.tree.nodes.node import Node

from pyparsing import ParseResults


class EnumerationValue(Node):
    def __init__(self, name: str, value: int, auto_generated: bool, properties: ParseResults | None = None) -> None:
        super().__init__(name=name, properties=properties)
        self._value = value
        self._auto_generated = auto_generated

    @staticmethod
    def create(properties: ParseResults, value: int) -> 'EnumerationValue':
        result = EnumerationValue(name=properties.name, properties=properties, value=value, auto_generated=False)
        return result

    @property
    def doc(self) -> str:
        if self._properties is None:
            return ""
        doc = self._properties.doc
        return str(doc) if doc is not None else ""

    @property
    def value(self) -> int:
        return self._value

    @property
    def auto_generated(self) -> bool:
        return self._auto_generated
