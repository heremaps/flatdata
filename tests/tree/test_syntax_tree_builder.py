'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import sys

sys.path.insert(0, "..")
from generator.tree.errors import MissingSymbol
from generator.tree.builder import SyntaxTreeBuilder
from generator.tree.nodes.trivial import Namespace, Structure, Field, Constant, Enumeration, EnumerationValue
from generator.tree.nodes.archive import Archive
from generator.tree.nodes.explicit_reference import ExplicitReference
import generator.tree.nodes.resources as res
from generator.tree.nodes.resources import Vector, Multivector, RawData, Instance, BoundResource
from generator.tree.nodes.references import ResourceReference, StructureReference, \
    FieldReference, ArchiveReference, BuiltinStructureReference, VectorReference, ConstantReference, \
    EnumerationReference

from nose.tools import *


def test_validating_archive_with_no_structure_defined_raises_missing_symbol_error():
    def __test(resource_type):
        with assert_raises(MissingSymbol):
            SyntaxTreeBuilder.build(
                """namespace foo{ archive A { resourceA : %s; } }""" % resource_type)

    for t in ["T", "vector< T >", "multivector< 33, V>"]:
        yield __test, t


def test_explicit_reference_decoration_fails_when_unknown_type_is_referenced():
    with assert_raises(MissingSymbol):
        SyntaxTreeBuilder.build("""namespace foo{
            struct A {
                refB : u64 : 64;
            }
            archive R {
                @explicit_reference( C.refB, resourceB )
                resourceA : vector< A >;
                resourceB : vector< A >;
            }
            }
            """)


def test_explicit_reference_decoration_fails_when_unknown_field_is_referenced():
    with assert_raises(MissingSymbol):
        SyntaxTreeBuilder.build("""namespace foo {
            struct A {
                refB : u64 : 64;
            }
            archive R {
                @explicit_reference( A.refX, resourceB )
                resourceA : vector< A >;
                resourceB : vector< A >;
            }
            }
            """)


def test_explicit_reference_decoration_fails_when_unknown_resource_is_referenced():
    with assert_raises(MissingSymbol):
        SyntaxTreeBuilder.build("""namespace foo{
            struct A {
                refB : u64 : 64;
            }
            archive R {
                @explicit_reference( A.refB, resourceB )
                resourceA : vector< A >;
            }
            }
            """)


def test_implicit_references_fail_on_unknown_resource():
    with assert_raises(MissingSymbol):
        SyntaxTreeBuilder.build("""namespace foo{
            struct A {
                refB : u64 : 64;
            }
            @bound_implicitly( B: resourceA, resourceC )
            archive R {
                resourceA : vector< A >;
                resourceB : vector< A >;
            }
            }
            """)


def test_multi_vector_references_builtin_type():
    tree = SyntaxTreeBuilder.build("""namespace n{
        struct T { t : u64 : 17; }
        archive A {
            r : multivector< 33, T >;
        }
        }
        """)
    assert_equal({
        ".n", ".n.T", ".n.T.t", ".n.A", ".n.A.r", ".n.A.r.@@n@T",
        ".n.A.r.@@_builtin@multivector@IndexType33",
        "._builtin", "._builtin.multivector",
        "._builtin.multivector.IndexType33", "._builtin.multivector.IndexType33.value"
    }, tree.symbols())


def test_duplicate_multivector_builtin_types_are_not_produced():
    tree = SyntaxTreeBuilder.build("""namespace n{
        struct T { t : u64 : 17; }
        archive A {
            r : multivector< 33, T >;
            r2 : multivector< 33, T >;
        }
        }
        """)
    assert_equal({
        ".n", ".n.T", ".n.T.t", ".n.A",
        ".n.A.r", ".n.A.r.@@n@T", ".n.A.r.@@_builtin@multivector@IndexType33",
        ".n.A.r2", ".n.A.r2.@@n@T", ".n.A.r2.@@_builtin@multivector@IndexType33",
        "._builtin", "._builtin.multivector",
        "._builtin.multivector.IndexType33", "._builtin.multivector.IndexType33.value"
    }, tree.symbols())


TREE_WITH_ALL_FEATURES = """
namespace ns{
    struct S0 {
        f0 : u64 : 64;
        f1 : u64 : 64;
    }

    struct S1 {
        f0 : u64 : 64;
    }

    @bound_implicitly( b: A0.v0, A0.v1 )
    archive A0 {
        v0 : vector< S1 >;
        v1 : multivector< 14, S1 >;
    }

enum Enum1 : u16 {
 A = 1, B=13, C
}

struct XXX { e : Enum1; f : .ns.Enum1 : 4; }

    const u32 C = 0xFFFFFFF;

    archive A1 {
        i : S0;
        v0 : vector< S1 >;

        @optional
        v1 : vector< S1 >;

        v2 : vector< XXX >;

        @explicit_reference( .ns.S0.f0, v0 )
        @explicit_reference( S0.f1, A1.v0 )
        @explicit_reference( S0.f1, .ns.A1.v1 )
        mv : multivector< 14, S0 >;
        rd : raw_data;
        a : archive A0;
    }
} // ns
"""


def test_all_flatdata_features_look_as_expected_in_fully_built_tree():
    tree = SyntaxTreeBuilder.build(TREE_WITH_ALL_FEATURES)

    assert_equal.__self__.maxDiff = None
    assert_equal({
        '._builtin': Namespace,
        '._builtin.multivector': Namespace,
        '._builtin.multivector.IndexType14': Structure,
        '._builtin.multivector.IndexType14.value': Field,
        '.ns': Namespace,
        '.ns.A0': Archive,
        '.ns.A0.@@ns@C': ConstantReference,
        '.ns.A0.b': BoundResource,
        '.ns.A0.b.@@ns@A0@v0': ResourceReference,
        '.ns.A0.b.@@ns@A0@v1': ResourceReference,
        '.ns.A0.v0': Vector,
        '.ns.A0.v0.@@ns@S1': StructureReference,
        '.ns.A0.v1': Multivector,
        '.ns.A0.v1.@@_builtin@multivector@IndexType14': BuiltinStructureReference,
        '.ns.A0.v1.@@ns@S1': StructureReference,
        '.ns.A1': Archive,
        '.ns.A1.@@ns@C': ConstantReference,
        '.ns.A1.a': res.Archive,
        '.ns.A1.a.@@ns@A0': ArchiveReference,
        '.ns.A1.i': Instance,
        '.ns.A1.i.@@ns@S0': StructureReference,
        '.ns.A1.mv': Multivector,
        '.ns.A1.mv.@@_builtin@multivector@IndexType14': BuiltinStructureReference,
        '.ns.A1.mv.@@ns@S0': StructureReference,
        '.ns.A1.mv.er__ns_S0_f0_v0': ExplicitReference,
        '.ns.A1.mv.er__ns_S0_f0_v0.@@ns@A1@v0': ResourceReference,
        '.ns.A1.mv.er__ns_S0_f0_v0.@@ns@S0': StructureReference,
        '.ns.A1.mv.er__ns_S0_f0_v0.@@ns@S0@f0': FieldReference,
        '.ns.A1.mv.er_S0_f1_A1_v0': ExplicitReference,
        '.ns.A1.mv.er_S0_f1_A1_v0.@@ns@A1@v0': ResourceReference,
        '.ns.A1.mv.er_S0_f1_A1_v0.@@ns@S0': StructureReference,
        '.ns.A1.mv.er_S0_f1_A1_v0.@@ns@S0@f1': FieldReference,
        '.ns.A1.mv.er_S0_f1__ns_A1_v1': ExplicitReference,
        '.ns.A1.mv.er_S0_f1__ns_A1_v1.@@ns@A1@v1': ResourceReference,
        '.ns.A1.mv.er_S0_f1__ns_A1_v1.@@ns@S0': StructureReference,
        '.ns.A1.mv.er_S0_f1__ns_A1_v1.@@ns@S0@f1': FieldReference,
        '.ns.A1.rd': RawData,
        '.ns.A1.v0': Vector,
        '.ns.A1.v0.@@ns@S1': StructureReference,
        '.ns.A1.v1': Vector,
        '.ns.A1.v1.@@ns@S1': StructureReference,
        '.ns.A1.v2': Vector,
        '.ns.A1.v2.@@ns@XXX': StructureReference,
        '.ns.C': Constant,
        '.ns.S0': Structure,
        '.ns.S0.f0': Field,
        '.ns.S0.f1': Field,
        '.ns.S1': Structure,
        '.ns.S1.f0': Field,
        '.ns.Enum1': Enumeration,
        '.ns.Enum1.A': EnumerationValue,
        '.ns.Enum1.B': EnumerationValue,
        '.ns.Enum1.C': EnumerationValue,
        '.ns.XXX': Structure,
        '.ns.XXX.e': Field,
        '.ns.XXX.e.@@ns@Enum1': EnumerationReference,
        '.ns.XXX.f': Field,
        '.ns.XXX.f.@@ns@Enum1': EnumerationReference,
    }, tree.symbols(include_types=True))


def test_tree_with_all_features_schema_results_in_the_same_tree():
    expected_tree = SyntaxTreeBuilder.build(TREE_WITH_ALL_FEATURES)
    schema = expected_tree.schema(expected_tree.find('.ns.A1'))
    actual_tree = SyntaxTreeBuilder.build(schema)
    assert_equal(expected_tree.symbols(include_types=True), actual_tree.symbols(include_types=True))


def test_resource_types_are_populated_from_structure_references():
    def __test(schema, resource_type, properties):
        tree = SyntaxTreeBuilder.build("""namespace n{
            struct T {
                f0 : u8 : 1;
            }
            archive A {
                r : %s;
            }
            }
            """ % schema)

        a = tree.find(".n.A")
        assert_is_instance(a, Archive)
        r = a.find("A.r")
        assert_is_instance(r, resource_type)

        for k, values in properties.items():
            assert_true(hasattr(r, k))
            assert_equal([tree.find(v) for v in values], getattr(r, k))

    for values in [
        ("T", res.Instance, {"referenced_structures": [".n.A.r.@@n@T"]}),
        ("vector< T >", res.Vector, {"referenced_structures": [".n.A.r.@@n@T"]}),
        ("multivector< 33, T>", res.Multivector, {
            "referenced_structures": ['.n.A.r.@@_builtin@multivector@IndexType33',
                                      '.n.A.r.@@n@T']}),
        ("raw_data", res.RawData, {"referenced_structures": []})
    ]:
        yield __test, values[0], values[1], values[2]


def test_constants_are_referred_from_every_archive():
    tree = SyntaxTreeBuilder.build("""
namespace m {
    const u8 C = 17;
    }
namespace n.m {
    const u8 C = 13;
    }
namespace n{
    const u8 C = 42;

    struct T { f : u8 : 7; }
    archive A {
        resourceA : T;
    }
}""")
    tree.find(".n.A.@@n@C")
    tree.find(".n.A.@@m@C")
    tree.find(".n.A.@@n@m@C")
