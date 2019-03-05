'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import sys

sys.path.insert(0, "..")
import generator.tree.nodes.trivial as nodes
from generator.tree.nodes.root import Root
from generator.tree.syntax_tree import SyntaxTree
import generator.tree.nodes.resources as res
from generator.tree.nodes.archive import Archive
import generator.tree.nodes.references as refs
from generator.tree.resolver import resolve_references
import generator.tree.errors as errors

from nose.tools import assert_equal, assert_is_instance, assert_raises


def _create_tree_resource_to_struct(actual, reference):
    root = Root().insert(nodes.Namespace(name="ns").insert(nodes.Structure(name=actual),
                                                           Archive(name="Archive").insert(
                                                               res.Vector(name="resource").insert(
                                                                   refs.StructureReference(
                                                                       name=reference)))))
    return SyntaxTree(root)


def _create_tree_resource_to_struct_with_extra_folding(actual, reference):
    root = Root().insert(nodes.Namespace(name="ns").insert(
        nodes.Namespace(name="fold").insert(nodes.Structure(name=actual)),
        Archive(name="Archive").insert(
            res.Vector(name="resource").insert(refs.StructureReference(name=reference)))))
    return SyntaxTree(root)


def _create_tree_resource_to_archive():
    root = Root().insert(nodes.Namespace(name="ns").insert(Archive(name="RefArchive"),
                                                           Archive(name="Archive").insert(
                                                               res.Archive(name="resource").insert(
                                                                   refs.ArchiveReference(
                                                                       name="RefArchive")))))
    return SyntaxTree(root)


def _create_tree_with_explicit_reference(name):
    root = Root().insert(nodes.Namespace(name="ns").insert(
        nodes.Structure(name="Struct").insert(nodes.Field(name="Field")),
        Archive(name="Archive").insert(
            res.Vector(name="resource").insert(refs.FieldReference(name="Struct.Field"),
                                               refs.ResourceReference(name=name)),
            res.Vector(name="resource2"))))
    return SyntaxTree(root)


def _create_tree_with_two_struct_references():
    root = Root().insert(nodes.Namespace(name="ns").insert(nodes.Structure(name="S1"),
                                                           nodes.Structure(name="S2"),
                                                           Archive(name="Archive").insert(
                                                               res.Multivector(
                                                                   name="resource").insert(
                                                                   refs.StructureReference(
                                                                       name="S1"),
                                                                   refs.StructureReference(
                                                                       name="S2")))))
    return SyntaxTree(root)


def _assert_missing_symbol_is_thrown(root, message):
    try:
        resolve_references(root)
    except errors.MissingSymbol as e:
        assert_equal(message, str(e))
    else:
        assert_false(True, "MissingSymbol was not thrown")


def test_resource_to_struct_references_are_resolved_for_the_current_scope():
    root = _create_tree_resource_to_struct(actual="Struct", reference="Struct")
    assert_is_instance(root.find('.ns.Archive.resource.@Struct'), refs.StructureReference)
    resolve_references(root)
    assert_is_instance(root.find('.ns.Archive.resource.@@ns@Struct'), refs.StructureReference)


def test_resource_to_struct_references_throw_missing_symbol_if_no_name_found_in_current_scope():
    root = _create_tree_resource_to_struct(actual="Struct", reference="Strict")
    try:
        import Levenshtein
    except ImportError:
        _assert_missing_symbol_is_thrown(root,
                                         'Missing symbol "Strict" in .ns.Archive.resource.@Strict.')
    else:
        _assert_missing_symbol_is_thrown(root,
                                         'Missing symbol "Strict" in .ns.Archive.resource.@Strict.'
                                         ' Did you mean ".ns.Struct"?')


def test_resource_to_struct_references_are_verified_if_global_path_is_specified():
    root = _create_tree_resource_to_struct_with_extra_folding("Struct", ".ns.fold.Struct")
    assert_is_instance(root.find('.ns.Archive.resource.@@ns@fold@Struct'), refs.StructureReference)
    resolve_references(root)
    assert_is_instance(root.find('.ns.Archive.resource.@@ns@fold@Struct'), refs.StructureReference)


def test_resource_references_are_verified_if_global_path_is_specified():
    root = _create_tree_resource_to_struct_with_extra_folding("Strict", ".ns.fold.Struct")
    assert_is_instance(root.find('.ns.Archive.resource.@@ns@fold@Struct'), refs.StructureReference)
    try:
        import Levenshtein
    except ImportError:
        _assert_missing_symbol_is_thrown(root,
                                         'Missing symbol ".ns.fold.Struct" in '
                                         '.ns.Archive.resource.@@ns@fold@Struct.')
    else:
        _assert_missing_symbol_is_thrown(root,
                                         'Missing symbol ".ns.fold.Struct" in '
                                         '.ns.Archive.resource.@@ns@fold@Struct.'
                                         ' Did you mean ".ns.fold.Strict"?')


def test_resource_to_archive_references_are_resolved():
    root = _create_tree_resource_to_archive()
    assert_is_instance(root.find('.ns.Archive.resource.@RefArchive'), refs.ArchiveReference)
    resolve_references(root)
    assert_is_instance(root.find('.ns.Archive.resource.@@ns@RefArchive'), refs.ArchiveReference)


def test_resource_to_field_references_are_resolved():
    root = _create_tree_with_explicit_reference("Archive.resource2")
    assert_is_instance(root.find('.ns.Archive.resource.@Struct@Field'), refs.FieldReference)
    resolve_references(root)
    assert_is_instance(root.find('.ns.Archive.resource.@@ns@Struct@Field'), refs.FieldReference)


def test_resource_to_resource_references_are_resolved_for_namespace_path():
    root = _create_tree_with_explicit_reference("Archive.resource2")
    assert_is_instance(root.find('.ns.Archive.resource.@Archive@resource2'), refs.ResourceReference)
    resolve_references(root)
    assert_is_instance(root.find('.ns.Archive.resource.@@ns@Archive@resource2'),
                       refs.ResourceReference)


def test_resource_to_resource_references_are_resolved_for_parent_scope():
    root = _create_tree_with_explicit_reference("resource2")
    assert_is_instance(root.find('.ns.Archive.resource.@resource2'), refs.ResourceReference)
    resolve_references(root)
    assert_is_instance(root.find('.ns.Archive.resource.@@ns@Archive@resource2'),
                       refs.ResourceReference)


def test_resolved_references_appear_in_original_order():
    root = _create_tree_with_two_struct_references()
    resolve_references(root)
    assert_equal("@@ns@S1", root.find('.ns.Archive.resource').children[0].name)
    assert_equal("@@ns@S2", root.find('.ns.Archive.resource').children[1].name)


def test_implicit_references_structure_is_resolved():
    root = Root().insert(nodes.Namespace("n").insert(
        Archive("A").insert(res.Vector("r1"), res.Vector("r2"),
                            res.BoundResource("b").insert(
                                refs.ResourceReference("A.r1"),
                                refs.ResourceReference("A.r2")))))
    resolve_references(root)
    assert_equal("@@n@A@r1", root.find('.n.A.b').children[0].name)
    assert_equal("@@n@A@r2", root.find('.n.A.b').children[1].name)
