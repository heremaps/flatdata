from flatdata.generator.engine import Engine
from common import *

from nose.tools import eq_, assert_is_instance


def check_signed_struct(s):
    eq_(-0x1, s.a)
    eq_(0x01234567, s.b)
    eq_(-0x28, s.c)
    eq_(0, s.d)


def check_simple_struct(s):
    eq_(0xFFFFFFFF, s.a)
    eq_(0xDEADBEEF, s.b)


def test_instance_reading():
    module = Engine(INSTANCE_TEST_SCHEMA).render_python_module()
    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": RESOURCE_PAYLOAD,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }
    archive = module.backward_compatibility_Archive(DictResourceStorage(valid_data))
    check_signed_struct(archive.resource)
    check_signed_struct(archive.resource[0])


def test_vector_reading():
    module = Engine(VECTOR_TEST_SCHEMA).render_python_module()
    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": RESOURCE_VECTOR_PAYLOAD,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }

    archive = module.backward_compatibility_Archive(DictResourceStorage(valid_data))
    eq_(2, len(archive.resource))
    check_signed_struct(archive.resource[0])
    check_signed_struct(archive.resource[1])


def test_multivector_reading():
    module = Engine(MULTIVECTOR_TEST_SCHEMA).render_python_module()
    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": MULTIVECTOR_RESOURCE_DATA,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode(),
        "resource_index": MULTIVECTOR_RESOURCE_INDEX,
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
    module = Engine(RAW_DATA_TEST_SCHEMA).render_python_module()
    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": RAW_DATA_RESOURCE_DATA,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode(),
    }

    archive = module.backward_compatibility_Archive(DictResourceStorage(valid_data))
    eq_(5, len(archive.resource))
    eq_(b"\xff", archive.resource[0])
    eq_(b"\xde", archive.resource[4])
    eq_(b"\xff\xef\xbe\xad\xde", archive.resource[0:5])
