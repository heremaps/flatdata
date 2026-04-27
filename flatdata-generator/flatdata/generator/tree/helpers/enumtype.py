from .basictype import BasicType


class EnumType:
    def __init__(self, name: str, basic_type: BasicType) -> None:
        assert not BasicType.is_basic_type(name), "%r is no valid enum name" % name
        self._name = name
        self._type = basic_type

    @property
    def name(self) -> str:
        return self._name

    @property
    def width(self) -> int:
        return self._type.width

    @property
    def annotation(self) -> str:
        return self._type.annotation

    @property
    def is_signed(self) -> bool:
        return self._type.is_signed
