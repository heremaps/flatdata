from flatdata.generator.tree.nodes.node import Node

class EnumerationValue(Node):
    def __init__(self, name, value, auto_generated, properties=None):
        super().__init__(name=name, properties=properties)
        self._value = value
        self._auto_generated = auto_generated

    @staticmethod
    def create(properties, value):
        result = EnumerationValue(name=properties.name, properties=properties, value=value, auto_generated=False)
        return result

    @property
    def doc(self):
        return self._properties.doc

    @property
    def value(self):
        return self._value

    @property
    def auto_generated(self):
        return self._auto_generated
