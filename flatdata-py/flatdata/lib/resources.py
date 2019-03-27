'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import json

import pandas as pd
import numpy as np

from .data_access import read_value
from .errors import CorruptResourceError

SIZE_OFFSET_IN_BITS = 64
SIZE_OFFSET_IN_BYTES = SIZE_OFFSET_IN_BITS // 8
SIZE_PADDING_IN_BYTES = 8


class ResourceBase:
    def __init__(self, mem, element_type):
        if len(mem) < (SIZE_OFFSET_IN_BYTES + SIZE_PADDING_IN_BYTES):
            raise CorruptResourceError()
        self._mem = mem
        self._element_type = element_type
        self._element_types = [element_type]
        self._type_size_in_bytes = self._element_type._SIZE_IN_BYTES if self._element_type else 1

    def size_in_bytes(self):
        return len(self._mem)

    def _item_offset(self, index):
        return SIZE_OFFSET_IN_BYTES + self._element_type._SIZE_IN_BYTES * index

    def _get_item(self, index):
        offset = self._item_offset(index)
        return self._element_type(self._mem, offset)

    def _repr_attributes(self):
        return {
            "container_type": self.__class__.__name__,
            "size": len(self),
            "size_in_bytes": self.size_in_bytes(),
            "element_types": [t._repr_attributes() for t in self._element_types if t is not None]
        }

    def __len__(self):
        raise NotImplementedError()

    def __repr__(self):
        return json.dumps(self._repr_attributes(), indent=4)

    @classmethod
    def open(cls, storage, name, initializer, is_optional=False):
        return cls(storage.get(name, is_optional), initializer)


class _VectorSlice:
    def __init__(self, s, sequence):
        self._slice = s
        self._sequence = sequence

    def to_numpy(self, limit=None):
        indices = self._slice.indices(len(self._sequence))
        num_items = len(range(*indices)) if not limit else limit
        result = np.empty(
            shape=num_items,
            dtype=self._sequence._element_type.dtype()
        )
        for index, item in self:
            result[index] = item.as_tuple()
        return result

    def to_data_frame(self, limit=None):
        return pd.DataFrame(data=self.to_numpy(limit))

    def __iter__(self):
        for i in range(*self._slice.indices(len(self._sequence))):
            yield self._sequence[i]

    def __getattr__(self, name):
        return pd.DataFrame(data=[[getattr(item, name)] for item in self], columns=[name])

    def __repr__(self):
        return "Displaying first 100 records:\n" + self.to_data_frame(limit=100).__repr__()


class Vector(ResourceBase):
    def __init__(self, mem, element_type):
        ResourceBase.__init__(self, mem, element_type)
        size_in_bytes = read_value(self._mem, 0, SIZE_OFFSET_IN_BITS, False)
        size, rem = divmod(size_in_bytes, self._type_size_in_bytes)
        assert rem == 0, "Malformed vector"
        self._size = size

    def to_data_frame(self):
        return self[:].to_data_frame()

    def __getitem__(self, index):
        if isinstance(index, slice):
            return _VectorSlice(index, self)

        if index >= self._size:
            raise IndexError("Vector access out of bounds")
        if index < 0:
            index += len(self)
        return self._get_item(index)

    def __iter__(self):
        for i in range(len(self)):
            yield self._get_item(i)

    def __getattr__(self, name):
        return pd.DataFrame(data=[[getattr(item, name)] for item in self], columns=[name])

    def __len__(self):
        return self._size


class Multivector(ResourceBase):
    def __init__(self, index_mem, mem, index_type, *element_types):
        self._index = Vector(index_mem, index_type)
        self._mem = mem
        self._element_types = element_types
        self._index_type = index_type

    @classmethod
    def open(cls, storage, name, initializer, is_optional=False):
        return cls(storage.get(name + "_index", is_optional),
                   storage.get(name, is_optional),
                   *initializer)

    def __len__(self):
        return len(self._index)

    def _bucket_offset(self, index):
        return self._index[index].value + SIZE_OFFSET_IN_BYTES

    def __getitem__(self, index):
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

    def __iter__(self):
        for i in range(len(self)):
            yield self[i]

    def __repr__(self):
        attrs = self._repr_attributes()
        attrs.update(index_type=self._index_type._repr_attributes())
        return json.dumps(attrs, indent=4)


class RawData(ResourceBase):
    def __len__(self):
        return read_value(self._mem, 0, SIZE_OFFSET_IN_BITS, False)

    def __getitem__(self, item):
        if isinstance(item, slice):
            return self._mem[
                slice(item.start + SIZE_OFFSET_IN_BYTES,
                      (item.stop + SIZE_OFFSET_IN_BYTES) if item.stop is not None else None,
                      item.step)
            ]
        return self._mem[item + SIZE_OFFSET_IN_BYTES:item + SIZE_OFFSET_IN_BYTES + 1]


class Instance(ResourceBase):
    def __getattr__(self, name):
        offset = self._item_offset(0)
        return getattr(self._element_type(self._mem, offset), name)

    def __len__(self):
        return 1
