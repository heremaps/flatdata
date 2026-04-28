'''
 Copyright (c) 2018 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import posixpath
import re

from jinja2 import Environment

from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.nodes.resources import (Vector, Multivector, Instance, RawData, BoundResource,
                                            Archive as ArchiveResource)
from flatdata.generator.tree.nodes.trivial import Structure, Constant, Enumeration, Namespace, Field
from flatdata.generator.tree.helpers.enumtype import EnumType
from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.syntax_tree import SyntaxTree
from . import BaseGenerator


class RustGenerator(BaseGenerator):
    """Flatdata to Rust generator"""

    RESERVED_KEYWORDS = [
        "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
        "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
        "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub",
        "pure", "ref", "return", "self", "sizeof", "static", "struct", "super", "trait", "true",
        "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield"]

    def __init__(self) -> None:
        BaseGenerator.__init__(self, "rust/rust.jinja2")

    def supported_nodes(self) -> list[type]:
        return [Structure, Archive, Constant, Enumeration]

    def filter_nodes(self, nodes: list[Node], tree: SyntaxTree) -> list[Node]:
        # Rust template traverses tree.root.children directly, not the nodes
        # list. Filtering is handled in the template via tree.is_local_node().
        return nodes

    @staticmethod
    def _import_reexports_for_namespace(ns: Node, tree: SyntaxTree) -> list[str]:
        """Return Rust pub use directives for imported types in a namespace."""
        if not tree.imports:
            return []
        # Collect source files of non-local direct children
        import_sources: set[str] = set()
        for child in ns.children:
            if not isinstance(child, Namespace) and not tree.is_local_node(child):
                if child.source_file:
                    import_sources.add(child.source_file)
        if not import_sources:
            return []
        # Build namespace path (e.g., "a::b::c")
        ns_parts: list[str] = []
        current: Node | None = ns
        while current is not None and current.parent is not None:
            ns_parts.append(current.name)
            current = current.parent
        ns_parts.reverse()
        ns_path = "::".join(ns_parts)
        # Map source files to module paths via source_file_map
        reexports: list[str] = []
        seen_modules: set[str] = set()
        for source_abs in import_sources:
            rel_path = tree.source_file_map.get(source_abs)
            if rel_path is None:
                continue
            normalized = posixpath.normpath(rel_path).replace('.flatdata', '')
            parts = normalized.split('/')
            # Each leading ".." requires an extra super:: to go up
            # one more level in the module tree
            dotdot_count = 0
            while dotdot_count < len(parts) and parts[dotdot_count] == '..':
                dotdot_count += 1
            remaining = parts[dotdot_count:]
            # super:: count:
            #   len(ns_parts) to escape the namespace module nesting
            #   + 1 to go from file-level module to its parent (sibling access)
            #   + dotdot_count for each ".." directory traversal
            super_prefix = "::".join(["super"] * (len(ns_parts) + 1 + dotdot_count))
            module_path = "::".join(remaining)
            full_path = f"{super_prefix}::{module_path}"
            if full_path not in seen_modules:
                seen_modules.add(full_path)
                reexports.append(f"pub use {full_path}::{ns_path}::*;")
        return reexports

    @staticmethod
    def _format_numeric_literal(value: str) -> str:
        try:
            # only apply this to integer values
            number = int(value)
            formatted_number = re.sub(
                r"(\d)(?=(\d{3})+(?!\d))", r"\1_", str(number))
            return formatted_number
        except ValueError:
            return value

    def _populate_environment(self, env: Environment, tree: SyntaxTree) -> None:
        env.globals["import_reexports_for_namespace"] = lambda ns: self._import_reexports_for_namespace(ns, tree)

        def _camel_to_snake_case(expr: str) -> str:
            step1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', expr)
            return re.sub('([a-z0-9])(A-Z)', r'\1_\2', step1).lower()

        env.filters["camel_to_snake_case"] = _camel_to_snake_case

        def _snake_to_upper_camel_case(expr: str) -> str:
            return ''.join(p.title() for p in expr.split('_'))

        env.filters["snake_to_upper_camel_case"] = _snake_to_upper_camel_case

        def _rust_doc(expr: str) -> str:
            lines = [
                re.sub(r'^[ \t]*(/\*\*\s?|/\*\s?|\*/|\*\s?)(.*?)\s*(\*/)?$',
                       r"/// \2", line).strip()
                for line in expr.split('\n')
            ]
            start = 0
            end = len(lines)
            if lines[0] == "///":
                start = 1
            if lines[-1] == "///":
                end = -1
            return "\n".join(lines[start:end])

        env.filters["rust_doc"] = _rust_doc

        def _escape_rust_keywords(expr: str) -> str:
            if expr in self.RESERVED_KEYWORDS:
                return "{}_".format(expr)
            return expr

        def _field_type(field: Field) -> str:
            assert field.type is not None
            if isinstance(field.type, EnumType):
                assert field.type_reference is not None
                assert field.parent is not None
                return "{}".format(
                    _fully_qualified_name(field.parent, field.type_reference.node))
            return "{}".format(field.type.name)

        def _primitive_type(field: Field) -> str:
            assert field.type is not None
            if isinstance(field.type, EnumType):
                assert field.type_reference is not None
                enum_node = field.type_reference.node
                assert isinstance(enum_node, Enumeration)
                assert enum_node.type is not None
                return "{}".format(enum_node.type.name)
            return "{}".format(field.type.name)

        def _fully_qualified_name(current: Node, node: Node) -> str:
            return "::".join((current.path_depth() - 1) * ["super"]) + str(node.path_with("::"))

        env.globals["fully_qualified_name"] = _fully_qualified_name
        env.filters["escape_rust_keywords"] = _escape_rust_keywords
        env.filters["field_type"] = _field_type
        env.filters["primitive_type"] = _primitive_type
        env.filters['structure_references'] = lambda ls: [
            x for x in ls if (isinstance(x.node, Structure)
                              and "_builtin.multivector" not in SyntaxTree.namespace_path(x.node))]
        env.filters['instance_resources'] = lambda ls: [
            x for x in ls if isinstance(x, Instance)]
        env.filters['vector_resources'] = lambda ls: [
            x for x in ls if isinstance(x, Vector)]
        env.filters['multivector_resources'] = lambda ls: [
            x for x in ls if isinstance(x, Multivector)]
        env.filters['rawdata_resources'] = lambda ls: [
            x for x in ls if isinstance(x, RawData)]
        env.filters['subarchive_resources'] = lambda ls: [
            x for x in ls if isinstance(x, ArchiveResource)]

        env.filters["is_builtin_namespace"] = lambda ns: ns.name == "_builtin"

        env.filters["supported_resources"] = lambda l: [
            x for x in l if not isinstance(x, BoundResource)]

        env.filters["format_numeric_literal"] = RustGenerator._format_numeric_literal

        env.filters["has_range"] = lambda struct: any(
            field.range for field in struct.fields)
