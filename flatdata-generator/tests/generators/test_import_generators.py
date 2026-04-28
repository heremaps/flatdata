'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import glob
import os

import pytest

from flatdata.generator.engine import Engine
from .assertions import unify_whitespace, diff


# Map of generator name → (expectation dir suffix, file extension)
GENERATORS = {
    'cpp': ('cpp_expectations', 'h'),
    'rust': ('rust_expectations', 'rs'),
    'flatdata': ('flatdata_expectations', 'flatdata'),
    'dot': ('dot_expectations', 'dot'),
    'py': ('py_expectations', 'py'),
}

BASEDIR = os.path.dirname(__file__)
TEST_DIR = os.path.normpath(os.path.join(BASEDIR, '..', '..', '..', 'test_cases', 'imports'))


def _discover_import_test_cases():
    """Discover import test case directories and their root schema files."""
    cases = []
    if not os.path.isdir(TEST_DIR):
        return cases
    for case_name in sorted(os.listdir(TEST_DIR)):
        case_dir = os.path.join(TEST_DIR, case_name)
        if not os.path.isdir(case_dir):
            continue
        # Find root schema: prefer main.flatdata, fall back to parent.flatdata (cyclic)
        for root_name in ['main.flatdata', 'parent.flatdata']:
            root_path = os.path.join(case_dir, root_name)
            if os.path.exists(root_path):
                root_stem = os.path.splitext(root_name)[0]
                cases.append((case_name, root_path, root_stem))
                break
    return cases


def _get_expectations(case_name, root_stem, generator_name):
    """Load expectation files for a given test case and generator."""
    expect_dir, ext = GENERATORS[generator_name]
    pattern = os.path.join(
        BASEDIR, expect_dir, 'imports', case_name, root_stem + '.' + ext + '*')
    expectations = []
    for path in sorted(glob.glob(pattern)):
        with open(path, 'r') as f:
            expectations.append(f.read())
    return expectations


def _get_test_params():
    """Generate (case_name, root_path, root_stem, generator_name) tuples."""
    params = []
    for case_name, root_path, root_stem in _discover_import_test_cases():
        for gen_name in GENERATORS:
            expect = _get_expectations(case_name, root_stem, gen_name)
            if expect:
                params.append(pytest.param(
                    root_path, gen_name, expect,
                    id=f"{case_name}-{gen_name}"))
    return params


@pytest.mark.parametrize("root_path,generator_name,expectations", _get_test_params())
def test_import_against_expectations(root_path, generator_name, expectations):
    """Test that import schemas generate output matching expectation snippets."""
    engine = Engine.from_file(root_path)
    output = engine.render(generator_name)
    output_unified = unify_whitespace(output)

    for expectation in expectations:
        expectation_unified = unify_whitespace(expectation)
        assert expectation_unified in output_unified, \
            "\nExpectation not found in output:\n========== DIFF ===========\n%s" % \
            diff(expectation, output)


def _get_generation_params():
    """All (case, generator) combos for smoke test — verify generation succeeds."""
    params = []
    for case_name, root_path, root_stem in _discover_import_test_cases():
        for gen_name in GENERATORS:
            params.append(pytest.param(
                root_path, gen_name,
                id=f"{case_name}-{gen_name}"))
    return params


@pytest.mark.parametrize("root_path,generator_name", _get_generation_params())
def test_import_generation_succeeds(root_path, generator_name):
    """Smoke test: all import schemas generate without errors for all backends."""
    engine = Engine.from_file(root_path)
    output = engine.render(generator_name)
    assert len(output) > 0


class TestImportSeparateCompilation:
    """Verify separate compilation behavior for C++ and Rust."""

    @pytest.mark.parametrize("case_name,root_path,root_stem",
                             _discover_import_test_cases(),
                             ids=[c[0] for c in _discover_import_test_cases()])
    def test_cpp_no_imported_struct_definitions(self, case_name, root_path, root_stem):
        """C++ output should not define structs from imported files."""
        engine = Engine.from_file(root_path)
        tree = engine.tree
        if not tree.imports:
            pytest.skip("No imports in this test case")

        output = engine.render("cpp")
        from flatdata.generator.tree.nodes.trivial import Structure
        for struct in tree.root.iterate(Structure):
            if not struct.is_local and "builtin" not in struct.path:
                # C++ structs are generated as union {name}Template
                assert f"{struct.name}Template" not in output, \
                    f"Imported struct {struct.name} should not be defined in C++ output"

    @pytest.mark.parametrize("case_name,root_path,root_stem",
                             _discover_import_test_cases(),
                             ids=[c[0] for c in _discover_import_test_cases()])
    def test_rust_no_imported_struct_definitions(self, case_name, root_path, root_stem):
        """Rust output should not define structs from imported files (outside schema strings)."""
        engine = Engine.from_file(root_path)
        tree = engine.tree
        if not tree.imports:
            pytest.skip("No imports in this test case")

        output = engine.render("rust")
        # Split out embedded schema strings (between r#"schema( and )schema"#)
        # to avoid false positives from schema definitions
        import re
        code_only = re.sub(r'r#"schema\(.*?\)schema"#', '', output, flags=re.DOTALL)

        from flatdata.generator.tree.nodes.trivial import Structure
        for struct in tree.root.iterate(Structure):
            if not struct.is_local and "builtin" not in struct.path:
                assert f"pub struct {struct.name}" not in code_only, \
                    f"Imported struct {struct.name} should not be defined in Rust output"

    @pytest.mark.parametrize("case_name,root_path,root_stem",
                             _discover_import_test_cases(),
                             ids=[c[0] for c in _discover_import_test_cases()])
    def test_flatdata_is_self_contained(self, case_name, root_path, root_stem):
        """Generated flatdata output must not contain import statements."""
        engine = Engine.from_file(root_path)
        output = engine.render("flatdata")
        assert 'import "' not in output, \
            "Generated flatdata schema must be self-contained (no imports)"
