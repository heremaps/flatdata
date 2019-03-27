'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from abc import ABCMeta, abstractmethod
from jinja2 import Environment, PackageLoader
from jinja2 import nodes
from jinja2.ext import Extension
from jinja2.exceptions import TemplateRuntimeError

from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.nodes.trivial import Structure, Enumeration, Constant, Namespace
from flatdata.generator.tree.nodes.resources import ResourceBase, BoundResource, Archive as \
    ArchiveResource, Vector, Multivector, Instance, RawData
from flatdata.generator.tree.syntax_tree import SyntaxTree
from flatdata.generator.tree.traversal import DfsTraversal


class BaseGenerator(metaclass=ABCMeta):
    """Abstract base class for Flatdata generators"""

    def __init__(self, template):
        self._template = template

    @abstractmethod
    def supported_nodes(self):
        """List of supported nodes by this generator"""
        raise RuntimeError(
            "Derived generators must implement _supported_nodes")

    @abstractmethod
    def _populate_environment(self, env):
        raise RuntimeError(
            "Derived generators must implement _populate_filters")

    def render(self, tree):
        """Generate the language implementation from the AST"""
        env = Environment(loader=PackageLoader('flatdata.generator', 'templates'), lstrip_blocks=True,
                          trim_blocks=True, autoescape=False, extensions=[RaiseExtension])
        env.filters['is_archive'] = lambda n: isinstance(n, Archive)
        env.filters['is_instance'] = lambda n: isinstance(n, Instance)
        env.filters['is_raw_data'] = lambda n: isinstance(n, RawData)
        env.filters['is_archive_resource'] = lambda n: isinstance(
            n, ArchiveResource)
        env.filters['is_structure'] = lambda n: isinstance(n, Structure)
        env.filters['is_enumeration'] = lambda n: isinstance(n, Enumeration)
        env.filters['is_constant'] = lambda n: isinstance(n, Constant)
        env.filters['is_namespace'] = lambda n: isinstance(n, Namespace)
        env.filters['is_resource'] = lambda n: isinstance(n, ResourceBase)
        env.filters['is_bound_resource'] = lambda n: isinstance(
            n, BoundResource)
        env.filters['is_vector'] = lambda n: isinstance(n, Vector)
        env.filters['is_multivector'] = lambda n: isinstance(n, Multivector)
        env.filters['is_multivector_index'] = lambda n: (isinstance(
            n, Structure) and "_builtin.multivector" in SyntaxTree.namespace_path(n))
        env.filters['namespaces'] = SyntaxTree.namespaces
        self._populate_environment(env)
        template = env.get_template(self._template)

        flatdata_nodes = [n for n, _ in DfsTraversal(tree).dependency_order() if
                          any([isinstance(n, t) for t in self.supported_nodes()])]
        return template.render(nodes=flatdata_nodes, tree=tree)


class RaiseExtension(Extension):
    """Nicer error formatting for jinja2"""

    tags = set(['raise'])

    def parse(self, parser):
        """The first token is the line number, followed by the expression"""
        lineno = next(parser.stream).lineno
        message_node = parser.parse_expression()
        return nodes.CallBlock(
            self.call_method(name='_raise', args=[message_node], lineno=lineno), [], [], [],
            lineno=lineno
        )

    #pylint: disable=no-self-use
    def _raise(self, msg, caller):
        """Helper callback."""
        raise TemplateRuntimeError(msg)
