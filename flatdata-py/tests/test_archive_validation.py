from flatdata.generator.engine import Engine
from flatdata.lib.errors import CorruptArchiveError, SchemaMismatchError
from common import DictResourceStorage, INSTANCE_TEST_SCHEMA, RESOURCE_PAYLOAD, ARCHIVE_SIGNATURE_PAYLOAD

from nose.tools import assert_raises


def test_archive_does_not_open_on_signature_resource_or_schemas_missing():
    module = Engine(INSTANCE_TEST_SCHEMA).render_python_module()
    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": RESOURCE_PAYLOAD,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }

    missing_signature = valid_data.copy()
    del missing_signature["Archive.archive"]

    corrupt_signature = valid_data.copy()
    corrupt_signature["Archive.archive"] = b'\xde\xad\xbe\xef'

    corrupt_signature_more_than_8_bytes = valid_data.copy()
    corrupt_signature_more_than_8_bytes["Archive.archive"] = b'\xde\xad\xbe\xef\0\0\0\0\0\0\0\0'

    missing_schema = valid_data.copy()
    del missing_schema["Archive.archive.schema"]

    corrupt_schema = valid_data.copy()
    corrupt_schema["Archive.archive.schema"] = b"foo"

    missing_resource = valid_data.copy()
    del missing_resource["resource"]

    missing_resource_schema = valid_data.copy()
    del missing_resource_schema["resource.schema"]

    corrupt_resource_schema = valid_data.copy()
    corrupt_resource_schema["resource.schema"] = b"foo"

    datasets = [
        (missing_signature, CorruptArchiveError),
        (corrupt_signature, CorruptArchiveError),
        (missing_schema, CorruptArchiveError),
        (corrupt_schema, SchemaMismatchError),
        (missing_resource, CorruptArchiveError),
        (missing_resource_schema, CorruptArchiveError),
        (corrupt_resource_schema, SchemaMismatchError),
    ]

    def _test(index, data, error_type):
        with assert_raises(error_type):
            module.backward_compatibility_Archive(DictResourceStorage(data))

    for index, payload in enumerate(datasets):
        data, error_type = payload
        yield _test, index, data, error_type
