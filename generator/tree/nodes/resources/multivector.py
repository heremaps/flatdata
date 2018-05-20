from collections import namedtuple

from .base import ResourceBase
from generator.tree.nodes.references import StructureReference
from generator.tree.nodes.trivial import Structure


class Multivector(ResourceBase):
    def __init__(self, name, properties=None, types=None, width=None, own_schema=None):
        super(Multivector, self).__init__(name=name, properties=properties, own_schema=own_schema)
        self._types = []
        if types is not None:
            self._types = types
        self._width = width

    @staticmethod
    def create(properties, own_schema):
        return Multivector(name=properties.name,
                           properties=properties,
                           types=[t for t in properties.type.multivector.type],
                           width=int(properties.type.multivector.width),
                           own_schema=own_schema)

    def _create_references(self):
        return [StructureReference(name=t) for t in self._types]

    @property
    def types(self):
        return self._types

    @property
    def width(self):
        return self._width

    @property
    def builtins(self):
        StructProperties = namedtuple(
            "Properties", ["name", "schema", "doc", "fields", "is_index"])
        FieldProperties = namedtuple("Properties", ["name", "width", "type"])
        properties = StructProperties(
            name="IndexType{width}".format(width=self._width),
            schema="struct IndexType%s { value : u64 : %s; }" % (self._width, self._width),
            doc="/** Builtin type to for MultiVector index */",
            fields=[FieldProperties(name="value", width=self._width, type="u64")],
            is_index=True)
        index_type = Structure.create(properties=properties, own_schema=properties.schema,
                                      definition="")
        return [index_type]
