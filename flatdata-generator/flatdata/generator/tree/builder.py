'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from pyparsing import ParseException, ParseSyntaxException

import flatdata.generator.tree.nodes.trivial as nodes
from flatdata.generator.grammar import flatdata_grammar
from flatdata.generator.tree.errors import (
    InvalidEnumWidthError, InvalidRangeName, InvalidRangeReference,
    InvalidConstReference, InvalidConstValueReference, DuplicateInvalidValueReference,
    InvalidStructInExplicitReference, OptionalRange)
from flatdata.generator.tree.nodes.explicit_reference import ExplicitReference
from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.syntax_tree import SyntaxTree
from flatdata.generator.tree.nodes.resources import Multivector, Vector, ResourceBase
from flatdata.generator.tree.nodes.references import (
    BuiltinStructureReference, ConstantReference, ConstantValueReference,
    EnumerationReference, StructureReference, InvalidValueReference)
from flatdata.generator.tree.nodes.root import Root
from flatdata.generator.tree.errors import ParsingError
from flatdata.generator.tree.traversal import DfsTraversal
from flatdata.generator.tree.helpers.basictype import BasicType
from flatdata.generator.tree.helpers.enumtype import EnumType

from .resolver import resolve_references


def _create_nested_namespaces(path: str) -> tuple[nodes.Namespace, nodes.Namespace]:
    assert not path.startswith(Node.PATH_SEPARATOR)
    splitpath = Node.splitpath(path)
    root = nodes.Namespace(name=splitpath[0])

    node: nodes.Namespace = root
    for name in splitpath[1:]:
        new_node = nodes.Namespace(name=name)
        node.insert(new_node)
        node = new_node
    return root, node


def _ensure_namespace(root: Root, path: str) -> nodes.Namespace:
    assert isinstance(root, Root)
    assert path.startswith(
        Node.PATH_SEPARATOR), "This method only works with root-level paths"

    found = root.get(path)
    if found is not None:
        assert isinstance(found, nodes.Namespace)
        return found

    last_common_parent = root.find_last(path)
    assert last_common_parent is not None
    first, last = _create_nested_namespaces(
        path[len(last_common_parent.path) + 1:])
    last_common_parent.insert(first)
    return last


def _innermost_namespace(root: Node) -> nodes.Namespace | None:
    if not isinstance(root, nodes.Namespace):
        return None
    namespace = root
    while namespace.children and isinstance(namespace.children[0], nodes.Namespace):
        assert len(namespace.children) == 1
        namespace = namespace.children[0]
    return namespace


def _merge_roots(roots: list[nodes.Namespace]) -> Root:
    result = Root()
    for root in roots:
        innermost = _innermost_namespace(root)
        assert innermost is not None
        target = _ensure_namespace(
            result, Node.PATH_SEPARATOR + innermost.path)
        for child in innermost.children:
            target.insert(child.detach())
    return result


def _build_node_tree(definition: str) -> Root:
    if not definition:
        return Root()

    try:
        parsed = flatdata_grammar.parseString(
            definition, parseAll=True).flatdata
    except (ParseException, ParseSyntaxException) as err:
        raise ParsingError(err)

    roots: list[nodes.Namespace] = []

    for namespace in parsed.namespace:
        root_namespace, target_namespace = _create_nested_namespaces(
            namespace.name)

        parsed_items = [
            (namespace.constants, nodes.Constant),
            (namespace.structures, nodes.Structure),
            (namespace.enumerations, nodes.Enumeration),
            (namespace.archives, Archive)
        ]

        for collection, cls in parsed_items:
            for item in collection:
                target_namespace.insert(cls.create(properties=item,  # type: ignore[attr-defined]  # subclasses (Structure, Enumeration, Archive) define create()
                                                   definition=definition))

        roots.append(root_namespace)

    return _merge_roots(roots)


def _append_builtin_structures(root: Root) -> None:
    multivectors = list(root.iterate(Multivector))
    for node in multivectors:
        assert node.parent is not None and node.parent.parent is not None
        namespace = _ensure_namespace(root, node.parent.parent.path + "._builtin.multivector")
        for builtin in node.builtins:
            found = namespace.get_relative(builtin.name)
            if found is None:
                namespace.insert(builtin)
            found = namespace.find_relative(builtin.name)
            node.insert(BuiltinStructureReference(name=found.path))


def _append_constant_references(root: Root) -> None:
    constants = [c for c in root.iterate(nodes.Constant)]
    constant_references = set(c.target for c in root.iterate(ConstantReference))
    archives = [a for a in root.iterate(Archive)]
    for archive in archives:
        for constant in constants:
            if not constant.path in constant_references:
                archive.insert(ConstantValueReference(constant.path))


def _update_field_type_references(root: Root) -> None:
    for field in root.iterate(nodes.Field):
        if field.type:
            continue
        reference = field.type_reference
        if isinstance(reference, EnumerationReference):
            enum_node = reference.node  # resolves to Enumeration at runtime
            field.type = EnumType(name=reference.name, basic_type=BasicType(
                name=enum_node.type.name, width=enum_node.type.width))  # type: ignore[attr-defined]  # .node resolves to Enumeration which has .type
            if reference.width and reference.width != enum_node.type.width:  # type: ignore[attr-defined]
                raise InvalidEnumWidthError(enumeration_name=reference.name,
                                            width=enum_node.type.width, provided_width=reference.width)  # type: ignore[attr-defined]


def _compute_structure_sizes(root: Root) -> None:
    # visit structs in the correct order. Not important right now,
    # but will make it very easy to add structs as fields in other structs
    for struct, _ in DfsTraversal(root).dependency_order():
        if not isinstance(struct, nodes.Structure):
            continue

        offset = 0
        for field in struct.children:
            if not isinstance(field, nodes.Field):
                continue
            field.offset = offset
            offset += int(field.type.width)
        struct.size_in_bits = offset

def _compute_max_resource_size(root: Root) -> None:
    # visit all explicit references and check how many bits they have available
    # the provides an upper bound on resource size
    for reference in root.iterate(ExplicitReference):
        field_node = reference.field.node  # resolves to Structure/Field at runtime
        if field_node.type.width == 64:  # type: ignore[attr-defined]  # .node resolves to a node with .type
            continue
        ref_limit = 2 ** field_node.type.width  # type: ignore[attr-defined]
        dest_node = reference.destination.node  # resolves to a resource node at runtime
        max_size = dest_node.max_size  # type: ignore[attr-defined]  # .node resolves to ResourceBase which has .max_size
        dest_node.max_size = ref_limit if max_size is None or max_size > ref_limit else max_size  # type: ignore[attr-defined]

def _check_ranges(root: Root) -> None:
    # First check that names are unique
    for field in root.iterate(nodes.Field):
        name = field.range
        if not name:
            continue
        assert field.parent is not None
        for sibling in field.parent.fields:  # type: ignore[attr-defined]  # parent is a Structure which has .fields
            if sibling.name == name:
                raise InvalidRangeName(name)
        # Also check that the range is not optional
        if field.invalid_value:
            raise OptionalRange(name)

    # Now check that structs with ranges are only used in vectors
    for reference in root.iterate(StructureReference):
        if (isinstance(reference.node, nodes.Structure) and reference.node.has_range
                and isinstance(reference.parent, ResourceBase) and not isinstance(reference.parent, Vector)):
            raise InvalidRangeReference(reference.target)

def _check_const_refs(root: Root) -> None:
    for field in root.iterate(nodes.Field):
        for ref in field.children_like(ConstantReference):
            const_node = ref.node  # resolves to Constant at runtime
            # Check that type matches
            if const_node.type.name != field.type.name:  # type: ignore[attr-defined]  # .node resolves to Constant which has .type
                raise InvalidConstReference(ref.target, const_node.type.name)  # type: ignore[attr-defined]
            # Check that value fits into field
            if const_node.type.bits_required(const_node.value) > field.type.width:  # type: ignore[attr-defined]  # Constant has .type and .value
                raise InvalidConstValueReference(ref.target, field.type.width)
        invalid_values = field.children_like(InvalidValueReference)
        if len(invalid_values) > 1:
            raise DuplicateInvalidValueReference(field.name, [x.target for x in invalid_values])

def _check_explicit_references(root: Root) -> None:
    for reference in root.iterate(ExplicitReference):
        for ref in reference.children_like(StructureReference):
            assert reference.parent is not None
            if not ref.target in [x.target for x in reference.parent.children_like(StructureReference)]:
                raise InvalidStructInExplicitReference(ref.node.name, reference.parent.name)

def build_ast(definition: str) -> SyntaxTree:
    """Build the Flatdata syntax tree from a definition"""
    root = _build_node_tree(definition=definition)
    _append_builtin_structures(root)
    resolve_references(root)
    _append_constant_references(root)
    _check_ranges(root)
    # now compute data based on resolved references
    _update_field_type_references(root)
    _compute_structure_sizes(root)
    _compute_max_resource_size(root)
    _check_const_refs(root)
    _check_explicit_references(root)
    return SyntaxTree(root)
