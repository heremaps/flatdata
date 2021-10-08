'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import tarfile

from .errors import CorruptResourceError
from .errors import MissingResourceError
from .file_resource_storage import FileResourceStorage


class TarArchiveResourceStorage:
    """
    Resource storage based on a memory-mapped TAR archive.
    """

    def __init__(self, tar_map, file_entries, dir_entries, sub_path):
        self.tar_map = tar_map
        self.file_entries = file_entries
        self.dir_entries = dir_entries
        self.sub_path = sub_path

    @classmethod
    def create(cls, tar_path, sub_path=""):
        tar_map = FileResourceStorage.memory_map(tar_path)
        file_entries = dict()
        dir_entries = set()
        with tarfile.open(tar_path, "r:") as tar:
            for file in tar:
                name = file.name
                if name.startswith("./"):
                    name = name[2:]
                if file.type == tarfile.GNUTYPE_SPARSE:
                    raise CorruptResourceError("Sparse files are not supported")
                if file.isreg():
                    file_entries[name] = (file.offset_data, file.size)
                if file.isdir() and name != ".":
                    dir_entries.add(name)

        return cls(tar_map, file_entries, dir_entries, sub_path)

    def get(self, key, is_optional=False):
        path = self._path(key)
        if path in self.file_entries:
            (offset, length) = self.file_entries[path]
            return MemoryMapSection(self.tar_map, offset, length)

        if path in self.dir_entries:
            return TarArchiveResourceStorage(self.tar_map, self.file_entries, self.dir_entries, path)

        if not is_optional:
            raise MissingResourceError(key)
        else:
            return None

    def ls(self):
        prefix = self._path("")
        entries = []
        for d in self.dir_entries:
            if d.startswith(prefix) and '/' not in d[len(prefix):]:
                entries.append(d[len(prefix):])
        for f in self.file_entries:
            if f.startswith(prefix) and '/' not in f[len(prefix):]:
                entries.append(f[len(prefix):])
        return entries

    def _path(self, key):
        if not self.sub_path:
            return key
        else:
            return self.sub_path + '/' + key


class MemoryMapSection:
    """
    Represent a slice of a memory mapped file.
    Keeps track of its position, as to emulate pointing to a dedicated file.
    """

    def __init__(self, inner, offset, length):
        self.inner = inner
        self.offset = offset
        self.length = length;
        self.pos = 0

    def __len__(self):
        return self.size()

    def __getitem__(self, key):
        if isinstance(key, slice):
            start = key.start if key.start is not None else 0
            start = self.offset + min(start, self.length)
            stop = key.stop if key.stop is not None else self.length
            stop = self.offset + min(stop, self.length)
            return self.inner[slice(start, stop, key.step)]
        else:
            if key < self.length:
                return self.inner.__getitem__(self.offset + key)
            else:
                raise IndexError('index out of range')

    def read(self, n=None):
        if n is None:
            n = self.length - self.pos
        self.inner.seek(self.offset + self.pos)
        data = self.inner.read(min(n, self.length - self.pos))
        self.pos += len(data)
        return data

    def size(self):
        return min(self.length, self.inner.size() - self.offset)
