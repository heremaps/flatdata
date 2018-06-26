'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from jinja2 import Environment, PackageLoader

from generator.tree.nodes.archive import Archive
from generator.tree.nodes.trivial import Structure, Enumeration, Constant, Namespace
from generator.tree.nodes.resources import ResourceBase, BoundResource, Archive as \
    ArchiveResource, Vector, Multivector, Instance, RawData
from generator.tree.syntax_tree import SyntaxTree
from generator.tree.traversal import DfsTraversal
from .RaiseExtension import RaiseExtension


class BaseGenerator:
    def __init__(self, template):
        self._template = template

    def _supported_nodes(self):
        raise RuntimeError("Derived generators must reimplement _supported_nodes")

    def _populate_environment(self):
        raise RuntimeError("Derived generators must reimplement _populate_filters")

    def render(self, tree):
        env = Environment(loader=PackageLoader('generator', 'templates'), lstrip_blocks=True,
                          trim_blocks=True, autoescape=False, extensions=[RaiseExtension])
        env.filters['is_archive'] = lambda n: isinstance(n, Archive)
        env.filters['is_instance'] = lambda n: isinstance(n, Instance)
        env.filters['is_raw_data'] = lambda n: isinstance(n, RawData)
        env.filters['is_archive_resource'] = lambda n: isinstance(n, ArchiveResource)
        env.filters['is_structure'] = lambda n: isinstance(n, Structure)
        env.filters['is_enumeration'] = lambda n: isinstance(n, Enumeration)
        env.filters['is_constant'] = lambda n: isinstance(n, Constant)
        env.filters['is_namespace'] = lambda n: isinstance(n, Namespace)
        env.filters['is_resource'] = lambda n: isinstance(n, ResourceBase)
        env.filters['is_bound_resource'] = lambda n: isinstance(n, BoundResource)
        env.filters['is_vector'] = lambda n: isinstance(n, Vector)
        env.filters['is_multivector'] = lambda n: isinstance(n, Multivector)
        env.filters['is_multivector_index'] = lambda n: (isinstance(n, Structure) and
            "_builtin.multivector" in SyntaxTree.namespace_path(n))
        env.filters['namespaces'] = SyntaxTree.namespaces
        self._populate_environment(env)
        template = env.get_template(self._template)

        nodes = [n for n, _ in DfsTraversal(tree).dependency_order() if
                 any([isinstance(n, t) for t in self._supported_nodes()])]
        return template.render(nodes=nodes, tree=tree)
