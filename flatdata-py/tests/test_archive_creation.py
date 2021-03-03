from flatdata.generator.engine import Engine
from flatdata.lib.archive_builder import FileResourceWriter
from common import DictResourceStorage, INSTANCE_TEST_SCHEMA, RESOURCE_PAYLOAD, ARCHIVE_SIGNATURE_PAYLOAD


class DummyResourceWriter:
    """
    Mimick `FileResourceWriter` and store all that is written in the dict `data`.
    """
    def __init__(self):
        self.data = dict()
    
    def get(self, key, is_subarchive):   
        if is_subarchive:
            return DummyResourceWriter()
        
        if key not in self.data:
            self.data[key] = bytearray()
        return DummyFileWriter(self.data[key])

class DummyFileWriter:
    """
    Mimick binary file writing and store result in `data` of
    the `BytesResourceWriter` that created it.
    """

    def __init__(self, data):
        self.data = data

    def write(self, thing):
        if isinstance(thing, str):
            self.data += thing.encode("utf-8")
        else:
            self.data += thing

    def close(self):
        pass


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
        assert memwrite.data[vkey] == vdata, f'"{vkey}" is "{memwrite.data[vkey]}", should be "{vdata}"'
