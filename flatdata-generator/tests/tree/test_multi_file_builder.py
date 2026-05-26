'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import os
import tempfile

import pytest

from flatdata.generator.tree.builder import build_ast_from_file
from flatdata.generator.tree.errors import (
    ImportFileNotFoundError, ImportParsingError, ParsingError, SymbolRedefinition)
from flatdata.generator.tree.nodes.trivial import Structure, Constant, Enumeration
from flatdata.generator.tree.nodes.archive import Archive


def _write_files(tmpdir, files):
    """Write a dict of {relative_path: content} into tmpdir, return root path."""
    for rel_path, content in files.items():
        full = os.path.join(tmpdir, rel_path)
        os.makedirs(os.path.dirname(full), exist_ok=True)
        with open(full, "w") as f:
            f.write(content)
    return tmpdir


class TestBuildAstFromFile:
    """Tests for multi-file AST building via build_ast_from_file."""

    def test_single_file_no_imports(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": 'namespace n{ struct S { f : u8 : 8; } }'
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        assert len(tree.imports) == 0
        structs = list(tree.root.iterate(Structure))
        assert any(s.name == "S" for s in structs)
        assert all(s.is_local for s in structs)

    def test_simple_import(self, tmp_path):
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
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        types_file = os.path.realpath(str(tmp_path / "types.flatdata"))
        root_file = os.path.realpath(str(tmp_path / "main.flatdata"))

        # imports contain direct imports of root
        assert len(tree.imports) == 1
        assert tree.imports[0].path == "types.flatdata"

        # Nodes from both files are in the tree
        structs = list(tree.root.iterate(Structure))
        assert any(s.name == "S" for s in structs)
        archives = list(tree.root.iterate(Archive))
        assert any(a.name == "A" for a in archives)

        # Source file tagging
        s_node = next(s for s in structs if s.name == "S")
        a_node = next(a for a in archives if a.name == "A")
        assert s_node.source_file == types_file
        assert a_node.source_file == root_file

        # is_local
        assert a_node.is_local
        assert not s_node.is_local
        assert tree.is_local_node(a_node)
        assert not tree.is_local_node(s_node)

    def test_diamond_import(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "a.flatdata";
import "b.flatdata";
namespace n{ archive Main { r : vector< S >; } }
''',
            "a.flatdata": '''
import "common.flatdata";
namespace n{ struct A { f : u8 : 8; } }
''',
            "b.flatdata": '''
import "common.flatdata";
namespace n{ struct B { f : u8 : 8; } }
''',
            "common.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))

        # All types present, no SymbolRedefinition
        structs = list(tree.root.iterate(Structure))
        names = {s.name for s in structs}
        assert "S" in names
        assert "A" in names
        assert "B" in names

        # Only direct imports of root
        import_paths = {i.path for i in tree.imports}
        assert import_paths == {"a.flatdata", "b.flatdata"}

    def test_cyclic_import(self, tmp_path):
        _write_files(str(tmp_path), {
            "parent.flatdata": '''
import "child.flatdata";
namespace n{
    struct ParentData { f : u8 : 8; }
    archive Parent { r : vector< ChildData >; }
}
''',
            "child.flatdata": '''
import "parent.flatdata";
namespace n{
    struct ChildData { f : u8 : 8; }
    archive Child { r : vector< ParentData >; }
}
'''
        })
        tree = build_ast_from_file(str(tmp_path / "parent.flatdata"))

        structs = list(tree.root.iterate(Structure))
        names = {s.name for s in structs}
        assert "ParentData" in names
        assert "ChildData" in names

        archives = list(tree.root.iterate(Archive))
        archive_names = {a.name for a in archives}
        assert "Parent" in archive_names
        assert "Child" in archive_names

    def test_cross_file_reference_resolution(self, tmp_path):
        """Types from imported file can be referenced by root file."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "types.flatdata";
namespace n{
    archive A { data : vector< Point >; }
}
''',
            "types.flatdata": '''
namespace n{ struct Point { x : u32 : 32; y : u32 : 32; } }
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))

        archives = list(tree.root.iterate(Archive))
        assert len(archives) == 1
        assert archives[0].name == "A"

    def test_cross_file_enum_reference(self, tmp_path):
        """Enum from imported file can be used as field type."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "enums.flatdata";
namespace n{
    struct Obj { kind : .n.Kind : 8; }
}
''',
            "enums.flatdata": '''
namespace n{
    enum Kind : u8 { A = 0, B = 1 }
}
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        structs = list(tree.root.iterate(Structure))
        assert any(s.name == "Obj" for s in structs)

    def test_transitive_import(self, tmp_path):
        """Transitive imports: main→a→b, main can use types from b."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "a.flatdata";
namespace n{ archive A { r : vector< Deep >; } }
''',
            "a.flatdata": '''
import "b.flatdata";
namespace n{ struct Mid { f : u8 : 8; } }
''',
            "b.flatdata": '''
namespace n{ struct Deep { f : u8 : 8; } }
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        structs = list(tree.root.iterate(Structure))
        names = {s.name for s in structs}
        assert "Mid" in names
        assert "Deep" in names

    def test_nested_path_import(self, tmp_path):
        """Import from a subdirectory."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "sub/types.flatdata";
namespace n{ archive A { r : vector< S >; } }
''',
            "sub/types.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        structs = list(tree.root.iterate(Structure))
        assert any(s.name == "S" for s in structs)

    def test_missing_import_raises_error(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": 'import "missing.flatdata"; namespace n{ struct S { f : u8 : 8; } }'
        })
        with pytest.raises(ImportFileNotFoundError):
            build_ast_from_file(str(tmp_path / "main.flatdata"))

    def test_import_parse_error_raises(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": 'import "bad.flatdata"; namespace n{ struct S { f : u8 : 8; } }',
            "bad.flatdata": 'this is not valid flatdata'
        })
        with pytest.raises(ImportParsingError):
            build_ast_from_file(str(tmp_path / "main.flatdata"))

    def test_root_file_parse_error_raises_parsing_error(self, tmp_path):
        """Root file with invalid syntax raises ParsingError, not ImportParsingError."""
        _write_files(str(tmp_path), {
            "main.flatdata": 'this is not valid flatdata'
        })
        with pytest.raises(ParsingError):
            build_ast_from_file(str(tmp_path / "main.flatdata"))

    def test_symbol_redefinition_across_files(self, tmp_path):
        """Same struct name in same namespace across different files → error."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "other.flatdata";
namespace n{ struct S { f : u8 : 8; } }
''',
            "other.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        with pytest.raises(SymbolRedefinition):
            build_ast_from_file(str(tmp_path / "main.flatdata"))


class TestSourceFileTagging:
    """Tests for source_file propagation on AST nodes."""

    def test_all_toplevel_types_tagged(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": '''
namespace n{
    const u8 C = 42;
    enum E : u8 { A = 0 }
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        root_file = os.path.realpath(str(tmp_path / "main.flatdata"))

        for node_type in [Structure, Constant, Enumeration, Archive]:
            for node in tree.root.iterate(node_type):
                if node.name.startswith("_"):
                    continue  # skip builtins
                assert node.source_file == root_file, \
                    f"{node_type.__name__} '{node.name}' not tagged with source file"

    def test_imported_nodes_tagged_with_import_file(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "lib.flatdata";
namespace n{ archive A { r : vector< S >; } }
''',
            "lib.flatdata": '''
namespace n{ struct S { f : u8 : 8; } }
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        lib_file = os.path.realpath(str(tmp_path / "lib.flatdata"))

        s_node = next(s for s in tree.root.iterate(Structure) if s.name == "S")
        assert s_node.source_file == lib_file

    def test_builtin_structures_tagged(self, tmp_path):
        """Builtin structures created for multivectors inherit source_file."""
        _write_files(str(tmp_path), {
            "main.flatdata": '''
namespace n{
    struct A { f : u8 : 8; }
    struct B { f : u8 : 8; }
    archive Ar { mv : multivector< 33, A, B >; }
}
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        root_file = os.path.realpath(str(tmp_path / "main.flatdata"))

        # Find builtin structures (in _builtin namespace)
        all_structs = list(tree.root.iterate(Structure))
        builtin_structs = [s for s in all_structs if "_builtin" in s.path]
        assert len(builtin_structs) > 0, "Expected builtin structures for multivector"

        for bs in builtin_structs:
            assert bs.source_file == root_file, \
                f"Builtin struct '{bs.name}' not tagged with source file"


class TestIsLocalNode:
    """Tests for SyntaxTree.is_local_node()."""

    def test_no_file_tracking_all_local(self, tmp_path):
        """When built from string (no file tracking), all nodes are local."""
        from flatdata.generator.tree.builder import build_ast
        tree = build_ast('namespace n{ struct S { f : u8 : 8; } }')
        structs = list(tree.root.iterate(Structure))
        for s in structs:
            assert tree.is_local_node(s)

    def test_local_vs_imported(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "lib.flatdata";
namespace n{
    struct Local { f : u8 : 8; }
    archive A { r : vector< Imported >; }
}
''',
            "lib.flatdata": '''
namespace n{ struct Imported { f : u8 : 8; } }
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))

        local = next(s for s in tree.root.iterate(Structure) if s.name == "Local")
        imported = next(s for s in tree.root.iterate(Structure) if s.name == "Imported")

        assert tree.is_local_node(local)
        assert not tree.is_local_node(imported)

    def test_child_nodes_inherit_locality(self, tmp_path):
        """Fields and resources inherit is_local_node from parent."""
        from flatdata.generator.tree.nodes.trivial import Field
        from flatdata.generator.tree.nodes.resources import Vector

        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "lib.flatdata";
namespace n{
    struct Local { f : u8 : 8; }
    archive A { r : vector< Local >; }
}
''',
            "lib.flatdata": '''
namespace n{ struct Imported { g : u8 : 8; } }
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))

        # Field of local struct
        local_struct = next(s for s in tree.root.iterate(Structure) if s.name == "Local")
        local_field = next(local_struct.iterate(Field))
        assert tree.is_local_node(local_field)

        # Field of imported struct
        imported_struct = next(s for s in tree.root.iterate(Structure) if s.name == "Imported")
        imported_field = next(imported_struct.iterate(Field))
        assert not tree.is_local_node(imported_field)


class TestBackwardCompatibility:
    """Verify build_ast() still works unchanged."""

    def test_build_ast_string_unchanged(self):
        from flatdata.generator.tree.builder import build_ast
        tree = build_ast('''
namespace n{
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
''')
        assert len(tree.imports) == 0
        structs = list(tree.root.iterate(Structure))
        assert any(s.name == "S" for s in structs)
        # All nodes default to is_local=True
        assert all(s.is_local for s in structs)

    def test_build_ast_empty_string(self):
        from flatdata.generator.tree.builder import build_ast
        tree = build_ast("")
        assert len(tree.imports) == 0


class TestReferenceNodeTagging:
    """Tests for pipeline-created reference nodes being tagged."""

    def test_builtin_structure_references_tagged(self, tmp_path):
        from flatdata.generator.tree.nodes.references import BuiltinStructureReference

        _write_files(str(tmp_path), {
            "main.flatdata": '''
namespace n{
    struct A { f : u8 : 8; }
    archive Ar { mv : multivector< 33, A >; }
}
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        root_file = os.path.realpath(str(tmp_path / "main.flatdata"))

        refs = list(tree.root.iterate(BuiltinStructureReference))
        assert len(refs) > 0
        for ref in refs:
            assert ref.source_file == root_file
            assert ref.is_local

    def test_constant_value_references_tagged(self, tmp_path):
        from flatdata.generator.tree.nodes.references import ConstantValueReference

        _write_files(str(tmp_path), {
            "main.flatdata": '''
namespace n{
    const u8 C = 42;
    struct S { f : u8 : 8; }
    archive A { r : vector< S >; }
}
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))
        root_file = os.path.realpath(str(tmp_path / "main.flatdata"))

        refs = list(tree.root.iterate(ConstantValueReference))
        assert len(refs) > 0
        for ref in refs:
            assert ref.source_file == root_file
            assert ref.is_local


class TestMultipleNamespacesAcrossFiles:
    """Tests for files defining different namespaces."""

    def test_different_namespaces_across_files(self, tmp_path):
        _write_files(str(tmp_path), {
            "main.flatdata": '''
import "other.flatdata";
namespace a{ struct S { f : u8 : 8; } }
''',
            "other.flatdata": '''
namespace b{ struct T { g : u8 : 8; } }
'''
        })
        tree = build_ast_from_file(str(tmp_path / "main.flatdata"))

        structs = list(tree.root.iterate(Structure))
        names = {s.name for s in structs}
        assert "S" in names
        assert "T" in names

        s_node = next(s for s in structs if s.name == "S")
        t_node = next(s for s in structs if s.name == "T")
        assert s_node.is_local
        assert not t_node.is_local
