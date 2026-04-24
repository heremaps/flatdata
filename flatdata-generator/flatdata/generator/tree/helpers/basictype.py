from flatdata.generator import grammar
from flatdata.generator.tree.errors import InvalidWidthError
from flatdata.generator.tree.errors import InvalidSignError


class BasicType:
    _WIDTH: dict[str, int] = {
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

    _TYPE_ANNOTATION: dict[str, str] = {
        "bool": "",
        "u8": "",
        "i8": "",
        "u16": "",
        "i16": "",
        "u32": "UL",
        "i32": "L",
        "u64": "ULL",
        "i64": "LL"
    }

    @staticmethod
    def is_basic_type(name: str) -> bool:
        return name in grammar.BASIC_TYPES

    def __init__(self, name: str, width: int | None = None) -> None:
        assert self.is_basic_type(name)
        self._name = name
        self._width: int = width if width is not None else self._WIDTH[self._name]
        if self._width > self._WIDTH[self.name]:
            raise InvalidWidthError(self._width, self._name)

    @property
    def name(self) -> str:
        return self._name

    @property
    def width(self) -> int:
        return self._width

    @property
    def is_signed(self) -> bool:
        return self._name[0] == 'i'

    @property
    def annotation(self) -> str:
        return self._TYPE_ANNOTATION[self._name]

    def bits_required(self, value: int) -> int:
        if self.is_signed:
            if value >= 0:
                # sign bit
                return value.bit_length() + 1
            # sign bit plus 2 complement allowes one more value
            return (-value -1).bit_length() + 1
        if value >= 0:
            return value.bit_length()
        raise InvalidSignError(value=value)

    def value_range(self) -> range:
        if self.is_signed:
            return range(-(2 ** (self.width - 1)), 2 ** (self.width - 1))
        return range(2 ** self.width)
