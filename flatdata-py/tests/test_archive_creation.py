from flatdata.generator.engine import Engine
from common import DictResourceStorage, INSTANCE_TEST_SCHEMA, RESOURCE_PAYLOAD, ARCHIVE_SIGNATURE_PAYLOAD


def test_create_empty_archive():
    module = Engine(INSTANCE_TEST_SCHEMA).render_python_module()
    dict_storage = DictResourceStorage()

    archive = module.backward_compatibility_ArchiveBuilder(dict_storage)
    #archive["resource"].a = 5