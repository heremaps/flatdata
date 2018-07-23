from .base import ResourceBase
from generator.tree.nodes.references import StructureReference


class Bitset(ResourceBase):
    def __init__(self, name, properties=None):
        super(Bitset, self).__init__(name=name, properties=properties)

    @staticmethod
    def create(properties):
        return Bitset(name=properties.name,
                      properties=properties)

    def _create_references(self):
        return []
