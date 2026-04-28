'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import os
import tempfile

import pytest

from flatdata.generator.tree.importer import (
    resolve_imports, ImportInfo
)
from flatdata.generator.tree.errors import ImportFileNotFoundError, ImportParsingError
from flatdata.generator.grammar import flatdata_grammar


class TestGrammarImport:
    """Test that the grammar correctly parses import statements."""

    def test_single_import(self):
        schema = 'import "bar.flatdata";\nnamespace foo { struct A { x : u32 : 32; } }'
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.imports) == 1
        assert parsed.imports[0].path == "bar.flatdata"

    def test_multiple_imports(self):
        schema = (
            'import "bar.flatdata";\n'
            'import "baz.flatdata";\n'
            'namespace foo { struct A { x : u32 : 32; } }'
        )
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.imports) == 2
        assert parsed.imports[0].path == "bar.flatdata"
        assert parsed.imports[1].path == "baz.flatdata"

    def test_import_with_path(self):
        schema = 'import "sub/dir/types.flatdata";\nnamespace foo { struct A { x : u32 : 32; } }'
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.imports) == 1
        assert parsed.imports[0].path == "sub/dir/types.flatdata"

    def test_no_imports(self):
        schema = 'namespace foo { struct A { x : u32 : 32; } }'
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.imports) == 0

    def test_import_only_file(self):
        """A file with only imports and no namespaces should parse."""
        schema = 'import "bar.flatdata";'
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.imports) == 1
        assert len(parsed.namespace) == 0

    def test_import_with_comment_before(self):
        schema = (
            '/* header comment */\n'
            'import "bar.flatdata";\n'
            'namespace foo { struct A { x : u32 : 32; } }'
        )
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.imports) == 1

    def test_comment_attached_to_import(self):
        """A comment directly before an import should be captured as its doc (except the first, which may be consumed by free_comments)."""
        schema = (
            'import "a.flatdata";\n'
            '/** docs for b */\n'
            'import "b.flatdata";\n'
            'namespace foo { struct A { x : u32 : 32; } }'
        )
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.imports) == 2
        assert parsed.imports[0].path == "a.flatdata"
        assert parsed.imports[1].path == "b.flatdata"
        assert "docs for b" in parsed.imports[1].doc

    def test_comment_between_imports_and_namespace(self):
        """A comment after the last import (before namespace) should attach to the namespace, not break parsing."""
        schema = (
            'import "a.flatdata";\n'
            '/* comment */\n'
            'namespace foo { struct A { x : u32 : 32; } }'
        )
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.imports) == 1

    def test_empty_schema(self):
        """An empty schema should parse (zero imports, zero namespaces)."""
        schema = ''
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.imports) == 0
        assert len(parsed.namespace) == 0

    def test_import_after_namespace(self):
        """Imports may appear after namespaces (relaxed ordering)."""
        schema = (
            'namespace foo { struct A { x : u32 : 32; } }\n'
            'import "bar.flatdata";'
        )
        parsed = flatdata_grammar.parse_string(schema, parse_all=True).flatdata
        assert len(parsed.namespace) == 1
        assert len(parsed.imports) == 1
        assert parsed.imports[0].path == "bar.flatdata"


def _write_temp_files(tmpdir: str, files: dict[str, str]) -> str:
    """Write files to tmpdir, return path to first file."""
    first_path = None
    for name, content in files.items():
        path = os.path.join(tmpdir, name)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, 'w') as f:
            f.write(content)
        if first_path is None:
            first_path = path
    assert first_path is not None
    return first_path


class TestResolveImports:
    """Test the recursive import resolver."""

    def test_no_imports(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": 'namespace foo { struct A { x : u32 : 32; } }'
            })
            files, root_imports = resolve_imports(root)
            assert len(files) == 1
            assert files[0].abs_path == os.path.realpath(root)
            assert root_imports == []

    def test_simple_import(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": (
                    'import "bar.flatdata";\n'
                    'namespace foo { struct A { x : u32 : 32; } }'
                ),
                "bar.flatdata": 'namespace bar { struct B { y : u32 : 32; } }'
            })
            files, root_imports = resolve_imports(root)
            assert len(files) == 2
            # Dependency-first order: bar before main
            assert files[0].abs_path == os.path.realpath(
                os.path.join(tmpdir, "bar.flatdata"))
            assert files[1].abs_path == os.path.realpath(root)
            assert len(root_imports) == 1
            assert root_imports[0].path == "bar.flatdata"

    def test_diamond_import(self):
        """A→B, A→C, B→D, C→D: D should appear only once."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "a.flatdata": (
                    'import "b.flatdata";\n'
                    'import "c.flatdata";\n'
                    'namespace a { struct A { x : u32 : 32; } }'
                ),
                "b.flatdata": (
                    'import "d.flatdata";\n'
                    'namespace b { struct B { x : u32 : 32; } }'
                ),
                "c.flatdata": (
                    'import "d.flatdata";\n'
                    'namespace c { struct C { x : u32 : 32; } }'
                ),
                "d.flatdata": 'namespace d { struct D { x : u32 : 32; } }'
            })
            files, root_imports = resolve_imports(
                os.path.join(tmpdir, "a.flatdata"))
            # Each file appears exactly once
            paths = [f.abs_path for f in files]
            assert len(paths) == len(set(paths))
            assert len(files) == 4
            # D should come before B and C (dependency-first)
            d_idx = next(i for i, f in enumerate(files)
                         if f.abs_path.endswith("d.flatdata"))
            b_idx = next(i for i, f in enumerate(files)
                         if f.abs_path.endswith("b.flatdata"))
            c_idx = next(i for i, f in enumerate(files)
                         if f.abs_path.endswith("c.flatdata"))
            assert d_idx < b_idx
            assert d_idx < c_idx

    def test_cyclic_import(self):
        """A→B, B→A: both files should be included, no infinite loop."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "a.flatdata": (
                    'import "b.flatdata";\n'
                    'namespace a { struct A { x : u32 : 32; } }'
                ),
                "b.flatdata": (
                    'import "a.flatdata";\n'
                    'namespace b { struct B { x : u32 : 32; } }'
                )
            })
            files, root_imports = resolve_imports(
                os.path.join(tmpdir, "a.flatdata"))
            assert len(files) == 2
            paths = {f.abs_path for f in files}
            assert any(p.endswith("a.flatdata") for p in paths)
            assert any(p.endswith("b.flatdata") for p in paths)

    def test_missing_import_file(self):
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": 'import "nonexistent.flatdata";\nnamespace foo { }'
            })
            with pytest.raises(ImportFileNotFoundError,
                               match="nonexistent.flatdata"):
                resolve_imports(root)

    def test_nested_path_import(self):
        """Import from a subdirectory."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": (
                    'import "sub/types.flatdata";\n'
                    'namespace foo { struct A { x : u32 : 32; } }'
                ),
                "sub/types.flatdata": 'namespace types { struct T { x : u32 : 32; } }'
            })
            files, root_imports = resolve_imports(
                os.path.join(tmpdir, "main.flatdata"))
            assert len(files) == 2
            assert root_imports[0].path == "sub/types.flatdata"

    @pytest.mark.skipif(not hasattr(os, 'symlink'), reason="symlinks not supported")
    def test_symlink_dedup(self):
        """Two imports of the same file via different paths (symlink) should dedup."""
        with tempfile.TemporaryDirectory() as tmpdir:
            _write_temp_files(tmpdir, {
                "real.flatdata": 'namespace r { struct R { x : u32 : 32; } }',
                "main.flatdata": (
                    'import "real.flatdata";\n'
                    'import "link.flatdata";\n'
                    'namespace m { struct M { x : u32 : 32; } }'
                )
            })
            # Create a symlink
            link_path = os.path.join(tmpdir, "link.flatdata")
            real_path = os.path.join(tmpdir, "real.flatdata")
            os.symlink(real_path, link_path)

            files, _ = resolve_imports(os.path.join(tmpdir, "main.flatdata"))
            # real.flatdata and link.flatdata resolve to the same canonical path
            assert len(files) == 2  # main + real (deduplicated)

    def test_relative_path_dedup(self):
        """Import via ./foo.flatdata and foo.flatdata should dedup."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": (
                    'import "bar.flatdata";\n'
                    'import "./bar.flatdata";\n'
                    'namespace m { struct M { x : u32 : 32; } }'
                ),
                "bar.flatdata": 'namespace b { struct B { x : u32 : 32; } }'
            })
            files, _ = resolve_imports(root)
            assert len(files) == 2  # main + bar (deduplicated)

    def test_content_preserved(self):
        """The returned content should preserve the original file content including import lines."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": (
                    'import "bar.flatdata";\n'
                    'namespace foo { struct A { x : u32 : 32; } }'
                ),
                "bar.flatdata": 'namespace bar { struct B { y : u32 : 32; } }'
            })
            files, _ = resolve_imports(root)
            main_file = next(f for f in files if f.abs_path.endswith("main.flatdata"))
            assert 'import "bar.flatdata"' in main_file.content
            assert 'namespace foo' in main_file.content

    def test_transitive_import(self):
        """A→B→C: all three files should be included."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "a.flatdata": (
                    'import "b.flatdata";\n'
                    'namespace a { struct A { x : u32 : 32; } }'
                ),
                "b.flatdata": (
                    'import "c.flatdata";\n'
                    'namespace b { struct B { x : u32 : 32; } }'
                ),
                "c.flatdata": 'namespace c { struct C { x : u32 : 32; } }'
            })
            files, root_imports = resolve_imports(
                os.path.join(tmpdir, "a.flatdata"))
            assert len(files) == 3
            # C before B before A
            paths = [os.path.basename(f.abs_path) for f in files]
            assert paths.index("c.flatdata") < paths.index("b.flatdata")
            assert paths.index("b.flatdata") < paths.index("a.flatdata")
            # Only direct imports of A returned
            assert len(root_imports) == 1
            assert root_imports[0].path == "b.flatdata"

    def test_parsed_result_cached(self):
        """Each ResolvedFile should carry its cached parse result."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": (
                    'import "bar.flatdata";\n'
                    'namespace foo { struct A { x : u32 : 32; } }'
                ),
                "bar.flatdata": 'namespace bar { struct B { y : u32 : 32; } }'
            })
            files, _ = resolve_imports(root)
            for f in files:
                assert f.parsed is not None
                assert "namespace" in f.parsed
            main = next(f for f in files if f.abs_path.endswith("main.flatdata"))
            assert "imports" in main.parsed
            assert main.parsed["imports"][0]["path"] == "bar.flatdata"

    def test_parse_error_in_imported_file(self):
        """A syntax error in an imported file should report the file path."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": (
                    'import "bad.flatdata";\n'
                    'namespace foo { struct A { x : u32 : 32; } }'
                ),
                "bad.flatdata": 'this is not valid flatdata syntax'
            })
            with pytest.raises(ImportParsingError, match="bad.flatdata"):
                resolve_imports(root)

    def test_parse_syntax_error_in_imported_file(self):
        """A ParseSyntaxException (error-stop) in an imported file should be caught."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": (
                    'import "bad.flatdata";\n'
                    'namespace foo { struct A { x : u32 : 32; } }'
                ),
                "bad.flatdata": 'namespace bar { struct { } }'
            })
            with pytest.raises(ImportParsingError, match="bad.flatdata"):
                resolve_imports(root)

    def test_import_of_empty_file(self):
        """An imported empty file should parse successfully."""
        with tempfile.TemporaryDirectory() as tmpdir:
            root = _write_temp_files(tmpdir, {
                "main.flatdata": (
                    'import "empty.flatdata";\n'
                    'namespace foo { struct A { x : u32 : 32; } }'
                ),
                "empty.flatdata": ''
            })
            files, _ = resolve_imports(root)
            assert len(files) == 2
