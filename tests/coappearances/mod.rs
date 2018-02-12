use std::cell::RefCell;
use std::convert;
use std::fmt;
use std::rc::Rc;

use flatdata::*;

define_resource_type!(
    Meta,
    "Meta",
    r#"namespace coappearances { /**
 * Meta information about the book.
 */
struct Meta {
    title_ref : u32 : 32;
    author_ref : u32 : 32;
} }
namespace coappearances { @explicit_reference( Meta.title_ref, strings )
    @explicit_reference( Meta.author_ref, strings )
    meta : Meta; }"#,
    8,
    (title_ref, u32, 0, 32),
    (author_ref, u32, 32, 32)
);

define_resource_type!(
    Character,
    "Character",
    r#"namespace coappearances { /**
 * A character.
 */
struct Character {
    name_ref : u32 : 32;
} }
namespace coappearances { @explicit_reference( Character.name_ref, strings )
    vertices : vector< Character >; }"#,
    4,
    (name_ref, u32, 0, 32)
);

define_resource_type!(
    Coappearance,
    "Coappearance",
    r#"namespace coappearances { /**
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
    edges : vector< Coappearance >; }"#,
    8,
    (a_ref, u32, 0, 16),
    (b_ref, u32, 16, 16),
    (count, u32, 32, 16),
    (first_chapter_ref, u32, 48, 16)
);

pub struct Graph {
    /// Holds memory mapped files alive.
    _storage: Rc<RefCell<ResourceStorage>>,
    // generated
    meta: Meta,
    vertices: ArrayView<Character>,
    edges: ArrayView<Coappearance>,
}

impl Graph {
    fn read_resource<R>(
        storage: &mut ResourceStorage,
        name: &str,
        schema: &str,
    ) -> Result<R, ResourceStorageError>
    where
        R: From<MemoryDescriptor>,
    {
        storage.read(name, schema).map(R::from)
    }

    pub fn meta(&self) -> &Meta {
        &self.meta
    }

    pub fn vertices(&self) -> &ArrayView<Character> {
        &self.vertices
    }

    pub fn edges(&self) -> &ArrayView<Coappearance> {
        &self.edges
    }
}

fn signature_name(archive_name: &str) -> String {
    format!("{}.archive", archive_name)
}

impl ArchiveElement for Graph {
    const NAME: &'static str = "Graph";
    const SCHEMA: &'static str = r#"namespace coappearances { /**
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
}

impl Archive for Graph {
    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError> {
        let meta: Meta;
        let vertices;
        let edges;
        {
            let res_storage = &mut *storage.borrow_mut();
            res_storage.read(&signature_name(Self::NAME), Self::SCHEMA)?;
            meta = Self::read_resource(res_storage, "meta", Meta::SCHEMA)
                .map(|mem: MemoryDescriptor| Meta::from(mem.data()))?;
            vertices = Self::read_resource(res_storage, "vertices", Character::SCHEMA)
                .map(|mem| ArrayView::new(&mem))?;
            edges = Self::read_resource(res_storage, "edges", Coappearance::SCHEMA)
                .map(|mem| ArrayView::new(&mem))?;
        }
        Ok(Self {
            _storage: storage,
            meta: meta,
            vertices: vertices,
            edges: edges,
        })
    }

    fn name(&self) -> &str {
        Self::NAME
    }

    fn schema(&self) -> &str {
        Self::SCHEMA
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Graph {{ vertices: {:?}, edges: {:?} }}",
            self.vertices, self.edges
        )
    }
}
