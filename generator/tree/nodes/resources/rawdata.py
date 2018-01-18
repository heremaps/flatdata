from .base import ResourceBase


class RawData(ResourceBase):
    def __init__(self, name, properties=None, own_schema=None):
        super(RawData, self).__init__(name=name, properties=properties, own_schema=own_schema)

    @staticmethod
    def create(properties, own_schema):
        return RawData(name=properties.name, properties=properties, own_schema=own_schema)

    def _create_references(self):
        return []
