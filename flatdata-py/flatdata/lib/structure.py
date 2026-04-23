from collections import namedtuple
from collections.abc import Callable, Iterator
import json
from typing import Any

import numpy as np

from .data_access import make_field_reader

FieldSignature = namedtuple(
    "FieldSignature", ["offset", "width", "is_signed", "dtype"])


class Structure:
    __slots__ = ('_mem', '_pos')
    _READERS: dict[str, Callable[[Any, int], int]] = {}
    _FIELDS: dict[str, Any]
    _FIELD_KEYS: list[str]
    _SCHEMA: str

    def __init_subclass__(cls, **kwargs: Any) -> None:
        super().__init_subclass__(**kwargs)
        fields = cls.__dict__.get('_FIELDS')
        if fields is not None:
            cls._READERS = {name: make_field_reader(f.offset, f.width, f.is_signed)
                            for name, f in fields.items()}

    def __init__(self, mem: Any, pos: int) -> None:
        self._mem = mem
        self._pos = pos

    def __getattr__(self, name: str) -> int:
        try:
            reader = self._READERS[name]
        except KeyError:
            raise AttributeError("Field %s not found in structure" % name)
        return reader(self._mem, self._pos)

    def __dir__(self) -> list[str]:
        return self._FIELD_KEYS

    def __iter__(self) -> Iterator[int]:
        for name in self._FIELD_KEYS:
            yield getattr(self, name)

    def as_dict(self) -> dict[str, int]:
        mem, pos = self._mem, self._pos
        return {name: reader(mem, pos) for name, reader in self._READERS.items()}

    def as_list(self) -> list[int]:
        mem, pos = self._mem, self._pos
        return [reader(mem, pos) for reader in self._READERS.values()]

    def as_tuple(self) -> tuple[int, ...]:
        mem, pos = self._mem, self._pos
        return tuple(reader(mem, pos) for reader in self._READERS.values())

    @classmethod
    def dtype(cls) -> list[tuple[str, np.dtype[Any]]]:
        return [(name, np.dtype(field.dtype)) for name, field in cls._FIELDS.items()]

    def as_nparray(self) -> np.ndarray[Any, Any]:
        mem, pos = self._mem, self._pos
        return np.array([tuple(reader(mem, pos) for reader in self._READERS.values())],
                        dtype=self.dtype())

    def schema(self) -> str:
        return self._SCHEMA

    @classmethod
    def _repr_attributes(cls) -> dict[str, Any]:
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
    def __repr__(cls) -> str:
        return json.dumps(cls._repr_attributes())

    def __repr__(self) -> str:  # type: ignore[no-redef]  # intentional: instance __repr__ shadows classmethod __repr__ above
        return json.dumps({
            "name": self.__class__.__name__,
            "attributes":
                {name: getattr(self, name)
                 for name, signature in self._FIELDS.items()}
        }, indent=4)
