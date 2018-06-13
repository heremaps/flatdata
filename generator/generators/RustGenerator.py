'''
 Copyright (c) 2018 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.tree.nodes.resources import (Vector, Multivector, Instance, RawData, BoundResource,
    Archive as ArchiveResource)
from generator.tree.nodes.trivial import Structure, Constant
from generator.tree.nodes.archive import Archive
from .BaseGenerator import BaseGenerator

import re

class RustGenerator(BaseGenerator):
    def __init__(self):
        BaseGenerator.__init__(self, "rust/rust.jinja2")

    def _supported_nodes(self):
        return [Structure, Archive, Constant]

    def _populate_environment(self, env):
        def _camel_to_snake_case(s):
            s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', s)
            return re.sub('([a-z0-9])(A-Z)', r'\1_\2', s1).lower()

        env.filters["camel_to_snake_case"] = _camel_to_snake_case

        def _snake_to_camel_case(s):
            return ''.join(p.title() for p in  s.split('_'))

        env.filters["snake_to_camel_case"] = _snake_to_camel_case

        def _rust_doc(s):
            lines = [re.sub(r'^[ \t]*(/\*\*|\*/|\*)', "///", line) for line in s.split('\n')]
            start = 0
            end = len(lines)
            if lines[0] == "///":
                start = 1
            if lines[-1] == "///":
                end = -1;
            return "\n".join(lines[start:end])

        env.filters["rust_doc"] = _rust_doc

        def _escape_rust_keywords(s):
            if s == "ref":
                return "ref_"
            if s == "type":
                return "type_"
            else:
                return s

        env.filters["escape_rust_keywords"] = _escape_rust_keywords

        env.filters['instance_resources'] = lambda ls: [
            x for x in ls if isinstance(x, Instance)]
        env.filters['vector_resources'] = lambda ls: [
            x for x in ls if isinstance(x, Vector)]
        env.filters['multivector_resources'] = lambda ls: [
            x for x in ls if isinstance(x, Multivector)]
        env.filters['rawdata_resources'] = lambda ls: [
            x for x in ls if isinstance(x, RawData)]
        env.filters['subarchive_resources'] = lambda ls: [
            x for x in ls if isinstance(x, ArchiveResource) and not x.optional]
        env.filters['optional_subarchive_resources'] = lambda ls: [
            x for x in ls if isinstance(x, ArchiveResource) and x.optional]

        env.filters["supported_resources"] = lambda l: [
            x for x in l if not isinstance(x, BoundResource)]
