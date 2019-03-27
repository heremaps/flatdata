'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from pyparsing import ParseException, ParseSyntaxException

import flatdata.generator.tree.nodes.trivial as nodes
from flatdata.generator.grammar import flatdata_grammar
from flatdata.generator.tree.errors import InvalidEnumWidthError
from flatdata.generator.tree.nodes.archive import Archive
from flatdata.generator.tree.nodes.node import Node
from flatdata.generator.tree.syntax_tree import SyntaxTree
from flatdata.generator.tree.nodes.resources import Multivector
from flatdata.generator.tree.nodes.references import BuiltinStructureReference, ConstantReference, EnumerationReference
from flatdata.generator.tree.nodes.root import Root
from flatdata.generator.tree.errors import ParsingError
from flatdata.generator.tree.traversal import DfsTraversal
from flatdata.generator.tree.helpers.basictype import BasicType
from flatdata.generator.tree.helpers.enumtype import EnumType

from .resolver import resolve_references

def _create_nested_namespaces(path):
    assert not path.startswith(Node.PATH_SEPARATOR)
    splitpath = Node.splitpath(path)
    root = nodes.Namespace(name=splitpath[0])

    node = root
    for name in splitpath[1:]:
        new_node = nodes.Namespace(name=name)
        node.insert(new_node)
        node = new_node
    return root, node


def _ensure_namespace(root, path):
    assert isinstance(root, Root)
    assert path.startswith(
        Node.PATH_SEPARATOR), "This method only works with root-level paths"

    found = root.get(path)
    if found is not None:
        assert isinstance(found, nodes.Namespace)
        return found

    last_common_parent = root.find_last(path)
    first, last = _create_nested_namespaces(
        path[len(last_common_parent.path) + 1:])
    last_common_parent.insert(first)
    return last


def _innermost_namespace(root):
    if not isinstance(root, nodes.Namespace):
        return None
    namespace = root
    while namespace.children and isinstance(namespace.children[0], nodes.Namespace):
        assert len(namespace.children) == 1
        namespace = namespace.children[0]
    return namespace


def _merge_roots(roots):
    result = Root()
    for root in roots:
        innermost = _innermost_namespace(root)
        target = _ensure_namespace(
            result, Node.PATH_SEPARATOR + innermost.path)
        for child in innermost.children:
            target.insert(child.detach())
    return result


def _build_node_tree(definition):
    if not definition:
        return Root()

    try:
        parsed = flatdata_grammar.parseString(
            definition, parseAll=True).flatdata
    except (ParseException, ParseSyntaxException) as err:
        raise ParsingError(err)

    roots = []

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
                target_namespace.insert(cls.create(properties=item,
                                                   definition=definition))

        roots.append(root_namespace)

    return _merge_roots(roots)


def _append_builtin_structures(root):
    for node in root.iterate(Multivector):
        namespace = _ensure_namespace(root, "._builtin.multivector")
        for builtin in node.builtins:
            found = namespace.get_relative(builtin.name)
            if found is None:
                namespace.insert(builtin)
            found = namespace.find_relative(builtin.name)
            node.insert(BuiltinStructureReference(name=found.path))


def _append_constant_references(root):
    constants = [c for c in root.iterate(nodes.Constant)]
    archives = [a for a in root.iterate(Archive)]
    for archive in archives:
        for constant in constants:
            archive.insert(ConstantReference(constant.path))


def _update_field_type_references(root):
    for field in root.iterate(nodes.Field):
        if field.type:
            continue
        reference = field.type_reference
        if isinstance(reference, EnumerationReference):
            field.type = EnumType(name=reference.name, basic_type=BasicType(
                name=reference.node.type.name, width=reference.width))
            if reference.width and reference.width < reference.node.bits_required:
                raise InvalidEnumWidthError(enumeration_name=reference.name,
                                            width=reference.node.bits_required, provided_width=reference.width)


def _compute_structure_sizes(root):
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


def build_ast(definition):
    """Build the Flatdata syntax tree from a definition"""
    root = _build_node_tree(definition=definition)
    _append_builtin_structures(root)
    _append_constant_references(root)
    resolve_references(root)
    # now compute data based on resolved references
    _update_field_type_references(root)
    _compute_structure_sizes(root)
    return SyntaxTree(root)
