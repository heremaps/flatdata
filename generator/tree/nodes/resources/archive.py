from .base import ResourceBase
from generator.tree.nodes.references import ArchiveReference


class Archive(ResourceBase):
    def __init__(self, name, properties=None, target=None, own_schema=None):
        super(Archive, self).__init__(name=name, properties=properties, own_schema=own_schema)
        self._target = target

    @staticmethod
    def create(properties, own_schema):
        return Archive(name=properties.name,
                       properties=properties,
                       target=properties.type.archive.name,
                       own_schema=own_schema)

    @property
    def target(self):
        targets = self.children_like(ArchiveReference)
        assert len(targets) == 1
        return targets[0]

    def _create_references(self):
        return [ArchiveReference(name=self._target)]
