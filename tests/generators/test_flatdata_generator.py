'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.generators.FlatdataGenerator import FlatdataGenerator
from .assertions import *

def test_normalization():
    expected_lines = [
        "namespace ns {",
        "const u32 C = 0xFFFFFFF;",
        "}",
        "",
        "namespace ns {",
        "const u32 D = -10;",
        "}",
        "",
        "namespace ns {",
        "struct S1",
        "{",
        "    f0 : u64 : 64;",
        "}",
        "}",
        "",
        "namespace ns {",
        "archive A0",
        "{",
        "    v0 : vector< .ns.S1 >;",
        "    v1 : multivector< 14, .ns.S1 >;",
        "}",
        "}",
        "",
        "namespace ns {",
        "struct S0",
        "{",
        "    f0 : u64 : 64;",
        "    f1 : u64 : 64;",
        "}",
        "}",
        "",
        "namespace ns {",
        "enum Enum1 : u16",
        "{",
        "    A = 1 ,",
        "    B = 13 ,",
        "    C = 14 ",
        "}",
        "}",
        "",
        "namespace ns {",
        "struct XXX",
        "{",
        "    e : .ns.Enum1 : 16;",
        "    f : .ns.Enum1 : 4;",
        "}",
        "}",
        "",
        "namespace ns {",
        "archive A1",
        "{",
        "    i : .ns.S0;",
        "    v0 : vector< .ns.S1 >;",
        "    @optional",
        "    v1 : vector< .ns.S1 >;",
        "    v2 : vector< .ns.XXX >;",
        "    @explicit_reference( .ns.S0.f0, .ns.A1.v0 )",
        "    @explicit_reference( .ns.S0.f1, .ns.A1.v0 )",
        "    @explicit_reference( .ns.S0.f1, .ns.A1.v1 )",
        "    mv : multivector< 14, .ns.S0 >;",
        "    rd : raw_data;",
        "    a : .ns.A0;",
        "}",
        "}",
        "",
    ]
    generate_and_assert_in("""
namespace ns{
    // Comment A
    struct S0 {
        f0 : u64 : 64;
        f1 : u64 : 64;
    }

    struct S1 {
        /*
         * Lots of comments
         */
        f0 : u64 : 64;
    }

    @bound_implicitly( b: A0.v0, A0.v1 )
    archive A0 {
        v0 : vector< S1 >;
        v1 : multivector< 14, S1 >;
    }

// Even more comments
enum Enum1 : u16 {
 A = 0x1, B=13,
 // Comment here as well
 C
}

struct XXX { e : Enum1; f : .ns.Enum1 : 4; }

    // Comments everywhere
    const u32 C = 0xFFFFFFF;
    const i32 D = -10;

    // Even here
    archive A1 {
        i : S0;
        // Another comment
        v0 : vector< S1 >;

        @optional
        v1 : vector< S1 >;

        v2 : vector< XXX >;

        // Yet another comment
        @explicit_reference( .ns.S0.f0, v0 )
        @explicit_reference( S0.f1, A1.v0 )
        @explicit_reference( S0.f1, .ns.A1.v1 )
        mv : multivector< 14, S0 >;
        rd : raw_data;
        a : archive A0;
    }
} // ns
""", FlatdataGenerator, *expected_lines)
