from .data_access import read_value
from collections import namedtuple
import json

FieldSignature = namedtuple("FieldSignature", ["offset", "width", "is_signed"])


class Structure(object):
    def __init__(self, mem, pos):
        self._mem = mem
        self._pos = pos

    def __getattr__(self, name):
        try:
            field = self._FIELDS[name]
        except KeyError:
            raise AttributeError("Field %s not found in structure" % name)
        return read_value(self._mem, field.offset, field.width, field.is_signed)

    def __dir__(self):
        return self._FIELD_KEYS

    def __iter__(self):
        for name in self._FIELD_KEYS:
            yield getattr(self, name)

    def as_dict(self):
        return dict([(name, getattr(self, name)) for name in self._FIELD_KEYS])

    def as_list(self):
        return [getattr(self, name) for name in self._FIELD_KEYS]

    def schema(self):
        return self._SCHEMA

    @classmethod
    def _repr_attributes(cls):
        return {
            "name": cls.__name__,
            "attributes": [
                {
                    "name": name,
                    "offset": signature.offset,
                    "width": signature.width,
                    "is_signed": signature.is_signed
                }
                for name, signature in cls._FIELDS.items()]
        }

    @classmethod
    def __repr__(cls):
        return json.dumps(cls._repr_attributes())

    def __repr__(self):
        return json.dumps({
            "name": self.__class__.__name__,
            "attributes":
                dict([(name, getattr(self, name)) for name, signature in self._FIELDS.items()])
        }, indent=4)
