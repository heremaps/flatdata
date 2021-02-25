from collections import namedtuple
import json
import numpy as np

from .data_access import read_value, write_value

FieldSignature = namedtuple(
    "FieldSignature", ["offset", "width", "is_signed", "dtype"])


class Structure:
    def __init__(self, mem, pos):
        self._mem = mem
        self._pos = pos

    def __getattr__(self, name):
        try:
            field = self._FIELDS[name]
        except KeyError:
            raise AttributeError("Field %s not found in structure" % name)
        return self._get_value(field)

    def _get_value(self, field):
        return read_value(self._mem, self._pos * 8 + field.offset, field.width, field.is_signed)

    #def __setattr__(self, name, value):
    #    try:
    #        field = self._FIELDS[name]
    #    except KeyError:
    #        raise AttributeError("Field %s not found in structure" % name)
    #    return self._set_value(field, value)

    #def _set_value(self, field, value):
    #    write_value(self._mem, self._pos * 8 + field.offset, field.width, field.is_signed, value)

    def __dir__(self):
        return self._FIELD_KEYS

    def __iter__(self):
        for name in self._FIELD_KEYS:
            yield getattr(self, name)

    def as_dict(self):
        return {name: self._get_value(field) for name, field in self._FIELDS.items()}

    def as_list(self):
        return [self._get_value(field) for field in self._FIELDS.values()]

    def as_tuple(self):
        return tuple(self._get_value(field) for field in self._FIELDS.values())

    @classmethod
    def dtype(cls):
        return [(name, np.dtype(field.dtype)) for name, field in cls._FIELDS.items()]

    def as_nparray(self):
        return np.array([tuple(self._get_value(field) for name, field in self._FIELDS.items())],
                        dtype=self.dtype())

    def schema(self):
        return self._SCHEMA

    @classmethod
    def _repr_attributes(cls):
        return {
            "name": cls.__name__,
            "doc": cls.__doc__,
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
                {name: getattr(self, name)
                 for name, signature in self._FIELDS.items()}
        }, indent=4)
