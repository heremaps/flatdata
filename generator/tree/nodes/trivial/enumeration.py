from generator.tree.nodes.node import Node
from .enumeration_value import EnumerationValue
from generator.tree.helpers.basictype import BasicType

class Enumeration(Node):
    def __init__(self, name, properties=None, own_schema=None, type=None):
        super(Enumeration, self).__init__(name=name, properties=properties)
        self._own_schema = own_schema
        self._type=type

        if self._type is not None:
            self._type = BasicType(name=self._type)

    @staticmethod
    def create(properties, own_schema, definition):
        result = Enumeration(name=properties.name, properties=properties, own_schema=own_schema, type=properties.type)

        current_assigned_value = 0
        for value in properties.enum_values:
            if value.constant:
                current_assigned_value = int(value.constant)
            value_node = EnumerationValue.create(properties=value, value=current_assigned_value)
            result.insert(value_node)
            current_assigned_value += 1

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
