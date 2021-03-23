import argparse
import fnmatch
import os
import sys

from flatdata.generator.engine import Engine
from flatdata.lib.resource_storage import *
from flatdata.lib.file_resource_writer import *

INSTANCE_TEST_SCHEMA = """
namespace backward_compatibility {
    struct SignedStruct {
        a : i16 : 5;
        b : u32 : 32;
        c : i32 : 7;
        d : u32 : 32;
    }
    archive Archive {
        resource: SignedStruct;
    }
}
"""

def main():
    # parser = argparse.ArgumentParser()
    # parser.add_argument("-p", "--path", type=str, dest="path", required=True,
    #                     help="Path to archive")
    # parser.add_argument("-s", "--schema", type=str, dest="schema", required=True,
    #                     help="Name of the archive")
    # args = parser.parse_args()

    # module = Engine(INSTANCE_TEST_SCHEMA).render_python_module()
    # builder = module.ArchiveBuilder(ResourceStorage(FileResourceWriter(), "/home/vinag/test"))
    # builder.set("resource", {"a": -0x1, "b": 0x01234567, "c": -0x28, "d": 0})
    # builder.finish()

#     multivector_test_schema = """
# namespace backward_compatibility {
#     struct SimpleStruct {
#         a : u32 : 32;
#         b : u32 : 32;
#     }
#     struct SignedStruct {
#         a : i16 : 5;
#         b : u32 : 32;
#         c : i32 : 7;
#         d : u32 : 32;
#     }
#     archive Archive {
#         resource: multivector< 33, SimpleStruct, SignedStruct >;
#     }
# }
# """

#     multivector_data = [
#     [
#         {
#             "name": "backward_compatibility_SignedStruct",
#             "attributes": {
#                 "a": -1,
#                 "b": 19088743,
#                 "c": -40,
#                 "d": 0
#             }
#         },
#         {
#             "name": "backward_compatibility_SimpleStruct",
#             "attributes": {
#                 "a": 4294967295,
#                 "b": 3735928559
#             }
#         }
#     ],
#     [],
#     [
#         {
#             "name": "backward_compatibility_SimpleStruct",
#             "attributes": {
#                 "a": 4294967295,
#                 "b": 3735928559
#             }
#         },
#         {
#             "name": "backward_compatibility_SignedStruct",
#             "attributes": {
#                 "a": -1,
#                 "b": 19088743,
#                 "c": -40,
#                 "d": 0
#             }
#         }
#     ],
#     [
#         {
#             "name": "backward_compatibility_SimpleStruct",
#             "attributes": {
#                 "a": 4294967295,
#                 "b": 3735928559
#             }
#         }
#     ]
#     ]

#     module = Engine(multivector_test_schema).render_python_module()
#     builder = module.ArchiveBuilder(ResourceStorage(FileResourceWriter(), "/home/vinag/test"))
#     builder.set("resource", multivector_data)
#     builder.finish()

    vector_test_schema = """
namespace backward_compatibility {
    struct SignedStruct {
        a : i16 : 5;
        b : u32 : 32;
        c : i32 : 7;
        d : u32 : 32;
    }
    archive Archive {
        resource: vector< SignedStruct >;
    }
}
"""

    module = Engine(vector_test_schema).render_python_module()
    builder = module.ArchiveBuilder(ResourceStorage(FileResourceWriter(), "/home/vinag/test"))
    builder.set("resource", [{"a": -0x1, "b": 0x01234567, "c": -0x28, "d": 0}] * 2)
    builder.finish()

if __name__ == "__main__":
    main()