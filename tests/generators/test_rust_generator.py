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
        }
    """, RustGenerator, """
pub mod n {
    /// There is some documentation about foo.
pub const FOO: i8 = 17;
// Comment
pub const BAR: u16 = 66;
}""")


def test_folded_namespaces_are_represented_correctly():
    generate_and_assert_in("""
        namespace n.nn{
        const i8 foo = 17;
        const u16 bar = 0x42;
        }
    """, RustGenerator, """
pub mod n {
    pub mod nn {
    pub const FOO: i8 = 17;
pub const BAR: u16 = 66;
}
}
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
    SMut,
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
        S, schema::resources::a::R0);
    // vector resources
;
    // multivector resources
;
    // raw data resources
;
    // subarchives
;
    // optional subarchives
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
    S, schema::resources::a::OBJECT_RESOURCE);""")


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
        T, schema::resources::a::VECTOR_RESOURCE);""")


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
    }
    }""", RustGenerator, """
/// Builtin type to for MultiVector index
define_index!(
    IndexType33,
    IndexType33Mut,
    schema::structs::INDEX_TYPE33,
    5,
    33
);

/// Builtin union type of T, U.
define_variadic_struct!(MultivectorResource, MultivectorResourceItemBuilder, IndexType33,
    0 => (T, add_t),
    1 => (U, add_u));

define_archive!(A, ABuilder,
    schema::structs::A;
    // struct resources
;
    // vector resources
;
    // multivector resources
    (multivector_resource, start_multivector_resource,
        MultivectorResource, schema::resources::a::MULTIVECTOR_RESOURCE,
        multivector_resource_index, IndexType33);
    // raw data resources
;
    // subarchives
;
    // optional subarchives
);""")


def test_raw_data_resource_is_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    archive A {
        raw_data_resource : raw_data;
    }
    }""", RustGenerator, """
(raw_data_resource, set_raw_data_resource,
    schema::resources::a::RAW_DATA_RESOURCE);""")


# TODO: Enable when optional resources are supported
def _test_optional_resource_is_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    archive A {
        @optional
        raw_data_resource : raw_data;
    }
    }""", RustGenerator, "NOT_YET_SUPPORTED")
