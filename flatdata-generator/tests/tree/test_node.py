import sys
import pytest

sys.path.insert(0, "..")
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.errors import SymbolRedefinition


def test_nodes_are_named():
    v = Node(name="abc")
    assert "abc" == v.name


def test_insertion_returns_parent_node():
    n = Node("foo")
    assert n == n.insert(Node("Bar"))


def test_find_child_returns_child_if_found():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert "Bar" == n.find_relative("Bar").name


def test_find_child_throws_if_not_found():
    n = Node("foo")
    pytest.raises(RuntimeError, n.find_relative, "Bar")


def test_set_name_resets_the_node_name():
    n = Node("foo")
    n.set_name("bar")
    assert "bar" == n.name


def test_erase_removes_node_from_the_tree_and_clears_its_parent():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    n.erase("Bar")
    pytest.raises(RuntimeError, n.find_relative, "Bar")
    assert s.parent == None


def test_insertion_works_with_lookup():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert "Bar" == n.find("foo.Bar").name


def test_lookup_of_nonexistent_symbol_throws():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    pytest.raises(RuntimeError, n.find, "baz")


def test_insertion_of_symbol_with_duplicate_name_trows():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    pytest.raises(SymbolRedefinition, n.insert, Node("Bar"))


def test_get_with_default_returns_value_if_found():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert s == n.get("foo.Bar", None)


def test_get_with_default_returns_default_if_not_found():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert n.get("foo.Baz", None) is None


def test_reparanting_of_symbol_trows():
    n = Node("foo")
    s = Node("Baz")
    n.insert(s)
    a = Node("bar")
    pytest.raises(RuntimeError, a.insert, s)


def test_insertion_returns_self():
    n = Node("n")
    assert n == n.insert(Node("S"))


def test_lookup_by_path():
    root = Node("foo")
    root.insert(Node("bar").insert(Node("baz").insert(Node("Dig"))))
    assert "Dig" == root.find("foo.bar.baz.Dig").name


def test_node_is_looked_up_by_its_path():
    root = Node("foo")
    s = Node("Dig")
    root.insert(Node("bar").insert(Node("baz").insert(s)))
    assert s == root.find(s.path)


def test_children_returns_children():
    n = Node("foo")
    s = Node("Bar")
    n.insert(s)
    assert [s] == n.children


def test_path_of_detached_node_is_just_its_name():
    root = Node("foo")
    assert "foo" == root.path


def test_path_contains_fully_path_with():
    root = Node("foo")
    s = Node("Dig")
    root.insert(Node("bar").insert(Node("baz").insert(s)))
    assert "foo.bar.baz.Dig" == s.path
