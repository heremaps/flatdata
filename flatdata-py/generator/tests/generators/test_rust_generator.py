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
    (f0, set_f0, u8, u8, 0, 3),
    (f1, set_f1, u16, u16, 3, 15));
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
    schema::a::A;
    // struct resources
    (r0, set_r0,
        super::n::S,
        schema::a::resources::R0, false);
    // vector resources
;
    // multivector resources
;
    // raw data resources
;
    // subarchives
);

""")

def test_namespaces_are_handled_properly():
    generate_and_assert_in("""
        namespace test_structures {
        struct A {
            x : u32;
        }

        enum E : u8 {
            VALUE_1
        }
        } // namespace test_structures

        namespace test_structures2 {
        struct A {
            x : u32;
            e : .test_structures.E;
        }
        } // namespace test_structures2

        namespace tiny_archive {
        archive TinyArchive {
        }
        } // tiny_archive

        namespace my_archive {
        archive MyArchive {
            a : .test_structures.A;
            some_data : vector< .test_structures2.A >;
            mixed_data : multivector< 32, .test_structures.A >;
            tiny : archive .tiny_archive.TinyArchive;
        }
        } // my_archive
        """, RustGenerator, """
pub mod test_structures {

pub mod schema {
pub mod structs {
pub const A: &str = r#"namespace test_structures {
struct A
{
    x : u32 : 32;
}
}

"#;}}

define_struct!(
    A,
    RefA,
    RefMutA,
    schema::structs::A,
    4,
    (x, set_x, u32, u32, 0, 32));


#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum E {
    Value1 = 0,
}

impl Int for E {
    const IS_SIGNED: bool = false;
}
}

pub mod test_structures2 {

pub mod schema {
pub mod structs {
pub const A: &str = r#"namespace test_structures {
enum E : u8
{
    VALUE_1 = 0,
}
}

namespace test_structures2 {
struct A
{
    x : u32 : 32;
    e : .test_structures.E : 8;
}
}

"#;}}

define_struct!(
    A,
    RefA,
    RefMutA,
    schema::structs::A,
    5,
    (x, set_x, u32, u32, 0, 32),
    (e, set_e, super::test_structures::E, u8, 32, 8));
}

pub mod tiny_archive {

pub mod schema {
pub mod structs {}pub mod tiny_archive {
pub const TINY_ARCHIVE: &str = r#"namespace tiny_archive {
archive TinyArchive
{
}
}

"#;
pub mod resources {}
}
}


define_archive!(TinyArchive, TinyArchiveBuilder,
    schema::tiny_archive::TINY_ARCHIVE;
    // struct resources
;
    // vector resources
;
    // multivector resources
;
    // raw data resources
;
    // subarchives
);

}

pub mod my_archive {

pub mod schema {
pub mod structs {}pub mod my_archive {
pub const MY_ARCHIVE: &str = r#"namespace test_structures {
struct A
{
    x : u32 : 32;
}
}

namespace test_structures {
enum E : u8
{
    VALUE_1 = 0,
}
}

namespace test_structures2 {
struct A
{
    x : u32 : 32;
    e : .test_structures.E : 8;
}
}

namespace tiny_archive {
archive TinyArchive
{
}
}

namespace my_archive {
archive MyArchive
{
    a : .test_structures.A;
    some_data : vector< .test_structures2.A >;
    mixed_data : multivector< 32, .test_structures.A >;
    tiny : archive .tiny_archive.TinyArchive;
}
}

"#;
pub mod resources {pub const A: &str = r#"namespace test_structures {
struct A
{
    x : u32 : 32;
}
}

namespace my_archive {
archive MyArchive
{
    a : .test_structures.A;
}
}

"#;pub const SOME_DATA: &str = r#"namespace test_structures {
enum E : u8
{
    VALUE_1 = 0,
}
}

namespace test_structures2 {
struct A
{
    x : u32 : 32;
    e : .test_structures.E : 8;
}
}

namespace my_archive {
archive MyArchive
{
    some_data : vector< .test_structures2.A >;
}
}

"#;pub const MIXED_DATA: &str = r#"namespace test_structures {
struct A
{
    x : u32 : 32;
}
}

namespace my_archive {
archive MyArchive
{
    mixed_data : multivector< 32, .test_structures.A >;
}
}

"#;pub const TINY: &str = r#"namespace tiny_archive {
archive TinyArchive
{
}
}

namespace my_archive {
archive MyArchive
{
    tiny : archive .tiny_archive.TinyArchive;
}
}

"#;}
}
}


/// Builtin union type of .test_structures.A.
define_variadic_struct!(MixedData, RefMixedData, BuilderMixedData,
    IndexType32,
    0 => ( A, super::test_structures::A, add_a));
define_archive!(MyArchive, MyArchiveBuilder,
    schema::my_archive::MY_ARCHIVE;
    // struct resources
    (a, set_a,
        super::test_structures::A,
        schema::my_archive::resources::A, false);
    // vector resources
    (some_data, set_some_data, start_some_data,
        super::test_structures2::A,
        schema::my_archive::resources::SOME_DATA, false);
    // multivector resources
    (mixed_data, start_mixed_data,
        MixedData,
        schema::my_archive::resources::MIXED_DATA,
        mixed_data_index, super::_builtin::multivector::IndexType32, false);
    // raw data resources
;
    // subarchives
    (tiny,
        super::tiny_archive::TinyArchive, super::tiny_archive::TinyArchiveBuilder,
        schema::my_archive::resources::TINY, false));

}

pub mod _builtin {

pub mod multivector {

pub mod schema {
pub mod structs {
pub const INDEX_TYPE32: &str = r#""#;}}
/// Builtin type to for MultiVector index
define_index!(
    IndexType32,
    RefIndexType32,
    RefMutIndexType32,
    schema::structs::INDEX_TYPE32,
    4,
    32
);

}

pub mod schema {
pub mod structs {}}
}

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
    super::n::S, schema::a::resources::OBJECT_RESOURCE, false);""")


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
        super::n::T, schema::a::resources::VECTOR_RESOURCE, false);""")


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
/// Builtin union type of .n.T, .n.U.
define_variadic_struct!(MultivectorResource, RefMultivectorResource, BuilderMultivectorResource,
    IndexType33,
    0 => ( T, super::n::T, add_t),
    1 => ( U, super::n::U, add_u));
/// Builtin union type of .n.T, .n.U.
define_variadic_struct!(OptMultivectorResource, RefOptMultivectorResource, BuilderOptMultivectorResource,
    IndexType33,
    0 => ( T, super::n::T, add_t),
    1 => ( U, super::n::U, add_u));
""",
"""
    (multivector_resource, start_multivector_resource,
        MultivectorResource, schema::a::resources::MULTIVECTOR_RESOURCE,
        multivector_resource_index, super::_builtin::multivector::IndexType33, false),""",
"""
    (opt_multivector_resource, start_opt_multivector_resource,
        OptMultivectorResource, schema::a::resources::OPT_MULTIVECTOR_RESOURCE,
        opt_multivector_resource_index, super::_builtin::multivector::IndexType33, true);
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
    schema::a::resources::RAW_DATA_RESOURCE_A, false),
(raw_data_resource_b, set_raw_data_resource_b,
    schema::a::resources::RAW_DATA_RESOURCE_B, true);""")

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
        schema::a::resources::RAW_DATA_RESOURCE, true);
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
