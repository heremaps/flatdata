from flatdata.generator.tree.errors import DuplicateEnumValueError, InvalidEnumValueError
from flatdata.generator.tree.helpers.basictype import BasicType
from flatdata.generator.tree.nodes.node import Node
from .enumeration_value import EnumerationValue


class Enumeration(Node):
    def __init__(self, name, properties=None, type=None):
        super(Enumeration, self).__init__(name=name, properties=properties)
        self._type = type

        if self._type is not None:
            self._type = BasicType(name=self._type)

    @staticmethod
    def create(properties, definition):
        result = Enumeration(name=properties.name, properties=properties, type=properties.type)

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

        bits_required = 0
        for value in unique_values:
            bits_required = max(bits_required, result.type.bits_required(value=value))
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

    @property
    def bits_required(self):
        return self._bits_required
