from flatdata.generator.engine import Engine
from common import *

import pytest


def check_signed_struct(s):
    assert -0x1 == s.a
    assert 0x01234567 == s.b
    assert -0x28 == s.c
    assert 0 == s.d


def check_simple_struct(s):
    assert 0xFFFFFFFF == s.a
    assert 0xDEADBEEF == s.b


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


def test_multi_namespace_instance_reading():
    root_namespace = "backward_compatibility"
    module = Engine(MULTI_NAMESPACE_TEST_SCHEMA).render_python_module(None, None, root_namespace)
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
    assert 2 == len(archive.resource)
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
    assert 4 == len(archive.resource)

    assert 2 == len(archive.resource[0])
    assert isinstance(archive.resource[0][0], module.backward_compatibility_SignedStruct)
    check_signed_struct(archive.resource[0][0])
    assert isinstance(archive.resource[0][1], module.backward_compatibility_SimpleStruct)
    check_simple_struct(archive.resource[0][1])

    assert 0 == len(archive.resource[1])

    assert 2 == len(archive.resource[2])
    assert isinstance(archive.resource[2][0], module.backward_compatibility_SimpleStruct)
    check_simple_struct(archive.resource[2][0])
    assert isinstance(archive.resource[2][1], module.backward_compatibility_SignedStruct)
    check_signed_struct(archive.resource[2][1])

    assert 1 == len(archive.resource[3])
    assert isinstance(archive.resource[3][0], module.backward_compatibility_SimpleStruct)
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
    assert 5 == len(archive.resource)
    assert b"\xff" == archive.resource[0]
    assert b"\xde" == archive.resource[4]
    assert b"\xff\xef\xbe\xad\xde" == archive.resource[0:5]
