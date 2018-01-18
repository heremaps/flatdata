'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''


class FlatdataSyntaxError(RuntimeError):
    pass


class SymbolRedefinition(FlatdataSyntaxError):
    def __init__(self, duplicate, existing):
        super(SymbolRedefinition, self).__init__(
            "Symbol redefined: {duplicate} already exists at {existing}".format(
                duplicate=duplicate,
                existing=existing))


class CircularReferencing(FlatdataSyntaxError):
    def __init__(self, node, child):
        super(CircularReferencing, self).__init__(
            "Circular reference in schema: {node} -> {child}".format(
                node=node, child=child))


class MissingSymbol(FlatdataSyntaxError):
    def __init__(self, name, options, node):
        message = "Missing symbol \"{name}\" in {path}.".format(name=name, path=node.path)
        try:
            import Levenshtein
            options = sorted(
                [(Levenshtein.distance(name, option.split('.')[-1]), option) for option in options],
                key=lambda x: x[0])
            if len(options) > 0:
                message += " Did you mean \"{options}\"?".format(options=options[0][1])
        except ImportError:
            pass
        super(MissingSymbol, self).__init__(message)


class IncorrectReferenceType(FlatdataSyntaxError):
    def __init__(self, actual, expected):
        super(IncorrectReferenceType, self).__init__(
            "Referring to incorrect type. Expected {expected}, actual {actual}".format(
                expected=expected, actual=actual))


class UnexpectedResourceType(FlatdataSyntaxError):
    def __init__(self, name):
        super(UnexpectedResourceType, self).__init__(
            "Unexpected resource type: {name}".format(name=name))


class ParsingError(FlatdataSyntaxError):
    def __init__(self, pyparsing_error):
        super(ParsingError, self).__init__(self.create_message(pyparsing_error))

    @staticmethod
    def create_message(e):
        return "Failed to parse the schema. Details below:\n" \
               "  {line}\n" \
               "  {pointer}\n" \
               "  {message}".format(line=e.line, pointer=" " * (e.column - 1) + "^",
                                    message=str(e))


class InvalidWidthError(FlatdataSyntaxError):
    def __init__(self, width, type):
        super(InvalidWidthError, self).__init__(
            "Bit field of {width}bit width cannot fit in {type}".format(width=width,
                                                                        type=type))
