from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.helpers.basictype import BasicType
from flatdata.generator.tree.errors import InvalidConstantValueError

from typing import Any


class Constant(Node):
    def __init__(self, name: str, properties: Any = None) -> None:
        super().__init__(name=name, properties=properties)
        if properties:
            self._value = int(properties.value, 0)
            if self.type.bits_required(self.value) > self.type.width:
                raise InvalidConstantValueError(name=name, value=self.value)

    @staticmethod
    def create(properties: Any, definition: Any) -> 'Constant':
        result = Constant(name=properties.name, properties=properties)
        return result

    @property
    def type(self) -> BasicType:
        return BasicType(self._properties.type)

    @property
    def doc(self) -> Any:
        return self._properties.doc

    @property
    def value(self) -> int:
        return self._value
