from generator.tree.nodes.node import Node
from .field import Field


class Structure(Node):
    def __init__(self, name, properties=None, own_schema=None, size_in_bits=None):
        """
        Use to instantiate empty structure.
        No special properties are evaluated.

        :param name: name
        :param properties: properties. can be missing.
        """
        super(Structure, self).__init__(name=name, properties=properties)
        self._own_schema = own_schema
        self._size_in_bits = size_in_bits

    @staticmethod
    def create(properties, own_schema, definition):
        result = Structure(name=properties.name, properties=properties, own_schema=own_schema)

        for field in properties.fields:
            result.insert(Field.create(properties=field))
        return result

    @property
    def doc(self):
        return self._properties.doc

    @property
    def size_in_bits(self):
        return self._size_in_bits

    @size_in_bits.setter
    def size_in_bits(self, value):
        self._size_in_bits = value

    @property
    def size_in_bytes(self):
        return (self._size_in_bits + 7) // 8

    @property
    def fields(self):
        return self.children_like(Field)
