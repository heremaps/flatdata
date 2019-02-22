'''
 Copyright (c) 2018 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.tree.nodes.resources import (Vector, Multivector, Instance, RawData, BoundResource,
    Archive as ArchiveResource)
from generator.tree.nodes.trivial import Structure, Constant, Enumeration
from generator.tree.helpers.enumtype import EnumType
from generator.tree.nodes.archive import Archive
from generator.tree.syntax_tree import SyntaxTree
from .BaseGenerator import BaseGenerator

import re

class RustGenerator(BaseGenerator):

    RESERVED_KEYWORDS = [
        "abstract", "alignof", "as", "become", "box", "break", "const", "continue", "crate", "do",
        "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
        "macro", "match", "mod", "move", "mut", "offsetof", "override", "priv", "proc", "pub",
        "pure", "ref", "return", "self", "sizeof", "static", "struct", "super", "trait", "true",
        "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield"]

    def __init__(self):
        BaseGenerator.__init__(self, "rust/rust.jinja2")

    def _supported_nodes(self):
        return [Structure, Archive, Constant, Enumeration]

    def _populate_environment(self, env):
        def _camel_to_snake_case(s):
            s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', s)
            return re.sub('([a-z0-9])(A-Z)', r'\1_\2', s1).lower()

        env.filters["camel_to_snake_case"] = _camel_to_snake_case

        def _snake_to_upper_camel_case(s):
            return ''.join(p.title() for p in  s.split('_'))

        env.filters["snake_to_upper_camel_case"] = _snake_to_upper_camel_case

        def _rust_doc(s):
            lines = [
                re.sub(r'^[ \t]*(/\*\*|/\*|\*/|\*)\s*(.*?)\s*(\*/)?$', r"/// \2", line).strip()
                for line in s.split('\n')
            ]
            start = 0
            end = len(lines)
            if lines[0] == "///":
                start = 1
            if lines[-1] == "///":
                end = -1;
            return "\n".join(lines[start:end])

        env.filters["rust_doc"] = _rust_doc

        def _escape_rust_keywords(s):
            if s in self.RESERVED_KEYWORDS:
                return "{}_".format(s)
            return s

        def _field_type(f):
            if isinstance(f.type, EnumType):
                return f.type_reference.node.name + ", " + f.type_reference.node.type.name
            return f.type.name + ", " + f.type.name

        def _format_numeric_literal(value):
            try:
                value = int(value)
                value = "{:_d}".format(value)
            except ValueError:
                pass
            return value

        env.filters["escape_rust_keywords"] = _escape_rust_keywords
        env.filters["field_type"] = _field_type
        env.filters['structure_references'] = lambda ls: [
            x for x in ls if (isinstance(x.node, Structure)
                and not "_builtin.multivector" in SyntaxTree.namespace_path(x.node) )]
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

        env.filters["format_numeric_literal"] = _format_numeric_literal
