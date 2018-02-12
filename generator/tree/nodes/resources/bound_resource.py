from .base import ResourceBase
from generator.tree.nodes.references import ResourceReference


class BoundResource(ResourceBase):
    def __init__(self, name, properties=None, resources=None, own_schema=None):
        super(BoundResource, self).__init__(name=name, properties=properties, own_schema=own_schema)
        self._resources = resources

    @staticmethod
    def create(properties, own_schema):
        return BoundResource(name=properties.name,
                             properties=properties,
                             resources=[r for r in properties.resources],
                             own_schema=own_schema)

    def _create_references(self):
        return [ResourceReference(name=r) for r in self._resources]

    @property
    def referenced_structures(self):
        return [s for r in self.children_like(ResourceReference) for s in
                r.node.referenced_structures]
