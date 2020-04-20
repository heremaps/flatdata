from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.references import EnumerationReference, ConstantReference
from flatdata.generator.tree.helpers.basictype import BasicType


class Field(Node):
    def __init__(self, name, properties=None, type=None, offset=None, width=None):
        super().__init__(name=name, properties=properties)
        self._offset = offset
        self._width = width
        self._decorations = list()
        if properties and 'decorations' in properties:
            self._decorations = properties.decorations

        for d in self.decorations:
            if "const_ref" in d:
                self.insert(ConstantReference(d.const_ref.name))

        if type is not None:
            if not BasicType.is_basic_type(type):
                self._type_reference = EnumerationReference(type, width=self._width)
                self._type = None
                self.insert(self._type_reference)
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
    def range(self):
        for d in self.decorations:
            if "range" in d:
                return d.range.name
        return None

    @property
    def const_refs(self):
        return self.children_like(ConstantReference)

    @property
    def decorations(self):
        return self._decorations

    @property
    def type(self):
        return self._type

    @type.setter
    def type(self, value):
        self._type = value

    @property
    def type_reference(self):
        return self._type_reference

    @property
    def offset(self):
        return self._offset

    @offset.setter
    def offset(self, value):
        self._offset = value

    @property
    def doc(self):
        return self._properties.doc
