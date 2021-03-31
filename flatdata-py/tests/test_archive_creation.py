from common import *
from flatdata.generator.engine import Engine
from flatdata.lib.resource_storage import _Resource


class DummyResourceWriter:
    """
    Mimick `FileResourceWriter` and store all that is written in the dict `data`.
    """
    def __init__(self):
        self.data = dict()

    def get(self, key, is_subarchive=False):   
        if is_subarchive:
            return DummyResourceWriter()
        
        if key not in self.data:
            self.data[key] = bytearray()

        self.data[key] = DummyFileWriter(key, self.data[key])
        return self.data[key]
    
    def open(self, name, file_path):
        pass

    def write(self, data):
        print("#1 write called")
        pass

    def close(self):
        pass

class DummyFileWriter(_Resource):
    """
    Mimick binary file writing and store result in `data` of
    the `BytesResourceWriter` that created it.
    """
    def __init__(self, key, data):
        super().__init__(key, path="dummypath")

def test_create_example_archive():
    module = Engine(INSTANCE_TEST_SCHEMA).render_python_module()
    memwrite = DummyResourceWriter()

    builder = module.backward_compatibility_ArchiveBuilder(memwrite)
    builder.set("resource", {"a": -0x1, "b": 0x01234567, "c": -0x28, "d": 0})
    builder.finish()

    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": RESOURCE_PAYLOAD,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }

    for (vkey, vdata) in valid_data.items():
        assert memwrite.data[vkey].get_data() == vdata, f'"{vkey}" is "{memwrite.data[vkey]}", should be "{vdata}"'

def test_create_vector_archive():
    module = Engine(VECTOR_TEST_SCHEMA).render_python_module()
    memwrite = DummyResourceWriter()

    builder = module.backward_compatibility_ArchiveBuilder(memwrite)
    builder.set("resource", [{"a": -0x1, "b": 0x01234567, "c": -0x28, "d": 0}] * 2)
    builder.finish()

    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": RESOURCE_VECTOR_PAYLOAD,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }

    for (vkey, vdata) in valid_data.items():
        assert memwrite.data[vkey].get_data() == vdata, f'"{vkey}" is "{memwrite.data[vkey]}", should be "{vdata}"'


def test_create_multivector_archive():
    multivector_data = [
    [
        {
            "name": "backward_compatibility_SignedStruct",
            "attributes": {
                "a": -1,
                "b": 19088743,
                "c": -40,
                "d": 0
            }
        },
        {
            "name": "backward_compatibility_SimpleStruct",
            "attributes": {
                "a": 4294967295,
                "b": 3735928559
            }
        }
    ],
    [],
    [
        {
            "name": "backward_compatibility_SimpleStruct",
            "attributes": {
                "a": 4294967295,
                "b": 3735928559
            }
        },
        {
            "name": "backward_compatibility_SignedStruct",
            "attributes": {
                "a": -1,
                "b": 19088743,
                "c": -40,
                "d": 0
            }
        }
    ],
    [
        {
            "name": "backward_compatibility_SimpleStruct",
            "attributes": {
                "a": 4294967295,
                "b": 3735928559
            }
        }
    ]
    ]

    module = Engine(MULTIVECTOR_TEST_SCHEMA).render_python_module()
    memwrite = DummyResourceWriter()

    builder = module.backward_compatibility_ArchiveBuilder(memwrite)
    builder.set("resource", multivector_data)
    builder.finish()

    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": MULTIVECTOR_RESOURCE_DATA,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode(),
        "resource_index": MULTIVECTOR_RESOURCE_INDEX,
        "resource_index.schema": bytearray(f'index({module.backward_compatibility_Archive.resource_schema("resource")})'.encode())
    }

    for (vkey, vdata) in valid_data.items():
        assert memwrite.data[vkey].get_data() == vdata, f'"{vkey}" is "{memwrite.data[vkey]}", should be "{vdata}"'

def test_create_raw_data():
    module = Engine(RAW_DATA_TEST_SCHEMA).render_python_module()
    memwrite = DummyResourceWriter()

    builder = module.backward_compatibility_ArchiveBuilder(memwrite)
    builder.set("resource", b"\xff\xef\xbe\xad\xde")
    builder.finish()

    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": RAW_DATA_RESOURCE_DATA,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode(),
    }

    for (vkey, vdata) in valid_data.items():
        assert memwrite.data[vkey].get_data() == vdata, f'"{vkey}" is "{memwrite.data[vkey]}", should be "{vdata}"'