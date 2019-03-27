from flatdata.generator import grammar
from flatdata.generator.tree.errors import InvalidWidthError
from flatdata.generator.tree.errors import InvalidSignError


class BasicType:
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
            raise InvalidWidthError(self._width, self._name)

    @property
    def name(self):
        return self._name

    @property
    def width(self):
        return self._width

    @property
    def is_signed(self):
        return self._name[0] == 'i'

    def bits_required(self, value):
        if self.is_signed:
            if value >= 0:
                # sign bit
                return value.bit_length() + 1
            # sign bit plus 2 complement allowes one more value
            return (-value -1).bit_length() + 1
        if value >= 0:
            return value.bit_length()
        raise InvalidSignError(value=value)
