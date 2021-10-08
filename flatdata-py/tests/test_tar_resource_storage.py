from common import *
from flatdata.generator.engine import Engine
from flatdata.lib.tar_archive_resource_storage import TarArchiveResourceStorage

from nose.tools import eq_
import tarfile
import tempfile
import os


def check_signed_struct(s):
    eq_(-0x1, s.a)
    eq_(0x01234567, s.b)
    eq_(-0x28, s.c)
    eq_(0, s.d)


def test_tar_resource_storage():
    module = Engine(INSTANCE_TEST_SCHEMA).render_python_module()
    valid_data = {
        "Archive.archive": ARCHIVE_SIGNATURE_PAYLOAD,
        "Archive.archive.schema": module.backward_compatibility_Archive.schema().encode(),
        "resource": RESOURCE_PAYLOAD,
        "resource.schema": module.backward_compatibility_Archive.resource_schema('resource').encode()
    }

    with tempfile.TemporaryDirectory() as tmpdir:
        archive_path = os.path.join(tmpdir, "archive.tar")
        cwd = os.getcwd()
        os.chdir(tmpdir)
        tar = tarfile.open(archive_path, "w")
        for key, value in valid_data.items():
            with open(os.path.join(tmpdir, key), "wb") as file:
                file.write(value)
            tar.add(key)
        tar.close()
        os.chdir(cwd)

        archive = module.backward_compatibility_Archive(TarArchiveResourceStorage.create(archive_path))
        check_signed_struct(archive.resource)
