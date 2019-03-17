from flatdata.generator.tree.nodes.references import ArchiveReference
from .base import ResourceBase


class Archive(ResourceBase):
    def __init__(self, name, properties=None, target=None):
        super(Archive, self).__init__(name=name, properties=properties)
        self._target = target

    @staticmethod
    def create(properties):
        return Archive(name=properties.name,
                       properties=properties,
                       target=properties.type.archive.name)

    @property
    def target(self):
        targets = self.children_like(ArchiveReference)
        assert len(targets) == 1
        return targets[0]

    def create_references(self):
        return [ArchiveReference(name=self._target)]
