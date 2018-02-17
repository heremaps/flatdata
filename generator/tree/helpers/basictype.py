from generator import grammar
from generator.tree.errors import InvalidWidthError


class BasicType(object):
    _WIDTH = {
        "bool": 1,
        "u8": 8,
        "i8": 8,
        "u16": 16,
        "i16": 16,
        "u32": 32,
        "i32": 32,
        "u64": 64,
        "i64": 64
    }

    @staticmethod
    def is_basic_type(name):
        return name in grammar.BASIC_TYPES

    def __init__(self, name, width=None):
        assert self.is_basic_type(name)
        self._name = name
        self._width = width
        if width is None:
            self._width = self._WIDTH[self._name]
        if self._width > self._WIDTH[self.name]:
            raise InvalidWidthError(width=self._width, type=self._name)

    @property
    def name(self):
        return self._name

    @property
    def width(self):
        return self._width

    @property
    def is_signed(self):
        return self._name[0] == 'i'
