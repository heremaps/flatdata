'''
 Copyright (c) 2018 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''
import re

from flatdata.generator.tree.nodes.resources import (Vector, Multivector, Instance, RawData, BoundResource,
                                            Archive as ArchiveResource)
from flatdata.generator.tree.nodes.trivial import Structure, Constant, Enumeration
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

    def __init__(self):
        BaseGenerator.__init__(self, "rust/rust.jinja2")

    def supported_nodes(self):
        return [Structure, Archive, Constant, Enumeration]

    @staticmethod
    def _format_numeric_literal(value):
        try:
            # only apply this to integer values
            number = int(value)
            formatted_number = re.sub(
                r"(\d)(?=(\d{3})+(?!\d))", r"\1_", str(number))
            return formatted_number
        except ValueError:
            return value

    def _populate_environment(self, env):
        def _camel_to_snake_case(expr):
            step1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', expr)
            return re.sub('([a-z0-9])(A-Z)', r'\1_\2', step1).lower()

        env.filters["camel_to_snake_case"] = _camel_to_snake_case

        def _snake_to_upper_camel_case(expr):
            return ''.join(p.title() for p in expr.split('_'))

        env.filters["snake_to_upper_camel_case"] = _snake_to_upper_camel_case

        def _rust_doc(expr):
            lines = [
                re.sub(r'^[ \t]*(/\*\*|/\*|\*/|\*)\s*(.*?)\s*(\*/)?$',
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

        def _escape_rust_keywords(expr):
            if expr in self.RESERVED_KEYWORDS:
                return "{}_".format(expr)
            return expr

        def _field_type(field):
            if isinstance(field.type, EnumType):
                return "{}, {}".format(
                    _fully_qualified_name(field.parent, field.type_reference.node),
                    field.type_reference.node.type.name
                )
            return "{}, {}".format(field.type.name, field.type.name)

        def _fully_qualified_name(current, node):
            return "::".join((current.path_depth() - 1) * ["super"]) + node.path_with("::")

        env.globals["fully_qualified_name"] = _fully_qualified_name
        env.filters["escape_rust_keywords"] = _escape_rust_keywords
        env.filters["field_type"] = _field_type
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

        env.filters["supported_resources"] = lambda l: [
            x for x in l if not isinstance(x, BoundResource)]

        env.filters["format_numeric_literal"] = RustGenerator._format_numeric_literal
