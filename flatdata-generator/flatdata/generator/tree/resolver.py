from collections.abc import Iterable

import flatdata.generator.tree.nodes.references as refs
import flatdata.generator.tree.nodes.trivial as nodes
import flatdata.generator.tree.nodes.resources as resources
from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.nodes.node import Node
from . import errors


_RESOLVED_BASE_TYPES = (refs.TypeReference, refs.RuntimeReference, refs.ConstantReference)


def _filter_references(iterable: Iterable[str]) -> list[str]:
    return [x for x in iterable if '@' not in x]


def _resolve_in_parent_scope(ref: refs.Reference) -> bool:
    if ref.parent is None or ref.parent.parent is None:
        return False
    scope = ref.parent
    while scope.parent is not None and not isinstance(scope.parent, nodes.Namespace):
        scope = scope.parent
        symbol = scope.get_relative(ref.target)
        if symbol is None:
            continue
        ref.update_reference(symbol.path)
        return True
    return False


def _resolve_in_parent_namespace(ref: refs.Reference) -> bool:
    namespace = ref.first_parent_like(nodes.Namespace)
    assert namespace, "No namespace found in the tree. Unable to do name resolution"
    symbol = namespace.get_relative(ref.target)
    if symbol is None:
        return False
    ref.update_reference(symbol.path)
    return True


def _resolve_as_fully_qualified_reference(ref: refs.Reference) -> bool:
    root = ref.root
    try:
        root.find(ref.target)
    except RuntimeError:
        return False
    return True


def _validate_target_type(root: Node, ref: refs.Reference) -> None:
    expected = {
        refs.StructureReference: nodes.Structure,
        refs.ArchiveReference: Archive,
        refs.ResourceReference: resources.ResourceBase,
        refs.FieldReference: nodes.Field,
        refs.BuiltinStructureReference: nodes.Structure,
        refs.VectorReference: resources.Vector,
        refs.ConstantValueReference: nodes.Constant,
        refs.InvalidValueReference: nodes.Constant,
        refs.EnumerationReference: nodes.Enumeration
    }[type(ref)]
    target = root.find(ref.target)
    if not isinstance(target, expected) and not issubclass(type(target), expected):
        raise errors.IncorrectReferenceType(ref.name, type(target), expected)


def resolve_references(tree: Node) -> None:
    for node in tree.root.iterate():
        assert type(node) not in _RESOLVED_BASE_TYPES, "Base reference types should not be used directly"
        if any([issubclass(type(node), t) for t in _RESOLVED_BASE_TYPES]):
            assert isinstance(node, refs.Reference)
            if node.is_qualified:
                resolved = _resolve_as_fully_qualified_reference(node)
            else:
                resolved = _resolve_in_parent_scope(node) or _resolve_in_parent_namespace(node)
            if not resolved:
                raise errors.MissingSymbol(node.target, _filter_references(tree.symbols()), node)
            _validate_target_type(tree.root, node)
