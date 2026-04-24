from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.references import BuiltinStructureReference, ResourceReference, StructureReference
from .base import ResourceBase

from pyparsing import ParseResults


class BoundResource(ResourceBase):
    def __init__(self, name: str, properties: ParseResults | None = None, resources: list[str] | None = None) -> None:
        super().__init__(name=name, properties=properties)
        self._resources = resources

    @staticmethod
    def create(properties: ParseResults) -> 'BoundResource':
        return BoundResource(name=properties.name,
                             properties=properties,
                             resources=[r for r in properties.resources])

    def create_references(self) -> list[Node]:
        assert self._resources is not None
        return [ResourceReference(name=r) for r in self._resources]

    @property
    def referenced_structures(self) -> list[BuiltinStructureReference | StructureReference]:
        return [s for r in self.children_like(ResourceReference) for s in
                r.node.referenced_structures]  # type: ignore[attr-defined]  # .node resolves to a resource type with referenced_structures
