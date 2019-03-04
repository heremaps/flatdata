'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from flatdata.errors import MissingResourceError

class DictResourceStorage(object):
    """
    Resource storage based on dict.
    """
    def __init__(self, data=None):
        self.data = data if data is not None else dict()

    def get(self, key, is_optional=False):
        if key not in self.data:
            if not is_optional:
                raise MissingResourceError(key)
            else:
                return None

        value = self.data[key]
        if isinstance(value, bytes):

            class Monkey(bytes):
                def read(self):
                    return self
            return Monkey(value)

        assert isinstance(value, dict)
        return DictResourceStorage(data=value)
