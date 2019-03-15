'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.generators.dot import DotGenerator
from .assertions import generate_and_assert_in


def test_folded_namespaces_are_represented_correctly():
    expected_lines = [
        """
subgraph cluster__foo
{
    penwidth=0;
    fontcolor="#516D7B";
    fillcolor="#F7F7F7";
    label=<<b>foo</b>>
subgraph cluster__foo_bar
{
    penwidth=0;
    fontcolor="#516D7B";
    fillcolor="#F7F7F7";
    label=<<b>bar</b>>
}
}
"""
    ]
    generate_and_assert_in("""
namespace foo.bar{}
    """, DotGenerator, *expected_lines)


def test_empty_schema_generates_empty_dot():
    expected_lines = [
        """
digraph FlatdataDot
{
    graph [splines=true, compound=false, rankdir=LR, rank=same, style=filled, fontsize="16", fontname="Courier New", penwidth=1, fontcolor="#516D7B"]
    node [shape=none, margin=none, fontsize=9, fontname="Courier New"];
    edge [style=solid, arrowsize=0.5, color="#257FAD", arrowtail="dot", dir="both"]
}
        """
    ]
    generate_and_assert_in("", DotGenerator, *expected_lines)


def test_structures_outside_of_archives_are_not_represented():
    unexpected_lines = [
        "_n_S"
    ]
    generate_and_assert_in("""
namespace n{
struct S {
    f : u64 : 3;
}
}
    """, DotGenerator, *[], unexpected_items=unexpected_lines)


def test_archives_are_represented_correctly():
    expected_lines = [
        """
subgraph cluster__n
{
    penwidth=0;
    fontcolor="#516D7B";
    fillcolor="#F7F7F7";
    label=<<b>n</b>>
subgraph cluster__n_A
{
    penwidth=1;
    color="#85D4FF";
    fillcolor="#EBF8FF";
    label=<<b>A</b>>
subgraph cluster__n_A_v
{
    label=<<b>v</b><br/><i>Vector</i>>
    penwidth=0;
    fontsize="9"
    fillcolor="#C4E6F8";

_n_A_v_n_S [label=<
<table border="0" cellborder="0" cellspacing="1" cellpadding="1" color="#516D7B">
    <tr>
        <td bgcolor="#257FAD">
            <b><font color="#EBF8FF">S</font></b>
        </td>
    </tr>
    <tr>
        <td bgcolor="#EBF8FF" port="port__n_A_v_n_S_f">
            <b><font color="#516D7B">f</font></b>:<font color="#568C3B">u64</font>:<font color="#D22D72">3</font>
        </td>
    </tr>
</table>>];
}
}
}
        """
    ]
    generate_and_assert_in("""
namespace n{
struct S {
    f : u64 : 3;
}
archive A {
    v : vector< S >;
}
}
    """, DotGenerator, *expected_lines)


def test_bound_resources_are_represented_correctly():
    expected_lines = [
        """
subgraph cluster__n_A
{
    penwidth=1;
    color="#85D4FF";
    fillcolor="#EBF8FF";
    label=<<b>A</b>>
subgraph cluster__n_A_b
{
    label=<<b>b</b><br/><i>BoundResource</i>>
    penwidth=0;
    fontsize="9"
    fillcolor="#C4E6F8";
_n_A_b_n_S [label=<
<table border="0" cellborder="0" cellspacing="1" cellpadding="1" color="#516D7B">
    <tr>
        <td bgcolor="#257FAD">
            <b><font color="#EBF8FF">S</font></b>
        </td>
    </tr>
    <tr>
        <td bgcolor="#EBF8FF" port="port__n_A_b_n_S_f">
            <b><font color="#516D7B">f</font></b>:<font color="#568C3B">u64</font>:<font color="#D22D72">3</font>
        </td>
    </tr>
</table>>];

_n_A_b_n_U [label=<
<table border="0" cellborder="0" cellspacing="1" cellpadding="1" color="#516D7B">
    <tr>
        <td bgcolor="#257FAD">
            <b><font color="#EBF8FF">U</font></b>
        </td>
    </tr>
    <tr>
        <td bgcolor="#EBF8FF" port="port__n_A_b_n_U_f">
            <b><font color="#516D7B">f</font></b>:<font color="#568C3B">u64</font>:<font color="#D22D72">3</font>
        </td>
    </tr>
</table>>];
}
}
}
        """
    ]
    generate_and_assert_in("""
namespace n{
struct S {
    f : u64 : 3;
}
struct U {
    f : u64 : 3;
}

@bound_implicitly(b: v1, v2)
archive A {
    v1 : vector< S >;
    v2 : vector< U >;
}
}
    """, DotGenerator, *expected_lines)


def test_explicit_references_are_generated_correctly():
    expected_lines = [
        "_n_A_v1_n_S:port__n_A_v1_n_S_f -> _n_A_v2_n_U [lhead=cluster__n_A_v2];"
    ]
    generate_and_assert_in("""
namespace n{
struct S {
    f : u64 : 3;
}
struct U {
    f : u64 : 3;
}

archive A {
    @explicit_reference(S.f, v2)
    v1 : vector< S >;
    v2 : vector< U >;
}
}
    """, DotGenerator, *expected_lines)


def test_explicit_between_bound_resources_are_generated_correctly():
    expected_lines = [
        "_n_A_bound_n_S:port__n_A_bound_n_S_f -> _n_A_v3_n_U [lhead=cluster__n_A_v3];",
        "_n_A_v3_n_U:port__n_A_v3_n_U_f -> _n_A_bound_n_S [lhead=cluster__n_A_bound]; "
    ]
    generate_and_assert_in("""
namespace n{
struct S {
    f : u64 : 3;
}
struct U {
    f : u64 : 3;
}

@bound_implicitly( bound: v1, v2)
archive A {
    @explicit_reference(S.f, v3)
    v1 : vector< S >;
    v2 : vector< U >;
    @explicit_reference(U.f, bound)
    v3 : vector< U >;
}
}
    """, DotGenerator, *expected_lines)
