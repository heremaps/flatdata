from generator.tree.nodes.node import Node

class EnumerationValue(Node):
    def __init__(self, name, value, properties=None, own_schema=None):
        super(EnumerationValue, self).__init__(name=name, properties=properties)
        self._own_schema = own_schema
        self._value = value

    @staticmethod
    def create(properties, value):
        result = EnumerationValue(name=properties.name, properties=properties, value = value)
        return result

    @property
    def doc(self):
        return self._properties.doc

    @property
    def value(self):
        return self._value
