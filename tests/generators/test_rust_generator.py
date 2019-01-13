'''
 Copyright (c) 2018 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.generators.RustGenerator import RustGenerator
from .assertions import *


def test_constants_are_declared_correctly():
    generate_and_assert_in("""
        namespace n{
        /**
         * There is some documentation about foo.
         */
        const i8 foo = 17;
        // Comment
        const u16 bar = 0x42;
        // Large constant
        const u64 foobar = 1000000000;
        }
    """, RustGenerator, """
/// There is some documentation about foo.
pub const FOO: i8 = 17;
""",
"""
// Comment
pub const BAR: u16 = 66;
""",
"""
// Large constant
pub const FOOBAR: u64 = 1_000_000_000;
""")


def test_structures_are_declared_correctly():
    generate_and_assert_in("""
        namespace n{
        struct S {
            f0 : u8 : 3;
            f1 : u16 : 15;
        }
        }
    """, RustGenerator, """
define_struct!(
    S,
    RefS,
    RefMutS,
    schema::structs::S,
    3,
    (f0, set_f0, u8, 0, 3),
    (f1, set_f1, u16, 3, 15));
""")


def test_archives_are_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    struct S {
        f0 : u8 : 3;
    }
    archive A {
        r0 : S;
    }
    }""", RustGenerator, """
define_archive!(A, ABuilder,
    schema::structs::A;
    // struct resources
    (r0, set_r0,
        S, schema::resources::a::R0, false);
    // vector resources
;
    // multivector resources
;
    // raw data resources
;
    // subarchives
);
""")


def test_object_resource_is_represented_correctly():
    generate_and_assert_in("""
    namespace n{
    struct S {
        f0 : u8 : 3;
    }
    archive A {
        object_resource : S;
    }
    }
""", RustGenerator, """
(object_resource, set_object_resource,
    S, schema::resources::a::OBJECT_RESOURCE, false);""")


def test_vector_resource_is_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    struct T {
        f0 : u8 : 3;
    }
    archive A {
        vector_resource : vector< T >;
    }
    }""", RustGenerator, """
(vector_resource, set_vector_resource, start_vector_resource,
        T, schema::resources::a::VECTOR_RESOURCE, false);""")


def test_multi_vector_resource_is_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    struct U {
        f0 : u8 : 3;
    }
    struct T {
        f0 : u8 : 3;
    }
    archive A {
        multivector_resource : multivector< 33, T, U >;
        @optional
        opt_multivector_resource : multivector< 33, T, U >;
    }
    }""", RustGenerator,
"""
/// Builtin type to for MultiVector index
define_index!(
    IndexType33,
    RefIndexType33,
    RefMutIndexType33,
    schema::structs::INDEX_TYPE33,
    5,
    33
);""",
"""
/// Builtin union type of T, U.
define_variadic_struct!(MultivectorResource, RefMultivectorResource, BuilderMultivectorResource,
    IndexType33,
    0 => (T, add_t),
    1 => (U, add_u));
/// Builtin union type of T, U.
define_variadic_struct!(OptMultivectorResource, RefOptMultivectorResource, BuilderOptMultivectorResource,
    IndexType33,
    0 => (T, add_t),
    1 => (U, add_u));
""",
"""
    (multivector_resource, start_multivector_resource,
        MultivectorResource, schema::resources::a::MULTIVECTOR_RESOURCE,
        multivector_resource_index, IndexType33, false),""",
"""
    (opt_multivector_resource, start_opt_multivector_resource,
        OptMultivectorResource, schema::resources::a::OPT_MULTIVECTOR_RESOURCE,
        opt_multivector_resource_index, IndexType33, true);
""")


def test_raw_data_resource_is_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    archive A {
        raw_data_resource_a : raw_data;
        @optional
        raw_data_resource_b : raw_data;
    }
    }""", RustGenerator, """
(raw_data_resource_a, set_raw_data_resource_a,
    schema::resources::a::RAW_DATA_RESOURCE_A, false),
(raw_data_resource_b, set_raw_data_resource_b,
    schema::resources::a::RAW_DATA_RESOURCE_B, true);""")

def test_optional_resource_is_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    archive A {
        @optional
        raw_data_resource : raw_data;
    }
    }""", RustGenerator,
"""
(raw_data_resource, set_raw_data_resource,
        schema::resources::a::RAW_DATA_RESOURCE, true);
""")


def test_unsigned_enum_is_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    /* enum doc */
    enum Variant : u32 {
        // A doc
        A = 42,
        // B doc
        B = 0x42
    }
    }""", RustGenerator, """
/// enum doc
#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Variant {
    // A doc
    A = 42,
    // B doc
    B = 66,
}

impl Int for Variant {
    const IS_SIGNED: bool = false;
}""")

def test_signed_enum_is_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    /* enum doc */
    enum Variant : i64 {
        // A doc
        A = 42,
        // B doc
        B = 0x42
    }
    }""", RustGenerator, """
/// enum doc
#[derive(Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum Variant {
    // A doc
    A = 42,
    // B doc
    B = 66,
}

impl Int for Variant {
    const IS_SIGNED: bool = true;
}""")
