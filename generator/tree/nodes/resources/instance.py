from .base import ResourceBase
from generator.tree.nodes.references import StructureReference


class Instance(ResourceBase):
    def __init__(self, name, properties=None, type=None, own_schema=None):
        super(Instance, self).__init__(name=name, properties=properties, own_schema=own_schema)
        self._type = type

    @staticmethod
    def create(properties, own_schema):
        return Instance(name=properties.name,
                        properties=properties,
                        type=properties.type.object.type,
                        own_schema=own_schema)

    def _create_references(self):
        return [StructureReference(name=self._type)]

    @property
    def type(self):
        return self._type

