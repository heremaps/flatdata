from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.references import ArchiveReference
from .base import ResourceBase

from typing import Any


class Archive(ResourceBase):
    def __init__(self, name: str, properties: Any = None, target: str | None = None) -> None:
        super().__init__(name=name, properties=properties)
        self._target = target

    @staticmethod
    def create(properties: Any) -> 'Archive':
        return Archive(name=properties.name,
                       properties=properties,
                       target=properties.type.archive.name)

    @property
    def target(self) -> ArchiveReference:
        targets = self.children_like(ArchiveReference)
        assert len(targets) == 1
        return targets[0]

    def create_references(self) -> list[Node]:
        assert self._target is not None
        return [ArchiveReference(name=self._target)]
