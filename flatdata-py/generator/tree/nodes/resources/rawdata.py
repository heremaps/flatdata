from .base import ResourceBase


class RawData(ResourceBase):
    def __init__(self, name, properties=None):
        super(RawData, self).__init__(name=name, properties=properties)

    @staticmethod
    def create(properties):
        return RawData(name=properties.name, properties=properties)

    def create_references(self):
        return []
