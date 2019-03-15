from .basictype import BasicType


class EnumType:
    def __init__(self, name, basic_type):
        assert not BasicType.is_basic_type(name), "%r is no valid enum name" % name
        self._name = name
        self._type = basic_type

    @property
    def name(self):
        return self._name

    @property
    def width(self):
        return self._type.width

    @property
    def is_signed(self):
        return self._type.is_signed
