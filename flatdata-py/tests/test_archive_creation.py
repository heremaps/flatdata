from flatdata.generator.engine import Engine
from flatdata.lib.archive_builder import FileResourceWriter
from common import DictResourceStorage, INSTANCE_TEST_SCHEMA, RESOURCE_PAYLOAD, ARCHIVE_SIGNATURE_PAYLOAD


class BytesResourceWriter:
    def __init__(self):
        self.data = bytearray()
    
    def get(self, key, is_subarchive):   
        if is_subarchive:
            return BytesResourceWriter()
        
        return self

    def write(self, thing):
        self.data += thing

    def close(self):
        pass


def test_create_empty_archive():
    module = Engine(INSTANCE_TEST_SCHEMA).render_python_module()
    memwrite = BytesResourceWriter()

    builder = module.backward_compatibility_ArchiveBuilder(memwrite)
    # is setting some dict fine for setting a struture, or
    # should we require it to be the specific class?
    builder.set("resource", {"a": -0x1, "b": 0x01234567, "c": -0x28, "d": 0})

    assert memwrite.data == RESOURCE_PAYLOAD, f'\nis            "{memwrite.data.hex()}",\nbut should be "{RESOURCE_PAYLOAD.hex()}"'
    # do we need something like a finish/close method?
    # if we do, should we try to make it work with python's
    # `with ... as archive_builder:` syntax?
    #builder.finish()
