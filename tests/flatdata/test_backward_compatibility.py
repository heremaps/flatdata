from generator.engine import Engine
from .common_testing_data import archive_signature_payload
from .dict_resource_storage import DictResourceStorage

from nose.tools import *


def check_signed_struct(s):
    eq_(-0x1, s.a)
    eq_(0x01234567, s.b)
    eq_(-0x28, s.c)
    eq_(0, s.d)


def check_simple_struct(s):
    eq_(0xFFFFFFFF, s.a)
    eq_(0xDEADBEEF, s.b)


def test_instance_reading():
    from .common_testing_data import instance_test_schema, resource_payload
    module = Engine(instance_test_schema).render_python_module()
    valid_data = {
        "Archive.archive": archive_signature_payload,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": resource_payload,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }
    archive = module.backward_compatibility_Archive(DictResourceStorage(valid_data))
    check_signed_struct(archive.resource)


def test_vector_reading():
    vector_test_schema = """
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

    resource_payload = (
        b"\x14\x00\x00\x00\x00\x00\x00\x00"  # Payload size in bytes
        b"\xff\xac\x68\x24\x00\x0b\x00\x00"  # Payload
        b"\x00\x00\xff\xac\x68\x24\x00\x0b"  # Payload
        b"\x00\x00\x00\x00"  # Payload
        b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
    )

    module = Engine(vector_test_schema).render_python_module()
    valid_data = {
        "Archive.archive": archive_signature_payload,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": resource_payload,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }

    archive = module.backward_compatibility_Archive(DictResourceStorage(valid_data))
    eq_(2, len(archive.resource))
    check_signed_struct(archive.resource[0])
    check_signed_struct(archive.resource[1])


def test_multivector_reading():
    multivector_test_schema = """
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

    multivector_resource_data = (
        b"\x31\x00\x00\x00\x00\x00\x00\x00"  # Payload size in bytes
        b"\x01\xff\xac\x68\x24\x00\x0b\x00\x00\x00\x00"  # Payload
        b"\x00\xff\xff\xff\xff\xef\xbe\xad\xde"  # Payload
        b"\x00\xff\xff\xff\xff\xef\xbe\xad\xde"  # Payload
        b"\x01\xff\xac\x68\x24\x00\x0b\x00\x00\x00\x00"  # Payload
        b"\x00\xff\xff\xff\xff\xef\xbe\xad\xde"  # Payload
        b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
    )

    multivector_resource_index = (
        b"\x14\x00\x00\x00\x00\x00\x00\x00"  # Index size in bytes
        b"\x00\x00\x00\x00\x00"  # Data pointer 1
        b"\x14\x00\x00\x00\x00"  # Data pointer 2
        b"\x14\x00\x00\x00\x00"  # Data pointer 3
        b"\x28\x00\x00\x00\x00"  # Data pointer 4
        b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
    )

    module = Engine(multivector_test_schema).render_python_module()
    valid_data = {
        "Archive.archive": archive_signature_payload,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": multivector_resource_data,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode(),
        "resource_index": multivector_resource_index,
        "resource_index.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }

    archive = module.backward_compatibility_Archive(DictResourceStorage(valid_data))
    eq_(4, len(archive.resource))

    eq_(2, len(archive.resource[0]))
    assert_is_instance(archive.resource[0][0], module.backward_compatibility_SignedStruct)
    check_signed_struct(archive.resource[0][0])
    assert_is_instance(archive.resource[0][1], module.backward_compatibility_SimpleStruct)
    check_simple_struct(archive.resource[0][1])

    eq_(0, len(archive.resource[1]))

    eq_(2, len(archive.resource[2]))
    assert_is_instance(archive.resource[2][0], module.backward_compatibility_SimpleStruct)
    check_simple_struct(archive.resource[2][0])
    assert_is_instance(archive.resource[2][1], module.backward_compatibility_SignedStruct)
    check_signed_struct(archive.resource[2][1])

    eq_(1, len(archive.resource[3]))
    assert_is_instance(archive.resource[3][0], module.backward_compatibility_SimpleStruct)
    check_simple_struct(archive.resource[3][0])


def test_raw_data_reading():
    raw_data_test_schema = """
namespace backward_compatibility {
    archive Archive {
        resource: raw_data;
    }
}
"""

    raw_data_resource_data = (
        b"\x05\x00\x00\x00\x00\x00\x00\x00"  # Payload size in bytes
        b"\xff\xef\xbe\xad\xde"  # Payload
        b"\x00\x00\x00\x00\x00\x00\x00\x00"  # Padding
    )

    module = Engine(raw_data_test_schema).render_python_module()
    valid_data = {
        "Archive.archive": archive_signature_payload,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": raw_data_resource_data,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode(),
    }

    archive = module.backward_compatibility_Archive(DictResourceStorage(valid_data))
    eq_(5, len(archive.resource))
    eq_(b"\xff", archive.resource[0])
    eq_(b"\xde", archive.resource[4])
    eq_(b"\xff\xef\xbe\xad\xde", archive.resource[0:5])
