
from abc import ABC, abstractmethod

from flatdata.generator.tree.nodes.explicit_reference import ExplicitReference
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.references import BuiltinStructureReference, StructureReference


class ResourceBase(Node, ABC):
    def __init__(self, name, properties=None,):
        super(ResourceBase, self).__init__(name=name, properties=properties)
        self._decorations = []
        if properties is not None and 'decorations' in properties:
            self._decorations = properties.decorations
            for decoration in self._decorations:
                if 'explicit_reference' in decoration:
                    self.insert(ExplicitReference.create(properties=decoration.explicit_reference))

    @abstractmethod
    def create_references(self):
        pass

    @property
    def optional(self):
        return any(['optional' in d for d in self.decorations])

    @property
    def doc(self):
        return self._properties.doc

    @property
    def decorations(self):
        return self._decorations

    @property
    def explicit_references(self):
        return self.children_like(ExplicitReference)

    @property
    def referenced_structures(self):
        return self.children_like(BuiltinStructureReference) + self.children_like(StructureReference)
