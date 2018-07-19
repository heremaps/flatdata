from generator.tree.errors import UnexpectedResourceType
from generator.tree.nodes.node import Node
import generator.tree.nodes.resources as resources


class _ResourceFactory(object):
    @staticmethod
    def _create_resource_object(properties):
        type = properties.type
        if 'vector' in type:
            cls = resources.Vector
        elif 'multivector' in type:
            cls = resources.Multivector
        elif 'object' in type:
            cls = resources.Instance
        elif 'raw_data' in type:
            cls = resources.RawData
        elif 'archive' in type:
            cls = resources.Archive
        else:
            raise UnexpectedResourceType(properties.type)

        assert issubclass(cls, resources.ResourceBase)
        return cls.create(properties=properties)

    @staticmethod
    def create_resource(properties):
        result = _ResourceFactory._create_resource_object(properties=properties)
        for r in result.create_references():
            result.insert(r)
        return result


class Archive(Node):
    def __init__(self, name, properties=None):
        super(Archive, self).__init__(name=name, properties=properties)

    @staticmethod
    def create(properties, definition):
        result = Archive(name=properties.name, properties=properties)
        for start, r, end in properties.resources:
            result.insert(_ResourceFactory.create_resource(r))

        for d in properties.decorations:
            if "bound_implicitly" in d:
                bound = resources.BoundResource.create(properties=d.bound_implicitly)
                for ref in bound.create_references():
                    bound.insert(ref)
                result.insert(bound)
        return result

    @property
    def resources(self):
        return self.children_like(resources.ResourceBase)
