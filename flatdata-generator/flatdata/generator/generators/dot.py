'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from flatdata.generator.tree.nodes.archive import Archive
from . import BaseGenerator

SCOPE_SEPARATOR = "__"
DECORATION_BOUND = "__bound__"


class DotGenerator(BaseGenerator):
    """Flatdata to DOT (graph description language) generator"""

    def __init__(self):
        BaseGenerator.__init__(self, "dot/dot.jinja2")

    def _populate_environment(self, env):
        env.autoescape = True

    def supported_nodes(self):
        return [Archive]
