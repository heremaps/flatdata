from flatdata.generator.tree.errors import UnexpectedResourceType
from flatdata.generator.tree.nodes.node import Node
import flatdata.generator.tree.nodes.resources as resources

from typing import Any


def _create_resource(properties: Any) -> resources.ResourceBase:
    resource_type = properties.type
    cls: type[resources.ResourceBase]
    if 'vector' in resource_type:
        cls = resources.Vector
    elif 'multivector' in resource_type:
        cls = resources.Multivector
    elif 'object' in resource_type:
        cls = resources.Instance
    elif 'raw_data' in resource_type:
        cls = resources.RawData
    elif 'archive' in resource_type:
        cls = resources.Archive
    else:
        raise UnexpectedResourceType(properties.type)

    assert issubclass(cls, resources.ResourceBase)
    result = cls.create(properties=properties)
    for reference in result.create_references():
        result.insert(reference)
    return result


class Archive(Node):
    def __init__(self, name: str, properties: Any = None) -> None:
        super().__init__(name=name, properties=properties)

    #pylint: disable=unused-argument
    @staticmethod
    def create(properties: Any, definition: Any) -> 'Archive':
        result = Archive(name=properties.name, properties=properties)
        for resource in properties.resources:
            result.insert(_create_resource(resource))

        for decoration in properties.decorations:
            if "bound_implicitly" in decoration:
                bound = resources.BoundResource.create(properties=decoration.bound_implicitly)
                for ref in bound.create_references():
                    bound.insert(ref)
                result.insert(bound)
        return result

    @property
    def resources(self) -> list[resources.ResourceBase]:
        return self.children_like(resources.ResourceBase)  # type: ignore[type-abstract]  # isinstance() with ABC is valid

    @property
    def doc(self) -> str:
        doc = self._properties.doc
        return str(doc) if doc is not None else ""
