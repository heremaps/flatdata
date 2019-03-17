'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
from copy import copy

from collections import OrderedDict
from flatdata.generator.tree.errors import SymbolRedefinition


class Node:
    """
    Node of a Syntax Tree.
    Every node is defined by its name and location in the tree.
    No two nodes with the same name at the same path are allowed.
    """

    PATH_SEPARATOR = '.'

    @staticmethod
    def splitpath(path):
        """
        Splits node path.
        """
        return path.split(Node.PATH_SEPARATOR)

    @staticmethod
    def jointwo(path, other):
        """
        Joins two node paths.
        """
        return Node.PATH_SEPARATOR.join([path, other])

    def __init__(self, name, properties=None):
        assert self.PATH_SEPARATOR not in name
        assert name
        self._name = name
        self._properties = properties
        self._children = OrderedDict()
        self._parent = None

    @property
    def name(self):
        """
        Returns the name of the node.
        """
        return self._name

    @property
    def children(self):
        """
        Returns a list of children nodes.
        """
        return list(self._children.values())

    def children_like(self, T):
        """
        Returns a list of children nodes of a given type, if any.
        """
        return [c for c in list(self._children.values()) if isinstance(c, T)]

    @property
    def parent(self):
        """
        Returns node's parent.
        """
        return self._parent

    def first_parent_like(self, T):
        """
        Returns first available parent of a given type or None if none found.
        """
        result = self
        while result.parent is not None and not isinstance(result.parent, T):
            result = result.parent
        return result.parent

    @property
    def path(self):
        """
        Returns nodes' path in a tree.
        """
        if self._parent is None:
            return self.name
        return Node.jointwo(self._parent.path, self.name)

    def path_with(self, separator='_'):
        """
        Returns nodes' path in a tree with a given characters as separator.
        """
        return self.path.replace(self.PATH_SEPARATOR, separator)

    def path_depth(self):
        """
        Returns nodes' depths in a tree
        """
        if self._parent is None:
            return 0
        return 1 + self._parent.path_depth()

    def set_name(self, value):
        """
        Sets the new name for the node. New name should not clash with any of siblings' names.
        :raises RuntimeError in case name is already in use
        :param value: new name
        """
        if self.name == value:
            return

        if self.parent is not None and value in self._parent._children:
            raise RuntimeError(
                "Cannot rename the node, name {value} is already in use".format(value=value))

        self._name = value
        if self.parent is not None:
            self.parent.reindex()

    def find(self, path):
        """
        Finds child node recursively by its path.
        :param path: Full path to the node up to the node search is started.
        :return: a node
        :raises: RuntimeError in case no node is found
        """
        keys = Node.splitpath(path)

        try:
            target = self
            if target.name != keys[0]:
                raise RuntimeError("Path {path} not found in tree. Options: {options}".format(
                    path=path, options=tuple(self.symbols())))

            for key in keys[1:]:
                target = target._children[key]
        except (KeyError, IndexError):
            raise RuntimeError("Path '{path}' not found in tree. Options: {options}".format(
                path=path, options=tuple(self.symbols())))
        return target

    def get(self, path, default=None):
        """
        Returns the node like find() does, but allows default value specification.
        """
        try:
            result = self.find(path)
        except RuntimeError:
            return default
        return result

    def find_relative(self, path):
        """
        Finds a child node recursively via its path relative to the current node.
        """
        return self.find(Node.jointwo(self.name, path))

    def find_last(self, path):
        """
        Finds a last node existing in the path. If no such node found, None is returned.
        """
        keys = Node.splitpath(path)
        if not keys:
            return None

        target = self
        if target.name != keys[0]:
            return None

        try:
            for key in keys[1:]:
                target = target._children[key]
        except KeyError:
            return target
        return target

    def get_relative(self, path, default=None):
        """
        Finds a child node recursively via its path relative to the current node.
        """
        return self.get(Node.jointwo(self.name, path), default)

    @property
    def root(self):
        """
        Returns the root node of the tree
        """
        result = self
        while result.parent is not None:
            result = result._parent
        return result

    def extract_subtree(self):
        """
        Extract the subtree of node (some nodes are copied)
        Also copies the path to the root of the tree
        """

        new_root = copy(self)
        while new_root._parent:
            parent = copy(new_root._parent)
            parent._children = OrderedDict()
            new_root._parent = parent
            parent._children[new_root.name] = new_root
            new_root = parent
        return new_root

    def insert(self, *nodes):
        """
        Inserts node into the tree.
        :raises: SymbolRedefinition in case node with the same name exists
        :raises: RuntimeError in case node is attempted to be reparented
        """
        for node in nodes:
            assert isinstance(node, Node), "Inserting not a node into a tree: %s" % node
            if node.name in self._children:
                raise SymbolRedefinition(duplicate=node, existing=self._children[node.name])

            if node.parent is not None:
                raise RuntimeError(
                    "Symbol {name} is already used at {path}. Reparanting occured.".format(
                        name=node.name, path=node.path))

            self._children[node.name] = node
            node._parent = self
        return self

    def erase(self, key):
        """
        Erase node with a given name from the tree.
        """
        node = self._children.pop(key)
        node._parent = None

    def reindex(self):
        """
        Reindex the node. Produces no side effects if called externally.
        """
        new_children = OrderedDict()
        for _key, node in self._children.items():
            new_children[node.name] = node
        self._children = new_children

    def iterate(self, node_type=None):
        """
        Iterates the nodes in pre-order traversal fashion
        """
        if node_type is None or isinstance(self, node_type):
            yield self
        for _, child in self._children.items():
            for node in child.iterate(node_type):
                yield node

    def parents(self):
        """
        Returns all node's parents up to the root of the tree.
        """
        par = self
        while par._parent is not None:
            yield par._parent
            par = par._parent

    def detach(self):
        """
        Detaches the node from its parent.
        """
        if self._parent is None:
            return self
        del self._parent._children[self.name]
        self._parent = None
        return self

    def symbols(self, include_types=False):
        """
        Returns paths of all nodes available in the tree, optionally with node types.
        :param include_types: return types along with paths
        """
        result = dict()
        for node in self.iterate():
            path = node.path
            if path:
                result[path] = type(node)
        if not include_types:
            return set(result.keys())
        return result

    def __repr__(self):
        return "{type}{{{path}}}".format(type=type(self).__name__, path=self.path)
