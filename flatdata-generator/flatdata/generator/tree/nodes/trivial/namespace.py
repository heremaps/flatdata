'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from flatdata.generator.tree.nodes.node import Node


class Namespace(Node):
    def __init__(self, name, properties=None):
        super(Namespace, self).__init__(name=name, properties=properties)
