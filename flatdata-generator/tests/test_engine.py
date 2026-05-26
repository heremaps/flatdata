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

    def test_nonexistent_root_file_raises(self, tmp_path):
        """Non-existent root file raises FlatdataSyntaxError, not FileNotFoundError."""
        with pytest.raises(FlatdataSyntaxError):
            Engine.from_file(str(tmp_path / "nonexistent.flatdata"))

    def test_invalid_root_file_raises_parsing_error(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": 'this is not valid flatdata'
        })
        with pytest.raises(ParsingError):
            Engine.from_file(str(tmp_path / "main.flatdata"))

    def test_render_with_imports(self, tmp_path):
        """Flatdata generator produces self-contained output with all types."""
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
        # Both local and imported types are emitted (monolithic)
        assert "struct S" in output
        assert "archive A" in output
        # No import directives in output — schema must be self-contained
        assert "import" not in output

    def test_schema_embedding_self_contained(self, tmp_path):
        """Schema embedding includes all dependencies from imports."""
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
        archive = next(engine.tree.root.iterate(Archive))
        schema = engine.tree.schema(archive)
        assert "struct S" in schema
        assert "archive A" in schema
        assert "import" not in schema

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

    def test_python_monolithic_with_imports(self, tmp_path):
        """Python generator emits all types including imported ones."""
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
        output = engine.render("py")
        assert "n_S" in output
        assert "n_A" in output

    def test_dot_monolithic_with_imports(self, tmp_path):
        """Dot generator renders all types including imported ones."""
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
        output = engine.render("dot")
        # Archive rendered
        assert "cluster__n_A" in output
        # Imported struct rendered within the archive's resource
        assert "_n_A_r_n_S" in output

    def test_cpp_separate_compilation_with_imports(self, tmp_path):
        """C++ generator emits only local types and #include directives."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "types.flatdata";
namespace n{
    struct Local { x : u8 : 8; }
    archive A { r : vector< S >; r2 : vector< Local >; }
}
''',
            "types.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        output = engine.render("cpp")
        # Include directive for imported file
        assert '#include "types.h"' in output
        # Local struct definition IS emitted
        assert "LocalTemplate" in output
        # Imported struct S is NOT emitted as a C++ struct definition
        assert "STemplate" not in output

    def test_cpp_include_path_mapping(self, tmp_path):
        """C++ import paths map .flatdata to .h correctly."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "sub/types.flatdata";
namespace n{ archive A { r : vector< S >; } }
''',
            "sub/types.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        output = engine.render("cpp")
        assert '#include "sub/types.h"' in output

    def test_cpp_no_imports_unchanged(self):
        """C++ generator without imports produces normal output (no empty include block)."""
        engine = Engine('''
namespace n{
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
''')
        output = engine.render("cpp")
        assert "struct S" in output or "SType" in output
        # No user includes (only system includes)
        assert '#include "' not in output

    def test_rust_separate_compilation_same_namespace(self, tmp_path):
        """Rust generator emits only local types with pub use re-exports."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "types.flatdata";
namespace n{
    struct Local { x : u8 : 8; }
    archive A { r : vector< S >; r2 : vector< Local >; }
}
''',
            "types.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        output = engine.render("rust")
        # Local struct IS emitted
        assert "pub struct Local" in output
        # Imported struct S is NOT emitted as a definition
        assert "pub struct S " not in output
        # Re-export directive brings imported types into scope
        assert "pub use super::super::types::n::*;" in output
        # Schema embedding is self-contained (includes imported S)
        assert "struct S" in output  # appears in schema strings

    def test_rust_separate_compilation_cross_namespace(self, tmp_path):
        """Rust generates namespace shims with re-exports for imported namespaces."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "types.flatdata";
namespace app{ archive A { r : vector< .common.S >; } }
''',
            "types.flatdata": '''
namespace common{ struct S { f : u8 : 8; } }
'''
        })
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        output = engine.render("rust")
        # Imported-only namespace is still emitted as a module shim
        assert "pub mod common" in output
        assert "pub use super::super::types::common::*;" in output
        # Local namespace has the archive
        assert "pub mod app" in output
        assert "struct A" in output

    def test_rust_subdirectory_import(self, tmp_path):
        """Rust re-export paths handle subdirectory imports correctly."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "sub/types.flatdata";
namespace n{ archive A { r : vector< S >; } }
''',
            "sub/types.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        output = engine.render("rust")
        assert "pub use super::super::sub::types::n::*;" in output

    def test_rust_no_imports_unchanged(self):
        """Rust generator without imports produces normal output."""
        engine = Engine('''
namespace n{
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
''')
        output = engine.render("rust")
        assert "pub struct S" in output
        assert "pub use super::" not in output

    def test_rust_transitive_import_reexports(self, tmp_path):
        """Rust re-exports work for transitively imported types."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "mid.flatdata";
namespace n{ archive A { r : vector< .lib.S >; } }
''',
            "mid.flatdata": '''
import "lib.flatdata";
namespace n{ struct Mid { m : u8 : 8; } }
''',
            "lib.flatdata": '''
namespace lib{ struct S { f : u8 : 8; } }
'''
        })
        engine = Engine.from_file(str(tmp_path / "main.flatdata"))
        output = engine.render("rust")
        # Transitive import gets a re-export shim
        assert "pub mod lib" in output
        assert "pub use super::super::lib::lib::*;" in output
        # Direct import also re-exported
        assert "pub use super::super::mid::n::*;" in output

    def test_rust_parent_directory_import(self, tmp_path):
        """Rust re-exports use multiple super:: for parent directory imports."""
        _write_files(str(tmp_path), {
            "sub/main.flatdata": '''
import "../shared.flatdata";
namespace n{ archive A { r : vector< S >; } }
''',
            "shared.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        engine = Engine.from_file(str(tmp_path / "sub" / "main.flatdata"))
        output = engine.render("rust")
        # "../shared" needs two super:: (one sibling + one for "..")
        assert "pub use super::super::super::shared::n::*;" in output


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
