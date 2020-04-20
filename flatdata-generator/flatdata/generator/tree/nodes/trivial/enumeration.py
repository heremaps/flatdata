from flatdata.generator.tree.errors import DuplicateEnumValueError, InvalidEnumValueError, SparseEnumError
from flatdata.generator.tree.helpers.basictype import BasicType
from flatdata.generator.tree.nodes.node import Node
from .enumeration_value import EnumerationValue


class Enumeration(Node):
    def __init__(self, name, properties=None, type=None, width=None):
        super().__init__(name=name, properties=properties)
        self._type = type

        if self._type is not None:
            self._type = BasicType(name=self._type, width=width)

    @staticmethod
    def create(properties, definition):
        width = None
        if properties.width:
            width = int(properties.width)
        result = Enumeration(name=properties.name, properties=properties, type=properties.type, width=width)

        current_assigned_value = 0
        unique_values = set()
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
        if len(properties.enum_values) * 2 + 256 < 2 ** result.type.width:
            raise SparseEnumError(enumeration_name=result._name, width=result.type.width)

        for missing_value in result.type.value_range():
            if not missing_value in unique_values:
                value_node = EnumerationValue(name="UNKNOWN_VALUE_" + str(missing_value).replace("-", "MINUS_"), value=missing_value, auto_generated=True)
                result.insert(value_node)

        for value in unique_values:
            bits_required = result.type.bits_required(value=value)
            if bits_required > result.type.width:
                raise InvalidEnumValueError(enumeration_name=result._name, value=value)
        result._bits_required = bits_required

        return result

    @property
    def doc(self):
        return self._properties.doc

    @property
    def type(self):
        return self._type

    @property
    def values(self):
        return self.children_like(EnumerationValue)
