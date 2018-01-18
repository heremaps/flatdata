from .base import ResourceBase
from generator.tree.nodes.references import StructureReference


class Vector(ResourceBase):
    def __init__(self, name, properties=None, type=None, own_schema=None):
        super(Vector, self).__init__(name=name, properties=properties, own_schema=own_schema)
        self._type = type

    @staticmethod
    def create(properties, own_schema):
        return Vector(name=properties.name,
                      properties=properties,
                      type=properties.type.vector.type,
                      own_schema=own_schema)

    def _create_references(self):
        return [StructureReference(name=self._type)]
