'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import os

import pytest

from flatdata.generator.engine import Engine
from flatdata.generator.tree.errors import (
    FlatdataSyntaxError, ImportFileNotFoundError, ParsingError)
from flatdata.generator.tree.nodes.trivial import Structure
from flatdata.generator.tree.nodes.archive import Archive


def _write_files(tmpdir, files):
    """Write a dict of {relative_path: content} into tmpdir."""
    for rel_path, content in files.items():
        full = os.path.join(tmpdir, rel_path)
        os.makedirs(os.path.dirname(full), exist_ok=True)
        with open(full, "w") as f:
            f.write(content)


class TestEngineFromFile:
    """Tests for Engine.from_file() with import support."""

    def test_single_file(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": '''
namespace n{
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
'''
        })
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        assert engine.tree is not None
        assert len(list(engine.tree.root.iterate(Archive))) == 1

    def test_with_imports(self, tmp_path):
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
        structs = list(engine.tree.root.iterate(Structure))
        archives = list(engine.tree.root.iterate(Archive))
        assert any(s.name == "S" for s in structs)
        assert any(a.name == "A" for a in archives)

    def test_schema_attribute_contains_root_file_content(self, tmp_path):
        content = 'namespace n{ struct S { f : u8 : 8; } }'
        _write_files(str(tmp_path), {"main.flatdata": content})
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        assert engine.schema == content

    def test_missing_import_raises(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": 'import "missing.flatdata"; namespace n{ struct S { f : u8 : 8; } }'
        })
        with pytest.raises(ImportFileNotFoundError):
            Engine.from_file(str(tmp_path / "main.flatdata"))

    def test_invalid_root_file_raises_parsing_error(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": 'this is not valid flatdata'
        })
        with pytest.raises(ParsingError):
            Engine.from_file(str(tmp_path / "main.flatdata"))

    def test_render_with_imports(self, tmp_path):
        """Engine.from_file() produces a renderable tree."""
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
        # Should not raise — tree is valid and renderable
        output = engine.render("flatdata")
        assert "struct S" in output

    def test_imports_metadata_available(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "types.flatdata";
namespace n{ archive A { r : vector< S >; } }
''',
            "types.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        assert len(engine.tree.imports) == 1
        assert engine.tree.imports[0].path == "types.flatdata"


class TestEngineBackwardCompat:
    """Verify Engine(schema_string) still works unchanged."""

    def test_engine_string_constructor(self):
        engine = Engine('''
namespace n{
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
''')
        assert engine.tree is not None
        structs = list(engine.tree.root.iterate(Structure))
        assert any(s.name == "S" for s in structs)

    def test_engine_string_render(self):
        engine = Engine('namespace n{ struct S { f : u8 : 8; } }')
        output = engine.render("flatdata")
        assert "struct S" in output

    def test_engine_string_no_imports(self):
        engine = Engine('namespace n{ struct S { f : u8 : 8; } }')
        assert len(engine.tree.imports) == 0

    def test_engine_string_with_imports_raises(self):
        """Import statements in string-based Engine should raise an error."""
        from flatdata.generator.tree.errors import UnresolvedImportError
        with pytest.raises(UnresolvedImportError):
            Engine('import "foo.flatdata"; namespace n{ struct S { f : u8 : 8; } }')
