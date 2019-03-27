'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from flatdata.generator.tree.nodes.references import TypeReference
from flatdata.generator.tree.nodes.trivial import Namespace
from flatdata.generator.tree.nodes.resources import ResourceBase, BoundResource
from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.nodes.references import ResourceReference

class SyntaxTree:
    """
    Flatdata Syntax Tree.
    Wraps a root node and adds number of operations like:
     - Reference-aware tree traversal
     - Schema resolution
    """

    def __init__(self, root):
        self._root = root

    @property
    def root(self):
        """
        Returns root node of the tree
        """
        return self._root

    def symbols(self, include_types=False):
        """
        Returns tree symbols
        """
        return self._root.symbols(include_types=include_types)

    def find(self, path):
        """
        Returns node at path
        """
        return self._root.find(path)

    def subtree(self, path):
        """
        Returns subtree of the given tree as a SyntaxTree
        """
        return SyntaxTree(self._root.find(path))

    def __repr__(self):
        result = []
        for item in self._root.iterate():
            result.append("    " * sum(1 for _ in item.parents()) + str(item))
        return '\n'.join(result)

    @staticmethod
    def dependent_types(node):
        def _unique(sequence):
            seen = set()
            return [item for item in sequence if not (item in seen or seen.add(item))]

        nodes = _unique([r.node for r in node.iterate(TypeReference)])
        for dependent_type in [SyntaxTree.dependent_types(n) for n in nodes]:
            nodes.extend(dependent_type)
        return _unique(nodes)

    @staticmethod
    def schema(node):
        from ..generators.flatdata import FlatdataGenerator
        generator = FlatdataGenerator()

        # extract subtree from syntax tree
        subtree = node.extract_subtree()
        return generator.render(subtree)

    @staticmethod
    def namespaces(node):
        """
        Returns parent namespace nodes for the given node in order of nesting starting with root.
        """
        return reversed([p for p in node.parents() if isinstance(p, Namespace)])

    @staticmethod
    def namespace_path(node, sep="."):
        """
        Returns namespace-qualified path for a given node with a given separator
        """
        return sep.join([n.name for n in SyntaxTree.namespaces(node)])

    @staticmethod
    def is_bound_implicitly(node):
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
    def binding_resources(node):
        if not isinstance(node, ResourceBase) or node.parent is None:
            return []

        assert isinstance(node.parent, Archive)
        archive = node.parent
        bound_resources = archive.children_like(BoundResource)
        result = []
        for resource in bound_resources:
            if any([c.node == node for c in resource.children_like(ResourceReference)]):
                result.append(resource)
        return result

    @staticmethod
    def binding_resources_or_self(node):
        if SyntaxTree.is_bound_implicitly(node):
            return SyntaxTree.binding_resources(node)
        return [node]
