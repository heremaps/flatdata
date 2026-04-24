from flatdata.generator.tree.errors import DuplicateEnumValueError, InvalidEnumValueError, SparseEnumError
from flatdata.generator.tree.helpers.basictype import BasicType
from flatdata.generator.tree.nodes.node import Node
from .enumeration_value import EnumerationValue

from pyparsing import ParseResults


class Enumeration(Node):
    def __init__(self, name: str, properties: ParseResults | None = None, type: str | None = None, width: int | None = None) -> None:
        super().__init__(name=name, properties=properties)
        self._type: BasicType | None = None

        if type is not None:
            self._type = BasicType(name=type, width=width)

    @staticmethod
    def create(properties: ParseResults, definition: str) -> 'Enumeration':
        width = None
        if properties.width:
            width = int(properties.width)
        result = Enumeration(name=properties.name, properties=properties, type=properties.type, width=width)

        current_assigned_value = 0
        unique_values: set[int] = set()
        for value in properties.enum_values:
            if value.constant:
                current_assigned_value = int(value.constant, 0)
            if current_assigned_value in unique_values:
                raise DuplicateEnumValueError(enumeration_name=result._name, value=current_assigned_value)
            unique_values.add(current_assigned_value)
            value_node = EnumerationValue.create(properties=value, value=current_assigned_value)
            result.insert(value_node)
            current_assigned_value += 1

        # we do not want to genarate too many (exponential) values, so restrict to multiples of input size
        assert result._type is not None
        if len(properties.enum_values) * 2 + 256 < 2 ** result._type.width:
            raise SparseEnumError(enumeration_name=result._name, width=result._type.width)

        for missing_value in result._type.value_range():
            if not missing_value in unique_values:
                value_node = EnumerationValue(name="UNKNOWN_VALUE_" + str(missing_value).replace("-", "MINUS_"), value=missing_value, auto_generated=True)
                result.insert(value_node)

        for value in unique_values:
            bits_required = result._type.bits_required(value=value)
            if bits_required > result._type.width:
                raise InvalidEnumValueError(enumeration_name=result._name, value=value)
        return result

    @property
    def doc(self) -> str:
        assert self._properties is not None
        doc = self._properties.doc
        return str(doc) if doc is not None else ""

    @property
    def type(self) -> BasicType | None:
        return self._type

    @property
    def values(self) -> list[EnumerationValue]:
        return self.children_like(EnumerationValue)
