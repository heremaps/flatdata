from flatdata.generator.tree.nodes.references import ResourceReference
from .base import ResourceBase


class BoundResource(ResourceBase):
    def __init__(self, name, properties=None, resources=None):
        super(BoundResource, self).__init__(name=name, properties=properties)
        self._resources = resources

    @staticmethod
    def create(properties):
        return BoundResource(name=properties.name,
                             properties=properties,
                             resources=[r for r in properties.resources])

    def create_references(self):
        return [ResourceReference(name=r) for r in self._resources]

    @property
    def referenced_structures(self):
        return [s for r in self.children_like(ResourceReference) for s in
                r.node.referenced_structures]
