from flatdata.generator.tree.nodes.node import Node


class Root(Node):
    def __init__(self):
        super(Root, self).__init__(name="__root_node_name_is_empty__", properties=None)
        self._name = ""
