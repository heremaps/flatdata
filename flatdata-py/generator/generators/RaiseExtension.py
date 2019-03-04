from jinja2 import nodes
from jinja2.ext import Extension
from jinja2.exceptions import TemplateRuntimeError


class RaiseExtension(Extension):
    tags = set(['raise'])

    def parse(self, parser):
        lineno = next(parser.stream).lineno
        message_node = parser.parse_expression()
        return nodes.CallBlock(
            self.call_method(name='_raise', args=[message_node], lineno=lineno),
            [], [], [], lineno=lineno
        )

    def _raise(self, msg, caller):
        raise TemplateRuntimeError(msg)
