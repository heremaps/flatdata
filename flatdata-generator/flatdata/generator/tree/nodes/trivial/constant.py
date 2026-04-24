from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.helpers.basictype import BasicType
from flatdata.generator.tree.errors import InvalidConstantValueError

from pyparsing import ParseResults


class Constant(Node):
    def __init__(self, name: str, properties: ParseResults | None = None) -> None:
        super().__init__(name=name, properties=properties)
        if properties:
            self._value = int(properties.value, 0)
            if self.type.bits_required(self.value) > self.type.width:
                raise InvalidConstantValueError(name=name, value=self.value)

    @staticmethod
    def create(properties: ParseResults, definition: str) -> 'Constant':
        result = Constant(name=properties.name, properties=properties)
        return result

    @property
    def type(self) -> BasicType:
        assert self._properties is not None
        return BasicType(self._properties.type)

    @property
    def doc(self) -> str:
        assert self._properties is not None
        doc = self._properties.doc
        return str(doc) if doc is not None else ""

    @property
    def value(self) -> int:
        return self._value
