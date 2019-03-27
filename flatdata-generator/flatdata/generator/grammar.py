# pylint: disable=invalid-name

'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from pyparsing import (
    Word, alphas, alphanums, nums, cppStyleComment,
    Keyword, Group, Optional, Or, OneOrMore, delimitedList, ZeroOrMore,
    hexnums, Combine, FollowedBy, ParseException as pyparsingParseException
)

ParseException = pyparsingParseException

identifier = Word(alphas + "_", alphas + alphanums + "_")
qualified_identifier = Word(alphas + "_.", alphas + alphanums + "_.")

BASIC_TYPES = ["bool", "i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64"]
basic_type = Or([Keyword(t) for t in BASIC_TYPES])

bit_width = Word(nums)

dec_literal = Word(nums)
hex_literal = Combine("0x" + Word(hexnums))
signed_literal = Combine(Optional('-') + (dec_literal ^ hex_literal))

comment = cppStyleComment

enumValue = Group(
    Optional(comment)("doc") +
    identifier("name") +
    Optional('=' + signed_literal("constant"))
)

enum = Group(
    Optional(comment)("doc") +
    Keyword("enum") +
    identifier("name") + ':' + basic_type("type") +
    '{' +
    delimitedList(enumValue, ",")("enum_values") +
    Optional(',') +
    '}'
)

field = Group(
    Optional(comment)("doc") +
    identifier("name") +':' -
    qualified_identifier("type") +
    Optional(':' + bit_width("width")) +
    ';'
)

struct = Group(
    Optional(comment)("doc") +
    Keyword("struct") -
    identifier("name") +
    "{" +
    OneOrMore(field)("fields") +
    "}"
)

vector = Group(
    Keyword("vector") - "<" + qualified_identifier("type") + ">"
)

multivector = Group(
    Keyword("multivector") -
    "<" +
    bit_width("width") + "," +
    delimitedList(qualified_identifier, ",")("type") +
    ">"
)

single_object = Group(
    qualified_identifier("type")
)

raw_data = Group(
    Keyword("raw_data")
)

archive_resource = Group(
    Keyword("archive") + qualified_identifier("name")
)

resource_type = Group(
    raw_data("raw_data") |
    vector("vector") |
    multivector("multivector") |
    archive_resource("archive") |
    single_object("object") 
)

def _combine_list(t):
    return "".join(t[0].asList())

explicit_field_reference_prefix = Group(
    OneOrMore((Optional(".") + identifier + ~FollowedBy(',')))
).setParseAction(_combine_list)

explicit_reference = Group(
    Keyword("@explicit_reference") -
    "(" +
    explicit_field_reference_prefix("source_type") +
    "." +
    identifier("source_field") + "," + qualified_identifier("destination") +
    ")"
)

bound_implicitly = Group(
    Keyword("@bound_implicitly") -
    "(" +
    identifier("name") + ":" +
    delimitedList(qualified_identifier)("resources") +
    ")"
)

optional_decoration = Group(
    Keyword("@optional")
)

resource_decorations = Group(
    explicit_reference("explicit_reference") |
    optional_decoration("optional")
)

archive_decorations = Group(
    bound_implicitly("bound_implicitly")
)

resource = Group(
    Optional(comment)("doc") +
    ZeroOrMore(resource_decorations)("decorations") +
    identifier("name") + ':' -
    resource_type("type") + ';'
)

archive = Group(
    Optional(comment)("doc") +
    ZeroOrMore(archive_decorations)("decorations") +
    Keyword("archive") -
    identifier("name") +
    "{" +
    ZeroOrMore(resource)("resources") +
    "}"
)

constant = Group(
    Optional(comment)("doc") +
    Keyword("const") +
    basic_type("type") +
    identifier("name") + "=" +
    signed_literal("value") +
    ";"
)

flatdata_entry = (
    enum.setResultsName("enumerations", listAllMatches=True) |
    struct.setResultsName("structures", listAllMatches=True) |
    archive.setResultsName("archives", listAllMatches=True) |
    constant.setResultsName("constants", listAllMatches=True) |
    comment.setResultsName("comment", listAllMatches=True)
)

free_comments = Optional(OneOrMore(comment)("comment"))

namespace = Group(
    Keyword("namespace") +
    qualified_identifier("name") +
    "{" +
    ZeroOrMore(flatdata_entry) + "}" +
    Optional(comment)
)

flatdata_grammar = Group(free_comments +
                         OneOrMore(namespace)("namespace")
                         )("flatdata")
