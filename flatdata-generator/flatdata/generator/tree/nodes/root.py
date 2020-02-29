from flatdata.generator.tree.nodes.node import Node


class Root(Node):
    def __init__(self, properties=None):
        super().__init__(name="__root_node_name_is_empty__", properties=properties)
        self._name = ""

    @property
    def doc(self):
        return self._properties.doc
