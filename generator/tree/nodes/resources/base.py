from generator.tree.nodes.explicit_reference import ExplicitReference
from generator.tree.nodes.node import Node
from generator.tree.nodes.references import BuiltinStructureReference, StructureReference


class ResourceBase(Node):
    def __init__(self, name, properties=None,):
        super(ResourceBase, self).__init__(name=name, properties=properties)
        self._decorations = []
        if properties is not None and 'decorations' in properties:
            self._decorations = properties.decorations
            for d in self._decorations:
                if 'explicit_reference' in d:
                    self.insert(ExplicitReference.create(properties=d.explicit_reference))

    def create_references(self):
        result = []
        result.extend(self._create_references())
        return result

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
