"""Common testing helpers & data"""

from flatdata.lib.errors import MissingResourceError


INSTANCE_TEST_SCHEMA = """
namespace backward_compatibility {
    struct SignedStruct {
        a : i16 : 5;
        b : u32 : 32;
        c : i32 : 7;
        d : u32 : 32;
    }
    archive Archive {
        resource: SignedStruct;
    }
}
"""
RESOURCE_PAYLOAD = (
    b"\x0a\x00\x00\x00\x00\x00\x00\x00"  # size of payload in bytes
    b"\xff\xac\x68\x24\x00\x0b\x00\x00"  # Payload
    b"\x00\x00"  # Payload
    b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
)

ARCHIVE_SIGNATURE_PAYLOAD = (
    b"\x00\x00\x00\x00\x00\x00\x00\x00"
    b"\x00\x00\x00\x00\x00\x00\x00\x00"
)


# pylint: disable=too-few-public-methods,missing-docstring
class DictResourceStorage:
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

            class _Monkey(bytes):
                def read(self):
                    return self
            return _Monkey(value)

        assert isinstance(value, dict)
        return DictResourceStorage(data=value)
