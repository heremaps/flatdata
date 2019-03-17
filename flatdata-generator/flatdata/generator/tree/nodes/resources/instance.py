from flatdata.generator.tree.nodes.references import StructureReference
from .base import ResourceBase


class Instance(ResourceBase):
    def __init__(self, name, properties=None, resource_type=None):
        super(Instance, self).__init__(name=name, properties=properties)
        self._type = resource_type

    @staticmethod
    def create(properties):
        return Instance(name=properties.name,
                        properties=properties,
                        resource_type=properties.type.object.type)

    def create_references(self):
        return [StructureReference(name=self._type)]

    @property
    def type(self):
        return self._type
