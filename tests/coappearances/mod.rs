use std::fmt;

use flatdata::*;

const META_SCHEMA: &str = r#"namespace coappearances { /**
 * Meta information about the book.
 */
struct Meta {
    title_ref : u32 : 32;
    author_ref : u32 : 32;
} }
namespace coappearances { @explicit_reference( Meta.title_ref, strings )
    @explicit_reference( Meta.author_ref, strings )
    meta : Meta; }"#;

const VERTICES_SCHEMA: &str = r#"namespace coappearances { /**
 * A character.
 */
struct Character {
    name_ref : u32 : 32;
} }
namespace coappearances { @explicit_reference( Character.name_ref, strings )
    vertices : vector< Character >; }"#;

define_archive_type!(Character, 4, (name_ref, u32, 0, 32));

const EDGES_SCHEMA: &str = r#"namespace coappearances { /**
 * An appearance of two characters in the same scene.
 *
 * count - multiplicity of the coappearance.
 * first_chapter_ref - a reference to the first chapter in which characters appear. How to get the
 * full range of chapters is described in `coappearances.cpp:read`.
 */
struct Coappearance {
    a_ref : u32 : 16;
    b_ref : u32 : 16;
    count : u32 : 16;
    first_chapter_ref: u32 : 16;
} }
namespace coappearances { @explicit_reference( Coappearance.a_ref, vertices )
    @explicit_reference( Coappearance.b_ref, vertices )
    @explicit_reference( Coappearance.first_chapter_ref, chapters )
    edges : vector< Coappearance >; }"#;

const CHAPTERS_SCHEMA: &str = r#"namespace coappearances { /**
 * A chapter in the book.
 */
struct Chapter {
    major: u8 : 4;
    minor: u8 : 7;
} }
namespace coappearances { chapters : vector< Chapter >; }"#;

const VERTICES_DATA_SCHEMA: &str = r#"namespace coappearances { /**
 * A nickname or an alternative name of a character.
 */
struct Nickname {
    ref: u32 : 32;
} }
namespace coappearances { /**
 * A description of a character.
 */
struct Description {
    ref: u32 : 32;
} }
namespace coappearances { /**
 * A relation of a character to another one.
 */
struct UnaryRelation {
    kind_ref: u32 : 32;
    to_ref: u32 : 16;
} }
namespace coappearances { /**
 * A relation of a character to two other characters.
 */
struct BinaryRelation {
    kind_ref: u32 : 32;
    to_a_ref: u32 : 16;
    to_b_ref: u32 : 16;
} }
namespace _builtin.multivector { struct IndexType32 { value : u64 : 32; } }
namespace coappearances { @explicit_reference( Nickname.ref, strings )
    @explicit_reference( Description.ref, strings )
    @explicit_reference( UnaryRelation.kind_ref, strings )
    @explicit_reference( UnaryRelation.to_ref, vertices )
    @explicit_reference( BinaryRelation.kind_ref, strings )
    @explicit_reference( BinaryRelation.to_a_ref, vertices )
    @explicit_reference( BinaryRelation.to_b_ref, vertices )
    vertices_data: multivector< 32, Nickname, Description, UnaryRelation, BinaryRelation >; }"#;

const VERTICES_DATA_INDEX_SCHEMA: &str = r#"index(namespace coappearances { /**
 * A nickname or an alternative name of a character.
 */
struct Nickname {
    ref: u32 : 32;
} }
namespace coappearances { /**
 * A description of a character.
 */
struct Description {
    ref: u32 : 32;
} }
namespace coappearances { /**
 * A relation of a character to another one.
 */
struct UnaryRelation {
    kind_ref: u32 : 32;
    to_ref: u32 : 16;
} }
namespace coappearances { /**
 * A relation of a character to two other characters.
 */
struct BinaryRelation {
    kind_ref: u32 : 32;
    to_a_ref: u32 : 16;
    to_b_ref: u32 : 16;
} }
namespace _builtin.multivector { struct IndexType32 { value : u64 : 32; } }
namespace coappearances { @explicit_reference( Nickname.ref, strings )
    @explicit_reference( Description.ref, strings )
    @explicit_reference( UnaryRelation.kind_ref, strings )
    @explicit_reference( UnaryRelation.to_ref, vertices )
    @explicit_reference( BinaryRelation.kind_ref, strings )
    @explicit_reference( BinaryRelation.to_a_ref, vertices )
    @explicit_reference( BinaryRelation.to_b_ref, vertices )
    vertices_data: multivector< 32, Nickname, Description, UnaryRelation, BinaryRelation >; })"#;

const STRINGS_SCHEMA: &str =
    r#"namespace coappearances { // All strings contained in the data separated by `\0`.
    strings: raw_data; }"#;

const GRAPH_SCHEMA: &str = r#"namespace coappearances { /**
 * Meta information about the book.
 */
struct Meta {
    title_ref : u32 : 32;
    author_ref : u32 : 32;
} }
namespace coappearances { /**
 * A character.
 */
struct Character {
    name_ref : u32 : 32;
} }
namespace coappearances { /**
 * An appearance of two characters in the same scene.
 *
 * count - multiplicity of the coappearance.
 * first_chapter_ref - a reference to the first chapter in which characters appear. How to get the
 * full range of chapters is described in `coappearances.cpp:read`.
 */
struct Coappearance {
    a_ref : u32 : 16;
    b_ref : u32 : 16;
    count : u32 : 16;
    first_chapter_ref: u32 : 16;
} }
namespace coappearances { /**
 * A nickname or an alternative name of a character.
 */
struct Nickname {
    ref: u32 : 32;
} }
namespace coappearances { /**
 * A description of a character.
 */
struct Description {
    ref: u32 : 32;
} }
namespace coappearances { /**
 * A relation of a character to another one.
 */
struct UnaryRelation {
    kind_ref: u32 : 32;
    to_ref: u32 : 16;
} }
namespace coappearances { /**
 * A relation of a character to two other characters.
 */
struct BinaryRelation {
    kind_ref: u32 : 32;
    to_a_ref: u32 : 16;
    to_b_ref: u32 : 16;
} }
namespace _builtin.multivector { struct IndexType32 { value : u64 : 32; } }
namespace coappearances { /**
 * A chapter in the book.
 */
struct Chapter {
    major: u8 : 4;
    minor: u8 : 7;
} }
namespace coappearances { @bound_implicitly( characters: vertices, vertices_data )
archive Graph {
    @explicit_reference( Meta.title_ref, strings )
    @explicit_reference( Meta.author_ref, strings )
    meta : Meta;

    @explicit_reference( Character.name_ref, strings )
    vertices : vector< Character >;

    @explicit_reference( Coappearance.a_ref, vertices )
    @explicit_reference( Coappearance.b_ref, vertices )
    @explicit_reference( Coappearance.first_chapter_ref, chapters )
    edges : vector< Coappearance >;

    @explicit_reference( Nickname.ref, strings )
    @explicit_reference( Description.ref, strings )
    @explicit_reference( UnaryRelation.kind_ref, strings )
    @explicit_reference( UnaryRelation.to_ref, vertices )
    @explicit_reference( BinaryRelation.kind_ref, strings )
    @explicit_reference( BinaryRelation.to_a_ref, vertices )
    @explicit_reference( BinaryRelation.to_b_ref, vertices )
    vertices_data: multivector< 32, Nickname, Description, UnaryRelation, BinaryRelation >;

    chapters : vector< Chapter >;

    // All strings contained in the data separated by `\0`.
    strings: raw_data;
} }"#;

define_archive_type!(Meta, 8, (title_ref, u32, 0, 32), (author_ref, u32, 32, 32));

define_archive_type!(
    Coappearance,
    8,
    (a_ref, u32, 0, 16),
    (b_ref, u32, 16, 16),
    (count, u32, 32, 16),
    (first_chapter_ref, u32, 48, 16)
);

define_archive_type!(Chapter, 2, (major, u8, 0, 4), (minor, u8, 4, 7));

// TODO: Resolve ref clashing with keywords of Rust.
define_archive_type!(Nickname, 4, (ref_, u32, 0, 32));
define_archive_type!(Description, 4, (ref_, u32, 0, 32));

define_archive_type!(
    UnaryRelation,
    6,
    (kind_ref, u32, 0, 32),
    (to_ref, u32, 32, 16)
);

define_archive_type!(
    BinaryRelation,
    8,
    (kind_ref, u32, 0, 32),
    (to_a_ref, u32, 32, 16),
    (to_b_ref, u32, 48, 16)
);

define_variadic_archive_type!(VerticesData, IndexType32,
    0 => Nickname,
    1 => Description,
    2 => UnaryRelation,
    3 => BinaryRelation);

define_index_type!(IndexType32, 4, 32);

define_archive!(Graph, GRAPH_SCHEMA;
    // struct resources
    (meta, Meta, META_SCHEMA);
    // raw data resources
    (strings, STRINGS_SCHEMA);
    // vector resources
    (vertices, Character, VERTICES_SCHEMA),
    (edges, Coappearance, EDGES_SCHEMA),
    (chapters, Chapter, CHAPTERS_SCHEMA);
    // multivector resources
    (vertices_data, VerticesData, VERTICES_DATA_SCHEMA,
        vertices_data_index, internal::IndexType32, VERTICES_DATA_INDEX_SCHEMA)
);
