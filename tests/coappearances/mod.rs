use flatdata::Struct;

mod schema {
    pub mod resources {
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

        pub const CHAPTERS: &str = r#"namespace coappearances { /**
 * A chapter in the book.
 */
struct Chapter {
    major: u8 : 4;
    minor: u8 : 7;
} }
namespace coappearances { chapters : vector< Chapter >; }"#;

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

        pub const VERTICES_DATA_INDEX: &str = r#"index(namespace coappearances { /**
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

        pub const STRINGS: &str =
            r#"namespace coappearances { // All strings contained in the data separated by `\0`.
    strings: raw_data; }"#;

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
    num_vertices : u32 : 16;
    num_edges : u32 : 16;
    max_degree : u32 : 16;
    min_degree : u32 : 16;
    num_connected_components : u32 : 16;
} }
namespace coappearances { struct Degree {
    value : u32;
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
    calculated_data : archive CalculatedData;
} }"#;
    }

    pub mod structs {
        // stubs
        pub const META: &str = "";
        pub const CHARACTER: &str = "";
        pub const COAPPEARANCE: &str = "";
        pub const CHAPTER: &str = "";
        pub const NICKNAME: &str = "";
        pub const DESCRIPTION: &str = "";
        pub const UNARYRELATION: &str = "";
        pub const BINARYRELATION: &str = "";
        pub const INDEXTYPE32: &str = "";
    }
}

define_struct!(
    Character,
    CharacterMut,
    ::coappearances::schema::structs::CHARACTER,
    4,
    (name_ref, set_name_ref, u32, 0, 32)
);

define_struct!(
    Meta,
    MetaMut,
    ::coappearances::schema::structs::META,
    8,
    (title_ref, set_title_ref, u32, 0, 32),
    (author_ref, set_author_ref, u32, 32, 32)
);

define_struct!(
    Coappearance,
    CoappearanceMut,
    ::coappearances::schema::structs::COAPPEARANCE,
    8,
    (a_ref, set_a_ref, u32, 0, 16),
    (b_ref, set_b_ref, u32, 16, 16),
    (count, set_count, u32, 32, 16),
    (first_chapter_ref, set_first_chapter_ref, u32, 48, 16)
);

define_struct!(
    Chapter,
    ChapterMut,
    ::coappearances::schema::structs::CHAPTER,
    2,
    (major, set_major, u8, 0, 4),
    (minor, set_minor, u8, 4, 7)
);

// TODO: Resolve ref clashing with keywords of Rust.
define_struct!(
    Nickname,
    NicknameMut,
    ::coappearances::schema::structs::NICKNAME,
    4,
    (ref_, set_ref_, u32, 0, 32)
);

define_struct!(
    Description,
    DescriptionMut,
    ::coappearances::schema::structs::DESCRIPTION,
    4,
    (ref_, set_ref_, u32, 0, 32)
);

define_struct!(
    UnaryRelation,
    UnaryRelationMut,
    ::coappearances::schema::structs::UNARYRELATION,
    6,
    (kind_ref, set_kind_ref, u32, 0, 32),
    (to_ref, set_to_ref, u32, 32, 16)
);

define_struct!(
    BinaryRelation,
    BinaryRelationMut,
    ::coappearances::schema::structs::BINARYRELATION,
    8,
    (kind_ref, set_kind_ref, u32, 0, 32),
    (to_a_ref, set_to_a_ref, u32, 32, 16),
    (to_b_ref, set_to_b_ref, u32, 48, 16)
);

define_variadic_struct!(VerticesData, VerticesDataItemBuilder, IndexType32,
    0 => (Nickname, add_nickname),
    1 => (Description, add_description),
    2 => (UnaryRelation, add_unary_relation),
    3 => (BinaryRelation, add_binary_relation));

define_index!(
    IndexType32,
    IndexType32Mut,
    ::coappearances::schema::structs::INDEXTYPE32,
    4,
    32
);

define_archive!(Graph, GraphBuilder, ::coappearances::schema::resources::GRAPH;
    // struct resources
    (meta, set_meta,
        Meta, ::coappearances::schema::resources::META);
    // vector resources
    (vertices, set_vertices, start_vertices,
        Character, ::coappearances::schema::resources::VERTICES),
    (edges, set_edges, start_edges,
        Coappearance, ::coappearances::schema::resources::EDGES),
    (chapters, set_chapters, start_chapters,
        Chapter, ::coappearances::schema::resources::CHAPTERS);
    // multivector resources
    (vertices_data, start_vertices_data,
        VerticesData, ::coappearances::schema::resources::VERTICES_DATA,
        vertices_data_index, internal::IndexType32,
        ::coappearances::schema::resources::VERTICES_DATA_INDEX);
    // raw data resources
    (strings, set_strings, ::coappearances::schema::resources::STRINGS)
);
