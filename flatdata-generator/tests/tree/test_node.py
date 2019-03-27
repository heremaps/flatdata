import sys

sys.path.insert(0, "..")
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.errors import SymbolRedefinition
from nose.tools import assert_equal, assert_is_none, assert_raises


def test_nodes_are_named():
    v = Node(name="abc")
    assert_equal("abc", v.name)


def test_insertion_returns_parent_node():
    n = Node("foo")
    assert_equal(n, n.insert(Node("Bar")))


def test_find_child_returns_child_if_found():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert_equal("Bar", n.find_relative("Bar").name)


def test_find_child_throws_if_not_found():
    n = Node("foo")
    assert_raises(RuntimeError, n.find_relative, "Bar")


def test_set_name_resets_the_node_name():
    n = Node("foo")
    n.set_name("bar")
    assert_equal("bar", n.name)


def test_erase_removes_node_from_the_tree_and_clears_its_parent():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    n.erase("Bar")
    assert_raises(RuntimeError, n.find_relative, "Bar")
    assert_is_none(s.parent)


def test_insertion_works_with_lookup():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert_equal("Bar", n.find("foo.Bar").name)


def test_lookup_of_nonexistent_symbol_throws():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert_raises(RuntimeError, n.find, "baz")


def test_insertion_of_symbol_with_duplicate_name_trows():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert_raises(SymbolRedefinition, n.insert, Node("Bar"))


def test_get_with_default_returns_value_if_found():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert_equal(s, n.get("foo.Bar", None))


def test_get_with_default_returns_default_if_not_found():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert_is_none(n.get("foo.Baz", None))


def test_reparanting_of_symbol_trows():
    n = Node("foo")
    s = Node("Baz")
    n.insert(s)
    a = Node("bar")
    assert_raises(RuntimeError, a.insert, s)


def test_insertion_returns_self():
    n = Node("n")
    assert_equal(n, n.insert(Node("S")))


def test_lookup_by_path():
    root = Node("foo")
    root.insert(Node("bar").insert(Node("baz").insert(Node("Dig"))))
    assert_equal("Dig", root.find("foo.bar.baz.Dig").name)


def test_node_is_looked_up_by_its_path():
    root = Node("foo")
    s = Node("Dig")
    root.insert(Node("bar").insert(Node("baz").insert(s)))
    assert_equal(s, root.find(s.path))


def test_children_returns_children():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert_equal([s], n.children)


def test_path_of_detached_node_is_just_its_name():
    root = Node("foo")
    assert_equal("foo", root.path)


def test_path_contains_fully_path_with():
    root = Node("foo")
    s = Node("Dig")
    root.insert(Node("bar").insert(Node("baz").insert(s)))
    assert_equal("foo.bar.baz.Dig", s.path)
