'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import re
import difflib

from flatdata.generator.tree.builder import build_ast

def unify_whitespace(value):
    removed_trailing = re.sub(r"\s+$", "", value)
    return re.sub(r"\s+", " ", removed_trailing)

def diff(a, b):
    return "\n".join(difflib.Differ().compare(a.split("\n"), b.split("\n")))

def generate_and_assert_in(definition, generator, *expectations, unexpected_items=None):
    tree = build_ast(definition=definition)
    contents = generator().render(tree)
    contents_unified = unify_whitespace(contents)

    assert expectations or unexpected_items, "No expectations specified"
    for expectation in expectations:
        expectation_unified = unify_whitespace(expectation)
        assert expectation_unified in contents_unified, "\n*Did not find:\n%s\n========== IN GENERATED CODE ===========\n%s\n\n========== DIFF ===========%s" % (expectation, contents, diff(expectation, contents))

    if unexpected_items:
        for unexpected_item in unexpected_items:
            unexpected_item_unified = unify_whitespace(unexpected_item)
            assert not unexpected_item_unified in contents_unified, "\n*Did find:\n%s\n========== IN GENERATED CODE ===========\n%s" % (unexpected_item, contents)
