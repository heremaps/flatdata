'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.tree.nodes.trivial import Structure
from generator.tree.nodes.archive import Archive
from .BaseGenerator import BaseGenerator

SCOPE_SEPARATOR = "__"
DECORATION_BOUND = "__bound__"


class DotGenerator(BaseGenerator):
    def __init__(self):
        BaseGenerator.__init__(self, "dot/dot.jinja2")

    def _populate_environment(self, env):
        env.autoescape = True

    def _supported_nodes(self):
        return [Archive]
