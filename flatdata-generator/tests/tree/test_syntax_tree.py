'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import sys

from nose.tools import assert_equal, assert_raises

sys.path.insert(0, "..")
from flatdata.generator.tree.syntax_tree import SyntaxTree
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.trivial import Structure
import flatdata.generator.tree.nodes.references as refs
from flatdata.generator.tree.errors import CircularReferencing
from flatdata.generator.tree.traversal import BfsTraversal, DfsTraversal


def create_basic_tree():
    return SyntaxTree(
        Node("a").insert(Node("b").insert(Node("c"), Node("d"), Node("e")),
                         Node("f").insert(
                             Node("g").insert(
                                 Node("h"))))
    )


def create_tree_with_type_refs():
    return SyntaxTree(
        Node("a").insert(Node("b").insert(Node("d"), refs.TypeReference("a.c.e")),
                         Node("c").insert(Node("e"), refs.TypeReference("a.b.d")))
    )


def create_tree_with_cycle():
    return SyntaxTree(
        Node("a").insert(Node("b").insert(Structure("c").insert(refs.TypeReference("a.d"))),
                         Node("d").insert(refs.TypeReference("a.b")))
    )


def create_tree_with_topological_ordering():
    return SyntaxTree(
        Node("ns").insert(Node("S0"),
                          Node("A0").insert(Node("R0").insert(refs.TypeReference("ns.A1")),
                                            Node("R1").insert(refs.TypeReference("ns.S0"))),
                          Node("A1"))
    )


def _check_traversal_order(tree, traversal, expected_paths, **kwargs):
    nodes = []
    attrs = []
    for node, attr in traversal(tree).iterate():
        nodes.append(node)
        attrs.append(attr)
    expected_nodes = [tree.find(n) for n in expected_paths]
    assert_equal(expected_paths, [n.path for n in nodes])
    assert_equal(expected_nodes, [n for n in nodes])

    for arg, value in kwargs.items():
        assert_equal(value, [getattr(v, arg) for v in attrs])


def test_bfs_traversal_basic_ordering():
    _check_traversal_order(create_basic_tree(), BfsTraversal, [
        "a",
        "a.b",
        "a.f",
        "a.b.c",
        "a.b.d",
        "a.b.e",
        "a.f.g",
        "a.f.g.h"
    ], distance=[0, 1, 1, 2, 2, 2, 2, 3])


def test_dfs_traversal_basic_ordering():
    _check_traversal_order(create_basic_tree(), DfsTraversal, [
        "a",
        "a.b",
        "a.b.c",
        "a.b.d",
        "a.b.e",
        "a.f",
        "a.f.g",
        "a.f.g.h"
    ])


def test_dfs_traversal_expands_type_references():
    _check_traversal_order(create_tree_with_type_refs(), DfsTraversal, [
        "a",
        "a.b",
        "a.b.d",
        "a.c.e",
        "a.c"
    ])


def test_bfs_traversal_expands_type_references():
    _check_traversal_order(create_tree_with_type_refs(), BfsTraversal, [
        "a",
        "a.b",
        "a.c",
        "a.b.d",
        "a.c.e"
    ])


def test_dfs_traversal_reports_cyclic_references():
    with assert_raises(CircularReferencing):
        for _, _ in DfsTraversal(create_tree_with_cycle()).iterate():
            pass


def test_dfs_traversal_provides_topological_ordering():
    assert_equal(
        [
            "ns.S0",
            "ns.A1",
            "ns.A0.R0",
            "ns.A0.R1",
            "ns.A0",
            "ns"
        ],
        [n.path for n, _ in
         DfsTraversal(create_tree_with_topological_ordering()).dependency_order()])
