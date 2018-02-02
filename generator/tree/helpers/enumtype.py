from generator import grammar
from generator.tree.errors import InvalidWidthError

from .basictype import BasicType


class EnumType(object):
    def __init__(self, name, basic_type):
        assert name not in grammar.BASIC_TYPES
        self._name = name
        self._type = basic_type;

    @property
    def name(self):
        return self._name

    @property
    def width(self):
        return self._type.width

    @property
    def is_signed(self):
        return self._type.is_signed
