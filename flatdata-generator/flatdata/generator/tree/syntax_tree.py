'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
from __future__ import annotations

from collections.abc import Iterator, Sequence
from typing import Any

from flatdata.generator.tree.nodes.references import TypeReference
from flatdata.generator.tree.nodes.trivial import Namespace
from flatdata.generator.tree.nodes.resources import ResourceBase, BoundResource
from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.references import ResourceReference
from flatdata.generator.tree.nodes.root import Root

class SyntaxTree:
    """
    Flatdata Syntax Tree.
    Wraps a root node and adds number of operations like:
     - Reference-aware tree traversal
     - Schema resolution
    """

    def __init__(self, root: Root | Node) -> None:
        self._root = root

    @property
    def root(self) -> Root | Node:
        """
        Returns root node of the tree
        """
        return self._root

    def symbols(self, include_types: bool = False) -> set[str] | dict[str, type]:
        """
        Returns tree symbols
        """
        return self._root.symbols(include_types=include_types)

    def find(self, path: str) -> Node:
        """
        Returns node at path
        """
        return self._root.find(path)

    def get(self, path: str, default: Node | None = None) -> Node | None:
        """
        Returns the node like find() does, but allows default value specification.
        """
        return self._root.get(path, default)

    def subtree(self, path: str) -> SyntaxTree:
        """
        Returns subtree of the given tree as a SyntaxTree
        """
        return SyntaxTree(self._root.find(path))

    def __repr__(self) -> str:
        result: list[str] = []
        for item in self._root.iterate():
            result.append("    " * sum(1 for _ in item.parents()) + str(item))
        return '\n'.join(result)

    @staticmethod
    def dependent_types(node: Node) -> list[Node]:
        def _unique(sequence: list[Node]) -> list[Node]:
            seen: set[Node] = set()
            return [item for item in sequence if not (item in seen or seen.add(item))]  # type: ignore[func-returns-value]  # intentional idiom: set.add() returns None (falsy) to deduplicate while preserving order

        nodes = _unique([r.node for r in node.iterate(TypeReference)])
        for dependent_type in [SyntaxTree.dependent_types(n) for n in nodes]:
            nodes.extend(dependent_type)
        return _unique(nodes)

    @staticmethod
    def schema(node: Node) -> str:
        from ..generators.flatdata import FlatdataGenerator
        generator = FlatdataGenerator()

        # extract subtree from syntax tree
        subtree = node.extract_subtree()
        return str(generator.render(subtree))

    @staticmethod
    def namespaces(node: Node) -> Iterator[Node]:
        """
        Returns parent namespace nodes for the given node in order of nesting starting with root.
        """
        return reversed([p for p in node.parents() if isinstance(p, Namespace)])

    @staticmethod
    def namespace_path(node: Node, sep: str = ".") -> str:
        """
        Returns namespace-qualified path for a given node with a given separator
        """
        return sep.join([n.name for n in SyntaxTree.namespaces(node)])

    @staticmethod
    def is_bound_implicitly(node: Node) -> bool:
        if not isinstance(node, ResourceBase) or node.parent is None:
            return False

        assert isinstance(node.parent, Archive)
        archive = node.parent
        bound_resources = archive.children_like(BoundResource)
        for resource in bound_resources:
            if any([c.node == node for c in resource.children_like(ResourceReference)]):
                return True
        return False

    @staticmethod
    def binding_resources(node: Node) -> list[BoundResource]:
        if not isinstance(node, ResourceBase) or node.parent is None:
            return []

        assert isinstance(node.parent, Archive)
        archive = node.parent
        bound_resources = archive.children_like(BoundResource)
        result: list[BoundResource] = []
        for resource in bound_resources:
            if any([c.node == node for c in resource.children_like(ResourceReference)]):
                result.append(resource)
        return result

    @staticmethod
    def binding_resources_or_self(node: Node) -> Sequence[ResourceBase | BoundResource]:
        if SyntaxTree.is_bound_implicitly(node):
            return SyntaxTree.binding_resources(node)
        assert isinstance(node, ResourceBase)
        return [node]
