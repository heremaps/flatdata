'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import sys

sys.path.insert(0, "..")
from generator.tree.syntax_tree import SyntaxTree
from generator.tree.builder import _build_node_tree, SyntaxTreeBuilder
from generator.tree.resolver import resolve_references

from nose.tools import *


def test_archive_member_schemas_references_dependent_types():
    root = _build_node_tree("""namespace n{
        struct T { t : u8 : 7; }
        struct U { u : u8 : 7; }
        struct V { v : u8 : 7; }
        archive A {
            a : T;
            b : vector< V >;
            c : multivector< 17, U, V >;
        }
        }
        """)
    resolve_references(root)
    assert_equal(SyntaxTree.schema(root.find(".n.A.a")),
                 """namespace n {
struct T
{
    t : u8 : 7;
}
}

namespace n {
archive A
{
    a : .n.T;
}
}

""")
    assert_equal(SyntaxTree.schema(root.find(".n.A.b")),
                 """namespace n {
struct V
{
    v : u8 : 7;
}
}

namespace n {
archive A
{
    b : vector< .n.V >;
}
}

""")
    assert_equal(SyntaxTree.schema(root.find(".n.A.c")),
                 """namespace n {
struct U
{
    u : u8 : 7;
}
}

namespace n {
struct V
{
    v : u8 : 7;
}
}

namespace n {
archive A
{
    c : multivector< 17, .n.U, .n.V >;
}
}

""")


def test_archive_schema_preserves_references():
    root = _build_node_tree("""namespace foo{
/// T Comment
struct T { /* fieldA comment*/ fieldA : u8 : 7; }
/**
 * Archive comment
 */
archive A {
    /// resource comment
    resourceA : T;
}
}
        """)
    resolve_references(root)
    expected = """namespace foo {
struct T
{
    fieldA : u8 : 7;
}
}

namespace foo {
archive A
{
    resourceA : .foo.T;
}
}

"""
    assert_equal(SyntaxTree.schema(root.find(".foo.A")), expected)


def test_schemas_are_not_duplicated_if_several_type_references_occur():
    root = _build_node_tree("""namespace foo{
/// T Comment
struct T { /* fieldA comment*/ fieldA : u8 : 7; }
/**
 * Archive comment
 */
archive A {
    resourceA : T;
    resourceB : T;
}
}
        """)
    resolve_references(root)
    expected = """namespace foo {
struct T
{
    fieldA : u8 : 7;
}
}

namespace foo {
archive A
{
    resourceA : .foo.T;
    resourceB : .foo.T;
}
}

"""
    actual = SyntaxTree.schema(root.find(".foo.A"))
    assert_equal(actual, expected)


def test_archive_schemas_include_constants():
    root = SyntaxTreeBuilder.build("""namespace foo{
const u8 C = 42;
struct T { f : u8 : 7; }
archive A {
    resourceA : T;
}
}
        """)
    resolve_references(root)
    expected = """namespace foo {
struct T
{
    f : u8 : 7;
}
}

namespace foo {
const u8 C = 42;
}

namespace foo {
archive A
{
    resourceA : .foo.T;
}
}

"""
    actual = SyntaxTree.schema(root.find(".foo.A"))
    assert_equal(actual, expected)
