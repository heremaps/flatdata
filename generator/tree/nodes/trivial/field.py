from generator.tree.nodes.node import Node
from generator.tree.helpers.basictype import BasicType


class Field(Node):
    def __init__(self, name, properties=None, type=None, offset=None, width=None):
        super(Field, self).__init__(name=name, properties=properties)
        self._type = type
        self._offset = offset
        self._width = width

        if self._type is not None:
            self._type = BasicType(name=self._type, width=self._width)

    @staticmethod
    def create(properties, offset):
        width = None
        if properties.width:
            width = int(properties.width)
        return Field(name=properties.name,
                     properties=properties,
                     type=properties.type,
                     offset=offset,
                     width=width)

    @property
    def type(self):
        return self._type

    @property
    def offset(self):
        return self._offset
