use flatdata::Struct;

mod schema {
    pub mod resources {
        pub const INVARIANTS: &str = r#"namespace coappearances { struct Invariants {
    max_degree : u32 : 16;
    max_degree_ref : u32 : 16;
    min_degree : u32 : 16;
    min_degree_ref : u32 : 16;
    num_connected_components : u32 : 16;
} }
namespace coappearances { invariants : Invariants; }"#;
        pub const VERTEX_DEGREES: &str = r#"namespace coappearances { struct Degree {
    value : u32 : 16;
} }
namespace coappearances { vertex_degrees : vector< Degree >; }"#;
        pub const META: &str = r#"namespace coappearances { /**
 * Meta information about the book.
 */
struct Meta {
    title_ref : u32 : 32;
    author_ref : u32 : 32;
} }
namespace coappearances { @explicit_reference( Meta.title_ref, strings )
    @explicit_reference( Meta.author_ref, strings )
    meta : Meta; }"#;
        pub const VERTICES: &str = r#"namespace coappearances { /**
 * A character.
 */
struct Character {
    name_ref : u32 : 32;
} }
namespace coappearances { @explicit_reference( Character.name_ref, strings )
    vertices : vector< Character >; }"#;
        pub const EDGES: &str = r#"namespace coappearances { /**
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
        pub const VERTICES_DATA: &str = r#"namespace coappearances { /**
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
        pub const CHAPTERS: &str = r#"namespace coappearances { /**
 * A chapter in the book.
 */
struct Chapter {
    major: u8 : 4;
    minor: u8 : 7;
} }
namespace coappearances { chapters : vector< Chapter >; }"#;
        pub const STRINGS: &str =
            r#"namespace coappearances { // All strings contained in the data separated by `\0`.
    strings: raw_data; }"#;
    }
    pub mod structs {
        pub const INDEX_TYPE32: &str =
            r#"namespace _builtin.multivector { struct IndexType32 { value : u64 : 32; } }"#;
        pub const DEGREE: &str = r#"namespace coappearances { struct Degree {
    value : u32 : 16;
} }"#;
        pub const INVARIANTS: &str = r#"namespace coappearances { struct Invariants {
    max_degree : u32 : 16;
    max_degree_ref : u32 : 16;
    min_degree : u32 : 16;
    min_degree_ref : u32 : 16;
    num_connected_components : u32 : 16;
} }"#;
        pub const CALCULATED_DATA: &str = r#"namespace coappearances { struct Invariants {
    max_degree : u32 : 16;
    max_degree_ref : u32 : 16;
    min_degree : u32 : 16;
    min_degree_ref : u32 : 16;
    num_connected_components : u32 : 16;
} }
namespace coappearances { struct Degree {
    value : u32 : 16;
} }
namespace coappearances { archive CalculatedData {
    invariants : Invariants;
    vertex_degrees : vector< Degree >;
} }"#;
        pub const CHAPTER: &str = r#"namespace coappearances { /**
 * A chapter in the book.
 */
struct Chapter {
    major: u8 : 4;
    minor: u8 : 7;
} }"#;
        pub const BINARY_RELATION: &str = r#"namespace coappearances { /**
 * A relation of a character to two other characters.
 */
struct BinaryRelation {
    kind_ref: u32 : 32;
    to_a_ref: u32 : 16;
    to_b_ref: u32 : 16;
} }"#;
        pub const UNARY_RELATION: &str = r#"namespace coappearances { /**
 * A relation of a character to another one.
 */
struct UnaryRelation {
    kind_ref: u32 : 32;
    to_ref: u32 : 16;
} }"#;
        pub const DESCRIPTION: &str = r#"namespace coappearances { /**
 * A description of a character.
 */
struct Description {
    ref: u32 : 32;
} }"#;
        pub const NICKNAME: &str = r#"namespace coappearances { /**
 * A nickname or an alternative name of a character.
 */
struct Nickname {
    ref: u32 : 32;
} }"#;
        pub const COAPPEARANCE: &str = r#"namespace coappearances { /**
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
} }"#;
        pub const CHARACTER: &str = r#"namespace coappearances { /**
 * A character.
 */
struct Character {
    name_ref : u32 : 32;
} }"#;
        pub const META: &str = r#"namespace coappearances { /**
 * Meta information about the book.
 */
struct Meta {
    title_ref : u32 : 32;
    author_ref : u32 : 32;
} }"#;
        pub const GRAPH: &str = r#"namespace coappearances { /**
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
namespace coappearances { archive CalculatedData {
    invariants : Invariants;
    vertex_degrees : vector< Degree >;
} }
namespace coappearances { struct Invariants {
    max_degree : u32 : 16;
    max_degree_ref : u32 : 16;
    min_degree : u32 : 16;
    min_degree_ref : u32 : 16;
    num_connected_components : u32 : 16;
} }
namespace coappearances { struct Degree {
    value : u32 : 16;
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

    // Optional archive containing calculated statistics.
    @optional
    calculated_data : archive CalculatedData;
} }"#;
    }
}

define_struct!(
    Degree,
    DegreeMut,
    schema::structs::DEGREE,
    2,
    (value, set_value, u32, 0, 16)
);

define_struct!(
    Invariants,
    InvariantsMut,
    schema::structs::INVARIANTS,
    10,
    (max_degree, set_max_degree, u32, 0, 16),
    (max_degree_ref, set_max_degree_ref, u32, 16, 16),
    (min_degree, set_min_degree, u32, 32, 16),
    (min_degree_ref, set_min_degree_ref, u32, 48, 16),
    (
        num_connected_components,
        set_num_connected_components,
        u32,
        64,
        16
    )
);

/// A chapter in the book.
define_struct!(
    Chapter,
    ChapterMut,
    schema::structs::CHAPTER,
    2,
    (major, set_major, u8, 0, 4),
    (minor, set_minor, u8, 4, 7)
);

/// A relation of a character to two other characters.
define_struct!(
    BinaryRelation,
    BinaryRelationMut,
    schema::structs::BINARY_RELATION,
    8,
    (kind_ref, set_kind_ref, u32, 0, 32),
    (to_a_ref, set_to_a_ref, u32, 32, 16),
    (to_b_ref, set_to_b_ref, u32, 48, 16)
);

/// A relation of a character to another one.
define_struct!(
    UnaryRelation,
    UnaryRelationMut,
    schema::structs::UNARY_RELATION,
    6,
    (kind_ref, set_kind_ref, u32, 0, 32),
    (to_ref, set_to_ref, u32, 32, 16)
);

/// A description of a character.
define_struct!(
    Description,
    DescriptionMut,
    schema::structs::DESCRIPTION,
    4,
    (ref_, set_ref, u32, 0, 32)
);

/// A nickname or an alternative name of a character.
define_struct!(
    Nickname,
    NicknameMut,
    schema::structs::NICKNAME,
    4,
    (ref_, set_ref, u32, 0, 32)
);

/// An appearance of two characters in the same scene.
/// count - multiplicity of the coappearance.
/// first_chapter_ref - a reference to the first chapter in which characters
/// appear. How to get the full range of chapters is described in
/// `coappearances.cpp:read`.
define_struct!(
    Coappearance,
    CoappearanceMut,
    schema::structs::COAPPEARANCE,
    8,
    (a_ref, set_a_ref, u32, 0, 16),
    (b_ref, set_b_ref, u32, 16, 16),
    (count, set_count, u32, 32, 16),
    (first_chapter_ref, set_first_chapter_ref, u32, 48, 16)
);

/// A character.
define_struct!(
    Character,
    CharacterMut,
    schema::structs::CHARACTER,
    4,
    (name_ref, set_name_ref, u32, 0, 32)
);

/// Meta information about the book.
define_struct!(
    Meta,
    MetaMut,
    schema::structs::META,
    8,
    (title_ref, set_title_ref, u32, 0, 32),
    (author_ref, set_author_ref, u32, 32, 32)
);

/// Builtin type to for MultiVector index */
define_index!(
    IndexType32,
    IndexType32Mut,
    schema::structs::INDEX_TYPE32,
    4,
    32
);

define_variadic_struct!(VerticesData, VerticesDataItemBuilder, IndexType32,
    0 => (Nickname, add_nickname),
    1 => (Description, add_description),
    2 => (UnaryRelation, add_unary_relation),
    3 => (BinaryRelation, add_binary_relation));

define_archive!(CalculatedData, CalculatedDataBuilder,
    schema::structs::CALCULATED_DATA;
    // struct resources
    (invariants, set_invariants,
        Invariants, schema::resources::INVARIANTS, false);
    // vector resources
    (vertex_degrees, set_vertex_degrees, start_vertex_degrees,
        Degree, schema::resources::VERTEX_DEGREES, false);
    // multivector resources
;
    // raw data resources
;
    // subarchives
);

define_archive!(Graph, GraphBuilder,
    schema::structs::GRAPH;
    // struct resources
    (meta, set_meta,
        Meta, schema::resources::META, false);
    // vector resources
    (vertices, set_vertices, start_vertices,
        Character, schema::resources::VERTICES, false),
    (edges, set_edges, start_edges,
        Coappearance, schema::resources::EDGES, false),
    (chapters, set_chapters, start_chapters,
        Chapter, schema::resources::CHAPTERS, false);
    // multivector resources
    (vertices_data, start_vertices_data,
        VerticesData, schema::resources::VERTICES_DATA,
        vertices_data_index, IndexType32, false);
    // raw data resources
    (strings, set_strings,
        schema::resources::STRINGS, false);
    // subarchives
    (calculated_data,
        CalculatedData, CalculatedDataBuilder,
        schema::resources::CALCULATED_DATA, true));
