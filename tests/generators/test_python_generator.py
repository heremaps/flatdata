'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.generators.PythonGenerator import PythonGenerator
from .assertions import *


def test_structures_are_defined_correctly():
    expectation = [
        """
class foo_S(flatdata.structure.Structure):
    _SCHEMA = \"\"\"namespace foo { struct S {
        member1 : u8 : 3;
        member2 : u64 : 17;
        member3 : i32 : 11;
    } }\"\"\"
    _SIZE_IN_BITS = 31
    _SIZE_IN_BYTES = 4
    _FIELDS = {
        "member1": flatdata.structure.FieldSignature(offset=0, width=3, is_signed=False),
        "member2": flatdata.structure.FieldSignature(offset=3, width=17, is_signed=False),
        "member3": flatdata.structure.FieldSignature(offset=20, width=11, is_signed=True),
    }
    """
    ]
    generate_and_assert_in("""
namespace foo{

struct S {
    member1 : u8 : 3;
    member2 : u64 : 17;
    member3 : i32 : 11;
}
}
""", PythonGenerator, *expectation)


def test_archives_are_defined_correctly():
    expectation = [
        """
    class foo_A(flatdata.archive.Archive):
        _SCHEMA = \"\"\"namespace foo { struct S {
            f1 : u8 : 3;
        } }
        namespace foo { archive A {
            r0 : S;
        } }\"\"\"
    _NAME = "A"
    _RESOURCES = {
        "A.archive" : flatdata.archive.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False),
        "r0": flatdata.archive.ResourceSignature(container=flatdata.resources.Instance,
            initializer=foo_S,
            schema=\"\"\"namespace foo { struct S {
           f1 : u8 : 3;
       } }
       namespace foo { r0 : S; }\"\"\",
        is_optional=False),
    }

    def __init__(self, path):
        flatdata.archive.Archive.__init__(self, path)
        """
    ]
    generate_and_assert_in("""
namespace foo{

struct S {
    f1 : u8 : 3;
}

archive A {
    r0 : S;
}
}
""", PythonGenerator, *expectation)


def test_resource_optionality():
    expectation = [
        """
        "r0": flatdata.archive.ResourceSignature(container=flatdata.resources.Instance,
            initializer=foo_S,
            schema=\"\"\"namespace foo { struct S {
            f1 : u8 : 3;
    } }
    namespace foo { @optional
    r0 : S; }\"\"\",
     is_optional=True),
        """
    ]
    generate_and_assert_in("""
namespace foo{

struct S {
    f1 : u8 : 3;
}

archive A {
    @optional
    r0 : S;
}
}
""", PythonGenerator, *expectation)
