'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

import imp

from generator.tree.builder import SyntaxTreeBuilder
from generator.tree.nodes.trivial.namespace import Namespace

from .generators.CppGenerator import CppGenerator
from .generators.DotGenerator import DotGenerator
from .generators.GoGenerator import GoGenerator
from .generators.PythonGenerator import PythonGenerator
from .generators.RustGenerator import RustGenerator
from .generators.FlatdataGenerator import FlatdataGenerator


class Engine(object):
    """
    Flatdata Generator Engine.
    Implements code generation from the given flatdata schema.
    """

    class GeneratorNotDefined(RuntimeError):
        pass

    _GENERATORS = {
        "cpp": CppGenerator,
        "dot": DotGenerator,
        "go": GoGenerator,
        "py": PythonGenerator,
        "rust": RustGenerator,
        "flatdata" : FlatdataGenerator
    }

    @classmethod
    def available_generators(cls):
        """
        Lists names of available code generators.
        """
        return list(cls._GENERATORS.keys())

    def __init__(self, schema):
        """
        Instantiates generator engine for a given schema.
        :raises FlatdataSyntaxError
        """
        self.schema = schema
        self.tree = SyntaxTreeBuilder.build(schema)

    def render(self, generator_name):
        """
        Render schema with a given generator
        :param generator_name:
        """
        generator = self._create_generator(generator_name)
        if generator is None:
            raise self.GeneratorNotDefined()

        output_content = generator.render(self.tree)
        return output_content

    def render_python_module(self, module_name=None, archive_name=None):
        """
        Render python module.
        :param module_name: Module name to use. If none, root namespace name is used.
        :param archive_name: Archive name to lookup, if specified, archive type is returned along with the model
        """
        root_namespace = self._find_root_namespace(self.tree)
        module_code = self.render("py")
        module = imp.new_module(module_name if module_name is not None else root_namespace.name)
        exec(module_code, module.__dict__)
        if archive_name is None:
            return module

        archive_type = getattr(module, root_namespace.name + "_" + archive_name) if archive_name else None
        return module, archive_type

    @classmethod
    def _create_generator(cls, name):
        generator_type = cls._GENERATORS.get(name, None)
        if generator_type is None:
            return None

        return generator_type()

    @staticmethod
    def _find_root_namespace(tree):
        root_children = tree.root.children
        root_namespaces = [
            child for child in root_children
            if isinstance(child, Namespace) and "builtin" not in child.name
        ]
        if not root_namespaces:
            raise RuntimeError("No root namespace found.")
        elif len(root_namespaces) > 1:
            raise RuntimeError("Ambiguous root namespace. Could not find root archive.")
        return root_namespaces[0]
