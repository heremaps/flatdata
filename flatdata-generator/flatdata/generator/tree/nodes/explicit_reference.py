from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.references import ResourceReference, FieldReference, StructureReference

from pyparsing import ParseResults


class ExplicitReference(Node):
    def __init__(self, name: str, properties: ParseResults | None = None) -> None:
        super().__init__(name=name, properties=properties)

    @staticmethod
    def create(properties: ParseResults) -> 'ExplicitReference':
        destination = properties.destination
        field = Node.jointwo(properties.source_type, properties.source_field)
        result = ExplicitReference(
            name="er_{field}_{destination}".format(field=field.replace(Node.PATH_SEPARATOR, '_'),
                                                   destination=destination.replace(
                                                       Node.PATH_SEPARATOR, '_')),
            properties=properties)
        result.insert(ResourceReference(name=destination))
        result.insert(FieldReference(name=field))
        result.insert(StructureReference(name=properties.source_type))
        return result


    @property
    def destination(self) -> ResourceReference:
        result = self.children_like(ResourceReference)
        assert len(result) == 1
        return result[0]

    @property
    def field(self) -> FieldReference:
        result = self.children_like(FieldReference)
        assert len(result) == 1
        return result[0]

    @property
    def structure(self) -> StructureReference:
        result = self.children_like(StructureReference)
        assert len(result) == 1
        return result[0]
