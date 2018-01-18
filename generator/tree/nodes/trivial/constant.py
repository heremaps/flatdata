from generator.tree.nodes.node import Node
from generator.tree.helpers.basictype import BasicType


class Constant(Node):
    def __init__(self, name, properties=None, own_schema=None):
        super(Constant, self).__init__(name=name, properties=properties)
        self._own_schema = own_schema

    @staticmethod
    def create(properties, own_schema, definition):
        result = Constant(name=properties.name, properties=properties, own_schema=own_schema)
        return result

    @property
    def type(self):
        return BasicType(self._properties.type)

    @property
    def value(self):
        return self._properties.value
