from .base import ResourceBase
from generator.tree.nodes.references import StructureReference


class Instance(ResourceBase):
    def __init__(self, name, properties=None, type=None):
        super(Instance, self).__init__(name=name, properties=properties)
        self._type = type

    @staticmethod
    def create(properties):
        return Instance(name=properties.name,
                        properties=properties,
                        type=properties.type.object.type)

    def _create_references(self):
        return [StructureReference(name=self._type)]

    @property
    def type(self):
        return self._type

