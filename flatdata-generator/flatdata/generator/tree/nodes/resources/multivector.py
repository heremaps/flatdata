from collections import namedtuple

from flatdata.generator.tree.nodes.references import StructureReference, BuiltinStructureReference
from flatdata.generator.tree.nodes.trivial import Structure
from .base import ResourceBase


class Multivector(ResourceBase):
    def __init__(self, name, properties=None, types=None, width=None):
        super(Multivector, self).__init__(name=name, properties=properties)
        self._types = []
        if types is not None:
            self._types = types
        self._width = width

    @staticmethod
    def create(properties):
        return Multivector(name=properties.name,
                           properties=properties,
                           types=[t for t in properties.type.multivector.type],
                           width=int(properties.type.multivector.width))

    def create_references(self):
        return [StructureReference(name=t) for t in self._types]

    @property
    def types(self):
        return self._types

    @property
    def width(self):
        return self._width

    @property
    def index_reference(self):
        builtin_refs = [node for node in self.children if isinstance(node, BuiltinStructureReference)]
        assert len(builtin_refs) == 1, "multivector has exactly one builtin ref which is its index"
        return builtin_refs[0]

    @property
    def builtins(self):
        class MemberDict(dict):
            def __getattr__(self, attr):
                return self.get(attr)
        decorations = [MemberDict({"range" : MemberDict({"name":"range"})})]
        field = MemberDict({"decorations":decorations, "name":"value", "width":self._width, "type":"u64"})
        properties = MemberDict({
            "name":"IndexType{width}".format(width=self._width),
            "schema":"struct IndexType%s { value : u64 : %s; }" % (self._width, self._width),
            "doc":"/** Builtin type to for MultiVector index */",
            "fields":[field]})
        index_type = Structure.create(properties=properties, definition="")
        return [index_type]
