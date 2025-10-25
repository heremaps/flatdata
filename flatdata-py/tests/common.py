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


MULTI_NAMESPACE_TEST_SCHEMA = """
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
namespace second {
}
"""



VECTOR_TEST_SCHEMA = """
namespace backward_compatibility {
    struct SignedStruct {
        a : i16 : 5;
        b : u32 : 32;
        c : i32 : 7;
        d : u32 : 32;
    }
    archive Archive {
        resource: vector< SignedStruct >;
    }
}
"""

RESOURCE_VECTOR_PAYLOAD = (
    b"\x14\x00\x00\x00\x00\x00\x00\x00"  # Payload size in bytes
    b"\xff\xac\x68\x24\x00\x0b\x00\x00"  # Payload
    b"\x00\x00\xff\xac\x68\x24\x00\x0b"  # Payload
    b"\x00\x00\x00\x00"  # Payload
    b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
)

MULTIVECTOR_TEST_SCHEMA = """
namespace backward_compatibility {
    struct SimpleStruct {
        a : u32 : 32;
        b : u32 : 32;
    }
    struct SignedStruct {
        a : i16 : 5;
        b : u32 : 32;
        c : i32 : 7;
        d : u32 : 32;
    }
    archive Archive {
        resource: multivector< 33, SimpleStruct, SignedStruct >;
    }
}
"""

MULTIVECTOR_RESOURCE_DATA = (
    b"\x31\x00\x00\x00\x00\x00\x00\x00"  # Payload size in bytes
    b"\x01\xff\xac\x68\x24\x00\x0b\x00\x00\x00\x00"  # Payload
    b"\x00\xff\xff\xff\xff\xef\xbe\xad\xde"  # Payload
    b"\x00\xff\xff\xff\xff\xef\xbe\xad\xde"  # Payload
    b"\x01\xff\xac\x68\x24\x00\x0b\x00\x00\x00\x00"  # Payload
    b"\x00\xff\xff\xff\xff\xef\xbe\xad\xde"  # Payload
    b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
)

MULTIVECTOR_RESOURCE_INDEX = (
    b"\x19\x00\x00\x00\x00\x00\x00\x00"  # Index size in bytes
    b"\x00\x00\x00\x00\x00"  # Data pointer 1
    b"\x14\x00\x00\x00\x00"  # Data pointer 2
    b"\x14\x00\x00\x00\x00"  # Data pointer 3
    b"\x28\x00\x00\x00\x00"  # Data pointer 4
    b"\x31\x00\x00\x00\x00"  # Sentinel (end of data 4)
    b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
)

RAW_DATA_TEST_SCHEMA = """
namespace backward_compatibility {
    archive Archive {
        resource: raw_data;
    }
}
"""

RAW_DATA_RESOURCE_DATA = (
    b"\x05\x00\x00\x00\x00\x00\x00\x00"  # Payload size in bytes
    b"\xff\xef\xbe\xad\xde"  # Payload
    b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
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
