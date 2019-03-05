'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import sys

from nose.tools import assert_equal, assert_equals, assert_false, assert_in, assert_is_instance, assert_raises, assert_true

sys.path.insert(0, "..")
import generator.tree.nodes.trivial as nodes
import generator.tree.nodes.resources as resources
import generator.tree.nodes.references as refs
from generator.tree.helpers.basictype import BasicType
from generator.tree.nodes.root import Root
from generator.tree.builder import _build_node_tree, _compute_structure_sizes, resolve_references, _update_field_type_references
# pylint: disable=unused-wildcard-import
from generator.tree.errors import *


def check_constant(c, typename, value):
    assert_is_instance(c, nodes.Constant)
    assert_equals(typename, c.type.name)
    assert_is_instance(c.type, BasicType)
    assert_equals(value, c.value)


def check_field(f, typename, width, offset):
    assert_is_instance(f, nodes.Field)
    assert_is_instance(f.type, BasicType)
    assert_equal(typename, f.type.name)
    assert_equal(offset, f.offset)
    assert_equal(width, f.type.width)


def check_struct(s, size_in_bits, size_in_bytes):
    assert_is_instance(s, nodes.Structure)
    assert_equal(size_in_bits, s.size_in_bits)
    assert_equal(size_in_bytes, s.size_in_bytes)


def check_explicit_reference(r, source_type, source_field, destination):
    assert_in("explicit_reference", r)
    ref = r.explicit_reference
    assert_equal(source_type, ref.source_type)
    assert_equal(source_field, ref.source_field)
    assert_equal(destination, ref.destination)


def check_bound_resource(b, name, refnames):
    assert_is_instance(b, resources.BoundResource)
    assert_equal(len(refnames), len(b.children))
    assert_equal(name, b.name)
    for i, ref in enumerate(refnames):
        refnode = b.children[i]
        assert_is_instance(refnode, refs.ResourceReference)
        assert_equal(ref, refnode.target)


def test_empty_schema_returns_none():
    tree = _build_node_tree("")
    assert_equal(set(), tree.symbols())


def test_empty_schema_with_namespace_parsed_correctly():
    tree = _build_node_tree("namespace foobar{}")
    root = tree.root
    assert_is_instance(root, Root)
    assert_is_instance(root.find(".foobar"), nodes.Namespace)


def test_empty_schema_with_nested_namespace_parsed_correctly():
    tree = _build_node_tree("namespace foo.bar.baz{}")
    for path, name in [(".foo", "foo"), (".foo.bar", "bar"), (".foo.bar.baz", "baz")]:
        node = tree.find(path)
        assert_is_instance(node, nodes.Namespace)
        assert_equal(name, node.name)
        assert_equal(path, node.path)


def test_structure_without_namespace_trows():
    with assert_raises(ParsingError):
        _build_node_tree("struct A { item : u32 : 16; }")


def test_archive_without_namespace_raises():
    with assert_raises(ParsingError):
        _build_node_tree("archive A { item : type; }")


def test_constants_are_placed_on_final_namespace():
    tree = _build_node_tree("""namespace a.b{ const i8 foo = 17;}""")
    assert_equal({".a", ".a.b", ".a.b.foo"}, tree.symbols())
    assert_is_instance(tree.find(".a.b.foo"), nodes.Constant)


def test_constants_are_parsed():
    tree = _build_node_tree("""namespace foo{
        const i8 foo = -17;
        const u16 bar = 0x42;
        const i16 foo2 = -0x20;
        const u32 bar2 = 19;
        }
        """)
    assert_equal({".foo", ".foo.bar", ".foo.foo", ".foo.foo2", ".foo.bar2"}, tree.symbols())
    check_constant(tree.find(".foo.foo"), "i8", -17)
    check_constant(tree.find(".foo.bar"), "u16", 0x42)
    check_constant(tree.find(".foo.foo2"), "i16", -0x20)
    check_constant(tree.find(".foo.bar2"), "u32", 19)


def test_duplicate_constants_raise_syntax_error():
    with assert_raises(SymbolRedefinition):
        _build_node_tree("""namespace a{
            const i8 foo = 17;
            const u16 foo = 0x42;
            }
            """)

def constant_size_check():
    with assert_raises(InvalidConstantValueError):
        _build_node_tree("""namespace a{
            const i8 foo = 128;
            }
            """)

def test_single_structure_is_parsed_correctly():
    tree = _build_node_tree("""namespace foo{
        struct Bar {
            fieldA : u8 : 2;
            fieldB : i64 : 33;
        }
        }
        """)
    _compute_structure_sizes(tree)
    assert_equal({".foo", ".foo.Bar", ".foo.Bar.fieldB", ".foo.Bar.fieldA"}, tree.symbols())

    check_struct(tree.find(".foo.Bar"), 35, 5)
    check_field(tree.find(".foo.Bar.fieldA"), "u8", 2, 0)
    check_field(tree.find(".foo.Bar.fieldB"), "i64", 33, 2)


def test_duplicate_structures_raise_syntax_error():
    with assert_raises(SymbolRedefinition):
        _build_node_tree("""namespace a{
            struct Foo { f : u8 : 2; }
            struct Foo { d : u32 : 32; }
            }
            """)


def test_two_structures_are_parsed_correctly():
    tree = _build_node_tree("""namespace foo{
        struct Bar {
            fieldA : u8 : 2;
        }
        struct Baz {
            fieldB : u32 : 17;
        }
        }
        """)
    _compute_structure_sizes(tree)
    assert_equal({".foo", ".foo.Baz", ".foo.Baz.fieldB", ".foo.Bar", ".foo.Bar.fieldA"},
                 tree.symbols())
    check_struct(tree.find(".foo.Bar"), 2, 1)
    check_struct(tree.find(".foo.Baz"), 17, 3)


def test_implicit_field_widths_are_set_correctly():
    def __test(typename, width):
        tree = _build_node_tree("""namespace n{
            struct s {
                f : %s;
            }
            }
            """ % typename)
        _compute_structure_sizes(tree)
        check_field(tree.find('.n.s.f'), typename, width, 0)

    for typename, width in [
        ('bool', 1),
        ('u8', 8),
        ('i8', 8),
        ('u16', 16),
        ('i16', 16),
        ('u32', 32),
        ('i32', 32),
        ('u64', 64),
        ('i64', 64)
    ]:
        yield __test, typename, width


def test_archives_resources_references_are_populated():
    def __test(schema, references):
        tree = _build_node_tree("""namespace foo{
            archive A {
                resourceA : %s;
            }
            }
            """ % schema)
        assert_equal({".foo", ".foo.A", ".foo.A.resourceA"}.union(references), tree.symbols())

        for r in references:
            assert_is_instance(tree.find(r), refs.TypeReference)

    for values in [
        ("T", {".foo.A.resourceA.@T"}),
        ("vector< bar.T >", {".foo.A.resourceA.@bar@T"}),
        ("multivector< 33, T, foo.T, bar.baz.Boo >",
         {".foo.A.resourceA.@T", ".foo.A.resourceA.@foo@T", ".foo.A.resourceA.@bar@baz@Boo"})
    ]:
        yield __test, values[0], values[1]


def test_archives_archive_resources_references_are_populated():
    tree = _build_node_tree("""namespace foo{
        archive A {
            resourceA : archive baz.B;
        }
        }
        """)
    assert_equal({".foo", ".foo.A", ".foo.A.resourceA", ".foo.A.resourceA.@baz@B"},
                 tree.symbols())
    assert_is_instance(tree.find(".foo.A.resourceA.@baz@B"), refs.TypeReference)


def test_multi_vector_builtin_types_are_correct():
    tree = _build_node_tree("""namespace foo{
        archive A {
            resourceA : multivector< 33, T >;
        }
        }
        """)
    res = tree.find(".foo.A.resourceA")
    assert_equal(33, res.width)
    assert_equal(1, len(res.builtins))

    index_type = res.builtins[0]
    _compute_structure_sizes(index_type)
    assert_equal({"IndexType33", "IndexType33.value"}, index_type.symbols())
    check_struct(index_type, 33, 5)
    check_field(index_type.find("IndexType33.value"), "u64", 33, 0)


def test_explicit_reference_decoration_spawns_all_the_right_references():
    tree = _build_node_tree("""namespace foo{
        archive X {
            @explicit_reference( A.refB, resourceB )
            @explicit_reference( A.refC, resourceC )
            resourceA : vector< A >;
        }
        }
        """)

    assert_equal({".foo",
                  ".foo.X",
                  ".foo.X.resourceA",
                  ".foo.X.resourceA.@A",
                  '.foo.X.resourceA.er_A_refC_resourceC',
                  '.foo.X.resourceA.er_A_refC_resourceC.@A',
                  '.foo.X.resourceA.er_A_refC_resourceC.@A@refC',
                  '.foo.X.resourceA.er_A_refC_resourceC.@resourceC',
                  '.foo.X.resourceA.er_A_refB_resourceB',
                  '.foo.X.resourceA.er_A_refB_resourceB.@A',
                  '.foo.X.resourceA.er_A_refB_resourceB.@A@refB',
                  '.foo.X.resourceA.er_A_refB_resourceB.@resourceB'
                  },
                 tree.symbols())

    r = tree.find(".foo.X.resourceA")
    assert_equal(2, len(r.decorations))
    check_explicit_reference(r.decorations[0], "A", "refB", "resourceB")
    check_explicit_reference(r.decorations[1], "A", "refC", "resourceC")

    assert_is_instance(tree.find(".foo.X.resourceA.er_A_refB_resourceB.@A@refB"),
                       refs.FieldReference)
    assert_is_instance(tree.find(".foo.X.resourceA.er_A_refB_resourceB.@resourceB"),
                       refs.ResourceReference)
    assert_is_instance(tree.find(".foo.X.resourceA.er_A_refC_resourceC.@A@refC"),
                       refs.FieldReference)
    assert_is_instance(tree.find(".foo.X.resourceA.er_A_refC_resourceC.@resourceC"),
                       refs.ResourceReference)


def test_optional_decoration():
    tree = _build_node_tree("""namespace foo{
        archive A {
            @optional
            resourceA : vector< S >;
        }
        }
        """)

    r = tree.find(".foo.A.resourceA")
    assert_equal(1, len(r.decorations))
    assert_in("optional", r.decorations[0])
    assert_true(r.optional)


def test_no_optional_decoration():
    tree = _build_node_tree("""namespace foo{
        archive A {
            resourceA : vector< S >;
        }
        }
        """)

    r = tree.find(".foo.A.resourceA")
    assert_equal(0, len(r.decorations))
    assert_false(r.optional)


def test_implicit_references_are_represented_correctly():
    tree = _build_node_tree("""namespace foo{
        struct A {
            refB : u64 : 64;
        }
        struct B { x : u64 : 64; }
        @bound_implicitly( Bound: resourceA, resourceB )
        archive X {
            resourceA : vector< A >;
            resourceB : vector< B >;
        }
        }
        """)

    b = tree.find(".foo.X.Bound")
    check_bound_resource(b, "Bound", ["resourceA", "resourceB"])


def test_multiple_archives_can_be_defined():
    tree = _build_node_tree("""namespace foo {
        struct S {
            x : u64 : 64;
        }

        archive A {
            resource1 : vector< S >;
        }

        struct F {
            x : u64 : 64;
        }

        archive B {
            resource0 : vector< F >;
            resource1 : vector< S >;
        }
        }
        """)

    assert_equal({".foo", ".foo.S", ".foo.S.x", ".foo.A", ".foo.A.resource1", ".foo.A.resource1.@S",
                  ".foo.F", ".foo.F.x", ".foo.B", ".foo.B.resource0", ".foo.B.resource0.@F",
                  ".foo.B.resource1",
                  ".foo.B.resource1.@S"},
                 tree.symbols())


def test_namespace_merging_works_for_separate_namespaces():
    tree = _build_node_tree("""
    namespace n1 {
        struct A {
            refN1 : u64 : 64;
        }
    }
    namespace n2 {
        struct A {
            refN2 : u64 : 64;
        }
    }
        """)

    assert_equal({'.n1', '.n1.A', '.n1.A.refN1',
                  '.n2', '.n2.A', '.n2.A.refN2'},
                 tree.symbols())


def test_namespace_merging_works_for_nested_namespaces():
    tree = _build_node_tree("""
    namespace n1 {
        struct A {
            refN1 : u64 : 64;
        }
    }
    namespace n1.n2 {
        struct A {
            refN2 : u64 : 64;
        }
    }
        """)

    assert_equal({'.n1', '.n1.A', '.n1.A.refN1',
                  '.n1.n2', '.n1.n2.A', '.n1.n2.A.refN2'},
                 tree.symbols())


def test_namespace_merging_works_for_same_namespaces_with_different_symbols():
    tree = _build_node_tree("""
    namespace n1.n2 {
        struct A {
            refN1 : u64 : 64;
        }
    }
    namespace n1.n2 {
        struct B {
            refN2 : u64 : 64;
        }
    }
        """)

    assert_equal({'.n1', '.n1.n2',
                  '.n1.n2.A', '.n1.n2.A.refN1',
                  '.n1.n2.B', '.n1.n2.B.refN2'},
                 tree.symbols())


def test_exceeding_field_width_results_in_an_error():
    with assert_raises(InvalidWidthError):
        _build_node_tree("""
        namespace n {
            struct A {
                f : u8 : 9;
            }
        } """)

def test_signed_enum_value_in_unsigned_enum():
    with assert_raises(InvalidSignError):
        _build_node_tree("""
        namespace n {
            enum A : u16 {
                VALUE_1 = -1
            }
        } """)

def test_not_enough_bits_for_enum_value():
    with assert_raises(InvalidEnumValueError):
        _build_node_tree("""
        namespace n {
            enum A : u8 {
                VALUE_1 = 256
            }
        } """)

def test_duplicate_enum_value():
    with assert_raises(DuplicateEnumValueError):
        _build_node_tree("""
        namespace n {
            enum A : u16 {
                VALUE_1 = 1,
                VALUE_2 = 0,
                VALUE_3
            }
        } """)

def test_not_enough_bits_in_enum_field():
    with assert_raises(InvalidEnumWidthError):
        tree = _build_node_tree("""
        namespace n {
            enum A : i16 {
                VALUE_1 = 127
            }
            struct B {
                f1 : A : 7;
            }
        } """)
        resolve_references(tree)
        _update_field_type_references(tree)

def test_enumeration():
    tree = _build_node_tree("""
    namespace n {
        enum A : u16 {
            VALUE_1,
            VALUE_2 = 4,
            VALUE_3,
            VALUE_4 = 0x10
        }
        struct B {
            f1 : A;
        }
    } """)
    resolve_references(tree)
    _update_field_type_references(tree)
    _compute_structure_sizes(tree)

    assert_equal({".n", ".n.A", ".n.A.VALUE_1", ".n.A.VALUE_2", ".n.A.VALUE_3", ".n.A.VALUE_4",
        ".n.B", ".n.B.f1", ".n.B.f1.@@n@A"}, tree.symbols())

    check_struct(tree.find(".n.B"), 16, 2)
