'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from pyparsing import Word, alphas, alphanums, nums, cppStyleComment, Keyword, Group, Optional, \
    Or, OneOrMore, originalTextFor, delimitedList, ZeroOrMore, hexnums, Combine, \
    ParseException as pyparsingParseException

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
    Optional(comment).setResultsName("doc") +
    identifier.setResultsName("name") + 
    Optional( '=' + signed_literal.setResultsName("constant") )
)

enum = originalTextFor(
    Group(
        Optional(comment).setResultsName("doc") +
        Keyword("enum") +
        identifier.setResultsName("name") +
        ':' +
        basic_type.setResultsName("type") +
        '{' +
        delimitedList(enumValue.setResultsName("enum_values", listAllMatches=True), ",") +
        '}'
    ), asString=False
)

field = Group(
    Optional(comment).setResultsName("doc") +
    identifier.setResultsName("name") + ':' +
    identifier.setResultsName("type") +
    Optional(':' + bit_width.setResultsName("width")) +
    ';'
)

struct = originalTextFor(
    Group(
        Optional(comment).setResultsName("doc") +
        Keyword("struct") +
        identifier.setResultsName("name") + "{" +
        OneOrMore(field.setResultsName("fields", listAllMatches=True)) + "}"
    ), asString=False
)

vector = Group(
    Keyword("vector") + "<" + qualified_identifier.setResultsName("type") + ">"
)

multivector = Group(
    Keyword("multivector") + "<" +
    bit_width.setResultsName("width") + "," +
    delimitedList(qualified_identifier.setResultsName("type", listAllMatches=True), ",") + ">"
)

single_object = Group(
    qualified_identifier.setResultsName("type")
)

raw_data = Group(
    Keyword("raw_data")
)

archive_resource = Group(
    Keyword("archive") + qualified_identifier.setResultsName("name")
)

resource_type = Group(
    raw_data.setResultsName("raw_data") ^
    single_object.setResultsName("object") ^
    vector.setResultsName("vector") ^
    multivector.setResultsName("multivector") ^
    archive_resource.setResultsName("archive")
)

explicit_reference = Group(
    Keyword("@explicit_reference") + "(" +
    identifier.setResultsName("source_type") + "." +
    identifier.setResultsName("source_field") + "," + qualified_identifier.setResultsName(
        "destination") + ")"
)

bound_implicitly = Group(
    Keyword("@bound_implicitly") + "(" + identifier.setResultsName("name") + ":" +
    delimitedList(qualified_identifier).setResultsName("resources") + ")"
)

optional_decoration = Group(
    Keyword("@optional")
)

resource_decorations = Group(
    explicit_reference.setResultsName("explicit_reference") ^
    optional_decoration.setResultsName("optional")
)

archive_decorations = Group(
    bound_implicitly.setResultsName("bound_implicitly")
)

resource = originalTextFor(
    Group(
        Optional(comment).setResultsName("doc") +
        ZeroOrMore(resource_decorations).setResultsName("decorations") +
        identifier.setResultsName("name") + ':' +
        resource_type.setResultsName("type") + ';'
    ), asString=False
)

archive = originalTextFor(
    Group(
        Optional(comment).setResultsName("doc") +
        ZeroOrMore(archive_decorations).setResultsName("decorations") +
        Keyword("archive") +
        identifier.setResultsName("name") +
        "{" +
        OneOrMore(resource.setResultsName("resources", listAllMatches=True)) +
        "}"
    ), asString=False
)

constant = originalTextFor(
    Group(
        Optional(comment).setResultsName("doc") +
        Keyword("const") +
        basic_type.setResultsName("type") +
        identifier.setResultsName("name") + "=" +
        signed_literal.setResultsName("value") +
        ";"
    ), asString=False
)

flatdata_entry = (
    enum.setResultsName("enumerations", listAllMatches=True) ^
    struct.setResultsName("structures", listAllMatches=True) ^
    archive.setResultsName("archives", listAllMatches=True) ^
    constant.setResultsName("constants", listAllMatches=True) ^
    comment.setResultsName("comment", listAllMatches=True)
)

free_comments = Optional(OneOrMore(comment).setResultsName("comment", listAllMatches=True))

namespace = Group(
    Keyword("namespace") +
    qualified_identifier.setResultsName("name") + "{"
    + ZeroOrMore(flatdata_entry)
    + "}" + Optional(comment)
)

flatdata_grammar = Group(free_comments +
                         OneOrMore(namespace.setResultsName("namespace", listAllMatches=True))
                         ).setResultsName("flatdata")
