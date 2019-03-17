from .node import Node

class Reference(Node):
    """
    References depict:
    - Resource -> Structure references
    - Resource -> Archive references

    References participate in cycle resolution.
    References participate in dependency resolution.
    """

    def __init__(self, name):
        super(Reference, self).__init__(name=Reference._referencify(name))

    @property
    def target(self):
        return Reference._dereferencify(self.name)

    def update_reference(self, new_value):
        assert new_value.endswith(self.target), \
            "References can only be updated during resolution for the same symbol: %s -> %s" % \
            (self.target, new_value)
        self.set_name(Reference._referencify(new_value))

    @property
    def node(self):
        return self.root.find(self.target)

    @property
    def is_qualified(self):
        return self.name[:2] == "@@"

    @staticmethod
    def _referencify(name):
        return "@" + name.replace(".", "@")

    @staticmethod
    def _dereferencify(name):
        return name[1:].replace("@", ".")


class RuntimeReference(Reference):
    """
    Runtime References depict:
    - Explicit references

    Runtime References are excluded from cycle resolution
    """


class ResourceReference(RuntimeReference):
    pass


class VectorReference(ResourceReference):
    pass


class FieldReference(RuntimeReference):
    pass


class TypeReference(Reference):
    pass


class StructureReference(TypeReference):
    pass


class BuiltinStructureReference(TypeReference):
    pass


class ArchiveReference(TypeReference):
    pass


class ConstantReference(TypeReference):
    pass

class EnumerationReference(TypeReference):
    """
    EnumerationReference depict:
    - Field Type -> Enumeration
    """
    def __init__(self, name, width=None):
        super(Reference, self).__init__(name=Reference._referencify(name))
        self._width = width

    @property
    def width(self):
        return self._width
