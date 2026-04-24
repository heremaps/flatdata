'''
 Copyright (c) 2023 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from collections.abc import Iterator
import json
from typing import Any

import pandas as pd
import numpy as np

from .data_access import read_value, read_field_vectorized
from .errors import CorruptResourceError

SIZE_OFFSET_IN_BITS = 64
SIZE_OFFSET_IN_BYTES = SIZE_OFFSET_IN_BITS // 8
SIZE_PADDING_IN_BYTES = 8


class ResourceBase:
    def __init__(self, mem: Any, element_type: type[Any]) -> None:
        if len(mem) < (SIZE_OFFSET_IN_BYTES + SIZE_PADDING_IN_BYTES):
            raise CorruptResourceError()
        self._mem = memoryview(mem)
        self._element_type = element_type
        self._element_types = [element_type]
        self._type_size_in_bytes = self._element_type._SIZE_IN_BYTES if self._element_type else 1
        self._raw_numpy_2d: np.ndarray[Any, np.dtype[np.uint8]] | None = None

    def size_in_bytes(self) -> int:
        return len(self._mem)

    def _item_offset(self, index: int) -> int:
        return int(SIZE_OFFSET_IN_BYTES + self._element_type._SIZE_IN_BYTES * index)

    def _get_item(self, index: int) -> Any:
        offset = self._item_offset(index)
        return self._element_type(self._mem, offset)

    def _as_numpy_2d(self) -> Any:
        """Return the raw data as a 2D numpy uint8 array of shape (n, struct_size).
        Zero-copy via np.frombuffer on the mmap'd memory. Cached after first call.
        """
        if self._raw_numpy_2d is None:
            n = len(self)
            struct_size = self._type_size_in_bytes
            raw = np.frombuffer(
                self._mem[SIZE_OFFSET_IN_BYTES:SIZE_OFFSET_IN_BYTES + n * struct_size],
                dtype=np.uint8,
            )
            self._raw_numpy_2d = raw.reshape(n, struct_size)
        return self._raw_numpy_2d

    def _repr_attributes(self) -> dict[str, Any]:
        return {
            "container_type": self.__class__.__name__,
            "size": len(self),
            "size_in_bytes": self.size_in_bytes(),
            "element_types": [t._repr_attributes() for t in self._element_types if t is not None]
        }

    def __len__(self) -> int:
        raise NotImplementedError()

    def __repr__(self) -> str:
        return json.dumps(self._repr_attributes(), indent=4)

    @classmethod
    def open(cls, storage: Any, name: str, initializer: Any, is_optional: bool = False) -> Any:
        return cls(storage.get(name, is_optional), initializer)


class _VectorSlice:
    def __init__(self, s: slice, sequence: 'Vector') -> None:
        self._slice = s
        self._sequence = sequence

    def to_numpy(self, limit: int | None = None) -> Any:
        raw_2d = self._sequence._as_numpy_2d()
        sliced = raw_2d[self._slice]
        if limit is not None:
            sliced = sliced[:limit]

        fields = self._sequence._element_type._FIELDS
        dtype = self._sequence._element_type.dtype()
        result = np.empty(sliced.shape[0], dtype=dtype)
        for name, field in fields.items():
            result[name] = read_field_vectorized(
                sliced, field.offset, field.width, field.is_signed
            )
        return result

    def to_data_frame(self, limit: int | None = None) -> pd.DataFrame:
        return pd.DataFrame(data=self.to_numpy(limit))

    def __iter__(self) -> Iterator[Any]:
        for i in range(*self._slice.indices(len(self._sequence))):
            yield self._sequence[i]

    def __getattr__(self, name: str) -> pd.DataFrame:
        try:
            field = self._sequence._element_type._FIELDS[name]
        except KeyError:
            raise AttributeError("Field %s not found in structure" % name)
        raw_2d = self._sequence._as_numpy_2d()[self._slice]
        values = read_field_vectorized(raw_2d, field.offset, field.width, field.is_signed)
        return pd.DataFrame(data=values, columns=[name])

    def __repr__(self) -> str:
        return "Displaying first 100 records:\n" + repr(self.to_data_frame(limit=100))


class Vector(ResourceBase):
    def __init__(self, mem: Any, element_type: type[Any]) -> None:
        ResourceBase.__init__(self, mem, element_type)
        size_in_bytes = read_value(self._mem, 0, SIZE_OFFSET_IN_BITS, False)
        size, rem = divmod(size_in_bytes, self._type_size_in_bytes)
        assert rem == 0, "Malformed vector"
        self._size = size

    def to_numpy(self) -> Any:
        """Convert entire vector to a numpy structured array (vectorized)."""
        raw_2d = self._as_numpy_2d()
        fields = self._element_type._FIELDS
        dtype = self._element_type.dtype()
        result = np.empty(self._size, dtype=dtype)
        for name, field in fields.items():
            result[name] = read_field_vectorized(
                raw_2d, field.offset, field.width, field.is_signed
            )
        return result

    def to_data_frame(self) -> pd.DataFrame:
        return pd.DataFrame(data=self.to_numpy())

    def __getitem__(self, index: int | slice) -> Any:
        if isinstance(index, slice):
            return _VectorSlice(index, self)

        if index < 0:
            index += len(self)
        if index >= self._size or index < 0:
            raise IndexError("Vector access out of bounds: " + str(index))
        return self._get_item(index)

    def __iter__(self) -> Iterator[Any]:
        mem = self._mem
        element_type = self._element_type
        size_bytes = self._type_size_in_bytes
        for i in range(self._size):
            yield element_type(mem, SIZE_OFFSET_IN_BYTES + size_bytes * i)

    def __getattr__(self, name: str) -> pd.DataFrame:
        try:
            field = self._element_type._FIELDS[name]
        except KeyError:
            raise AttributeError("Field %s not found in structure" % name)
        raw_2d = self._as_numpy_2d()
        values = read_field_vectorized(raw_2d, field.offset, field.width, field.is_signed)
        return pd.DataFrame(data=values, columns=[name])

    def __len__(self) -> int:
        return self._size


class _MultivectorSlice:
    def __init__(self, s: slice, sequence: 'Multivector') -> None:
        self._slice = s
        self._sequence = sequence

    def __iter__(self) -> Iterator[list[Any]]:
        for i in range(*self._slice.indices(len(self._sequence))):
            yield self._sequence[i]

    def __repr__(self) -> str:
        return [x for x in self].__repr__()


class Multivector(ResourceBase):
    def __init__(self, index_mem: Any, mem: Any, index_type: type[Any], *element_types: type[Any]) -> None:
        self._index = Vector(index_mem, index_type)
        self._mem = mem
        self._element_types = list(element_types)
        self._index_type = index_type

    @classmethod
    def open(cls, storage: Any, name: str, initializer: Any, is_optional: bool = False) -> 'Multivector':
        return cls(storage.get(name + "_index", is_optional),
                   storage.get(name, is_optional),
                   *initializer)

    def __len__(self) -> int:
        # The last entry is just a sentinel
        return max(0, len(self._index) - 1)

    def _bucket_offset(self, index: int) -> int:
        return int(self._index[index].value) + SIZE_OFFSET_IN_BYTES

    def __getitem__(self, index: int | slice) -> Any:
        if isinstance(index, slice):
            return _MultivectorSlice(index, self)

        offset = self._bucket_offset(index)
        next_offset = self._bucket_offset(index + 1)
        elements = []
        while offset < next_offset:
            type_index = read_value(self._mem, offset * 8, 8, False)
            offset += 1
            element_type = self._element_types[type_index]
            element = element_type(self._mem, offset)
            elements.append(element)
            offset += element_type._SIZE_IN_BYTES
        return elements

    def __iter__(self) -> Iterator[list[Any]]:
        for i in range(len(self)):
            yield self[i]

    def __repr__(self) -> str:
        attrs = self._repr_attributes()
        attrs.update(index_type=self._index_type._repr_attributes())
        return json.dumps(attrs, indent=4)


class RawData(ResourceBase):
    def __len__(self) -> int:
        return read_value(self._mem, 0, SIZE_OFFSET_IN_BITS, False)

    def __getitem__(self, item: int | slice) -> memoryview:
        if isinstance(item, slice):
            return self._mem[
                slice(item.start + SIZE_OFFSET_IN_BYTES,
                      (item.stop + SIZE_OFFSET_IN_BYTES) if item.stop is not None else None,
                      item.step)
            ]
        return self._mem[item + SIZE_OFFSET_IN_BYTES:item + SIZE_OFFSET_IN_BYTES + 1]

    def sub_str(self, index: int, separator: bytes = b'\0') -> str:
        for i in range(index, len(self)):
            if self[i:i + len(separator)] == separator:
                return bytes(self[index:i]).decode("utf-8")
        return bytes(self[index]).decode("utf-8")

    def sub_str_list(self, index: int, separator: bytes = b'\0', list_separator: bytes = b'\0\0') -> list[str]:
        result = []
        for i in range(index, len(self)):
            if index == i and self[i:i + len(list_separator)] == list_separator:
                break
            if self[i:i + len(separator)] == separator:
                result.append(bytes(self[index:i]).decode("utf-8"))
                index = i + 1
        return result

    def sub_str_array(self, index: int, size: int, separator: bytes = b'\0') -> list[str]:
        result = []
        for i in range(index, len(self)):
            if self[i:i + len(separator)] == separator:
                result.append(bytes(self[index:i]).decode("utf-8"))
                index = i + 1
                if len(result) == size:
                    break
        return result


class Instance(ResourceBase):
    def __getitem__(self, index: int | slice) -> Any:
        if isinstance(index, slice):
            raise IndexError("Instance has only one item")

        if index < 0:
            index += 1
        if index >= 1 or index < 0:
            raise IndexError("Instance access out of bounds")

        return self._get_item(index)

    def __iter__(self) -> Iterator[Any]:
        for i in range(1):
            yield self._get_item(i)

    def __getattr__(self, name: str) -> Any:
        offset = self._item_offset(0)
        return getattr(self._element_type(self._mem, offset), name)

    def __len__(self) -> int:
        return 1
