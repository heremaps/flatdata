from .node import Node

class Reference(Node):
    """
    References depict:
    - Resource -> Structure references
    - Resource -> Archive references

    References participate in cycle resolution.
    References participate in dependency resolution.
    """

    def __init__(self, name: str) -> None:
        super().__init__(name=Reference._referencify(name))

    @property
    def target(self) -> str:
        return Reference._dereferencify(self.name)

    def update_reference(self, new_value: str) -> None:
        assert new_value.endswith(self.target), \
            "References can only be updated during resolution for the same symbol: %s -> %s" % \
            (self.target, new_value)
        self.set_name(Reference._referencify(new_value))

    @property
    def node(self) -> Node:
        return self.root.find(self.target)

    @property
    def is_qualified(self) -> bool:
        return self.name[:2] == "@@"

    @staticmethod
    def _referencify(name: str) -> str:
        return "@" + name.replace(".", "@")

    @staticmethod
    def _dereferencify(name: str) -> str:
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

class ConstantValueReference(ConstantReference):
    pass

class InvalidValueReference(ConstantReference):
    pass

class EnumerationReference(TypeReference):
    """
    EnumerationReference depict:
    - Field Type -> Enumeration
    """
    def __init__(self, name: str, width: int | None = None) -> None:
        super().__init__(name)
        self._width = width

    @property
    def width(self) -> int | None:
        return self._width
