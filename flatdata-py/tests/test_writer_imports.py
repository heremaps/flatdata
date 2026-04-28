'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import os

import pytest

from flatdata.lib.flatdata_writer import Writer
from flatdata.generator.engine import Engine


def _write_files(tmpdir, files):
    """Write a dict of {relative_path: content} into tmpdir."""
    for rel_path, content in files.items():
        full = os.path.join(tmpdir, rel_path)
        os.makedirs(os.path.dirname(full), exist_ok=True)
        with open(full, "w") as f:
            f.write(content)


class TestWriterFromFile:
    """Tests for Writer.from_file() with import support."""

    def test_from_file_with_imports(self, tmp_path):
        """Writer.from_file() works with schemas that use imports."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "types.flatdata";
namespace n{
    archive A { r : vector< S >; }
}
''',
            "types.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        writer = Writer.from_file(
            str(tmp_path / "main.flatdata"),
            str(tmp_path / "output"),
        )
        assert writer.builder is not None

    def test_from_file_no_imports(self, tmp_path):
        """Writer.from_file() works with single-file schemas."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
namespace n{
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
'''
        })
        writer = Writer.from_file(
            str(tmp_path / "main.flatdata"),
            str(tmp_path / "output"),
        )
        assert writer.builder is not None

    def test_from_file_explicit_archive_name(self, tmp_path):
        """Writer.from_file() accepts explicit archive name."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "types.flatdata";
namespace n{
    archive MyArchive { r : vector< S >; }
}
''',
            "types.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        writer = Writer.from_file(
            str(tmp_path / "main.flatdata"),
            str(tmp_path / "output"),
            archive_name="MyArchive",
        )
        assert writer.builder is not None

    def test_from_file_infers_local_archive(self, tmp_path):
        """Writer.from_file() infers the local archive even when imports define other archives."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "lib.flatdata";
namespace n{
    archive Main { r : vector< .lib.S >; }
}
''',
            "lib.flatdata": '''
namespace lib{
    struct S { f : u8 : 8; }
    archive Lib { r : vector< S >; }
}
'''
        })
        writer = Writer.from_file(
            str(tmp_path / "main.flatdata"),
            str(tmp_path / "output"),
        )
        assert writer.builder is not None

    def test_from_file_missing_file(self, tmp_path):
        """Writer.from_file() raises RuntimeError for missing schema file."""
        with pytest.raises(RuntimeError):
            Writer.from_file(
                str(tmp_path / "nonexistent.flatdata"),
                str(tmp_path / "output"),
            )


class TestWriterStringConstructor:
    """Tests for Writer(schema_string) backward compatibility."""

    def test_string_constructor_works(self, tmp_path):
        """Writer(schema_string) still works for self-contained schemas."""
        schema = '''
namespace n{
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
'''
        writer = Writer(schema, str(tmp_path / "output"))
        assert writer.builder is not None

    def test_string_constructor_rejects_imports(self, tmp_path):
        """Writer(schema_string) raises RuntimeError when schema has imports."""
        schema = '''
import "types.flatdata";
namespace n{
    archive A { r : vector< S >; }
}
'''
        with pytest.raises(RuntimeError, match="import"):
            Writer(schema, str(tmp_path / "output"))


class TestInspectorSchemaPattern:
    """Verify the inspector's Engine(schema_string) pattern works for embedded schemas."""

    def test_embedded_schema_renders_module(self):
        """Engine(embedded_schema_string) renders correctly — no imports in embedded schemas."""
        schema = '''
namespace n{
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
'''
        engine = Engine(schema)
        module, archive_type = engine.render_python_module(
            archive_name="A")
        assert archive_type is not None
        assert hasattr(module, "n_A")

    def test_embedded_schema_from_imports_is_self_contained(self, tmp_path):
        """Schema embedding produces self-contained schemas with no import statements."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "types.flatdata";
namespace n{
    archive A { r : vector< S >; }
}
''',
            "types.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        output = engine.render("flatdata")
        # Embedded flatdata schema should be self-contained (no imports)
        assert "import" not in output
        # All types present
        assert "S" in output
        assert "archive A" in output
