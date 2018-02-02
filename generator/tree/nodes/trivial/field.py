from generator.tree.nodes.node import Node
from generator.tree.nodes.references import EnumerationReference
from generator.tree.helpers.basictype import BasicType


class Field(Node):
    def __init__(self, name, properties=None, type=None, offset=None, width=None):
        super(Field, self).__init__(name=name, properties=properties)
        self._offset = offset
        self._width = width

        if type is not None:
            if not BasicType.is_basic_type(type):
                self._type = EnumerationReference(type, width=self._width)
                self.insert(self._type)
            else:
                self._type = BasicType(name=type, width=self._width)

    @staticmethod
    def create(properties, offset=None):
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

    @type.setter
    def type(self, value):
        self._type = value

    @property
    def offset(self):
        return self._offset

    @offset.setter
    def offset(self, value):
        self._offset = value
