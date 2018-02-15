use std::cell::RefCell;
use std::convert;
use std::fmt;
use std::rc::Rc;
use std::slice;

use flatdata::*;

const META_SCHEMA: &'static str = r#"namespace coappearances { /**
 * Meta information about the book.
 */
struct Meta {
    title_ref : u32 : 32;
    author_ref : u32 : 32;
} }
namespace coappearances { @explicit_reference( Meta.title_ref, strings )
    @explicit_reference( Meta.author_ref, strings )
    meta : Meta; }"#;

define_archive_type!(Meta, 8, (title_ref, u32, 0, 32), (author_ref, u32, 32, 32));

const CHARACTER_SCHEMA: &'static str = r#"namespace coappearances { /**
 * A character.
 */
struct Character {
    name_ref : u32 : 32;
} }
namespace coappearances { @explicit_reference( Character.name_ref, strings )
    vertices : vector< Character >; }"#;

define_archive_type!(Character, 4, (name_ref, u32, 0, 32));

const COAPPEARANCE_SCHEMA: &'static str = r#"namespace coappearances { /**
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

define_archive_type!(
    Coappearance,
    8,
    (a_ref, u32, 0, 16),
    (b_ref, u32, 16, 16),
    (count, u32, 32, 16),
    (first_chapter_ref, u32, 48, 16)
);

const CHAPTER_SCHEMA: &'static str = r#"namespace coappearances { /**
 * A chapter in the book.
 */
struct Chapter {
    major: u8 : 4;
    minor: u8 : 7;
} }
namespace coappearances { chapters : vector< Chapter >; }"#;

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

pub enum VerticesData {
    Nickname(Nickname),
    Description(Description),
    UnaryRelation(UnaryRelation),
    BinaryRelation(BinaryRelation),
}

const VERTICESDATA_SCHEMA: &'static str = r#"namespace coappearances { /**
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

impl VariadicArchiveType for VerticesData {
    fn size_in_bytes(&self) -> usize {
        match *self {
            VerticesData::Nickname(_) => Nickname::SIZE_IN_BYTES,
            VerticesData::Description(_) => Description::SIZE_IN_BYTES,
            VerticesData::UnaryRelation(_) => UnaryRelation::SIZE_IN_BYTES,
            VerticesData::BinaryRelation(_) => BinaryRelation::SIZE_IN_BYTES,
        }
    }
}

impl convert::From<(u8, StreamType)> for VerticesData {
    fn from((type_index, data): (u8, StreamType)) -> VerticesData {
        match type_index {
            0 => VerticesData::Nickname(Nickname::from(data)),
            1 => VerticesData::Description(Description::from(data)),
            2 => VerticesData::UnaryRelation(UnaryRelation::from(data)),
            3 => VerticesData::BinaryRelation(BinaryRelation::from(data)),
            _ => panic!("invalid type index"),
        }
    }
}

impl fmt::Debug for VerticesData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VerticesData::Nickname(ref inner) => write!(f, "{:?}", inner),
            VerticesData::Description(ref inner) => write!(f, "{:?}", inner),
            VerticesData::UnaryRelation(ref inner) => write!(f, "{:?}", inner),
            VerticesData::BinaryRelation(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

const VERTICES_DATA_INDEX_SCHEMA: &'static str = r#"index(namespace coappearances { /**
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

// TODO: Do not expose this type.
define_archive_type!(IndexType32, 4, (value, u64, 0, 32));

impl IndexType for IndexType32 {
    fn value(&self) -> usize {
        self.value() as usize
    }
}

const STRINGS_SCHEMA: &'static str =
    r#"namespace coappearances { // All strings contained in the data separated by `\0`.
    strings: raw_data; }"#;

pub struct Graph {
    /// Holds memory mapped files alive.
    _storage: Rc<RefCell<ResourceStorage>>,
    // generated
    meta: Meta,
    vertices: ArrayView<Character>,
    edges: ArrayView<Coappearance>,
    vertices_data: MultiArrayView<IndexType32, VerticesData>,
    chapters: ArrayView<Chapter>,
    strings: MemoryDescriptor,
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

    pub fn vertices_data(&self) -> &MultiArrayView<IndexType32, VerticesData> {
        &self.vertices_data
    }

    pub fn chapters(&self) -> &ArrayView<Chapter> {
        &self.chapters
    }

    pub fn strings(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.strings.data(), self.strings.size_in_bytes()) }
    }
}

fn signature_name(archive_name: &str) -> String {
    format!("{}.archive", archive_name)
}

impl Archive for Graph {
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

    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError> {
        let meta;
        let vertices;
        let edges;
        let vertices_data;
        let chapters;
        let strings;
        {
            let res_storage = &mut *storage.borrow_mut();
            res_storage.read(&signature_name(Self::NAME), Self::SCHEMA)?;
            meta = Self::read_resource(res_storage, "meta", META_SCHEMA)
                .map(|mem: MemoryDescriptor| Meta::from(mem.data()))?;
            vertices = Self::read_resource(res_storage, "vertices", CHARACTER_SCHEMA)
                .map(|mem| ArrayView::new(&mem))?;
            edges = Self::read_resource(res_storage, "edges", COAPPEARANCE_SCHEMA)
                .map(|mem| ArrayView::new(&mem))?;
            let vertices_data_index = Self::read_resource(
                res_storage,
                "vertices_data_index",
                VERTICES_DATA_INDEX_SCHEMA,
            ).map(|mem| ArrayView::new(&mem))?;
            vertices_data = Self::read_resource(res_storage, "vertices_data", VERTICESDATA_SCHEMA)
                .map(|mem| MultiArrayView::new(vertices_data_index, &mem))?;
            chapters = Self::read_resource(res_storage, "chapters", CHAPTER_SCHEMA)
                .map(|mem| ArrayView::new(&mem))?;
            strings = Self::read_resource(res_storage, "strings", STRINGS_SCHEMA)?;
        }
        Ok(Self {
            _storage: storage,
            meta: meta,
            vertices: vertices,
            edges: edges,
            vertices_data: vertices_data,
            chapters: chapters,
            strings: strings,
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
            "Graph {{ meta: {:?}, vertices: {:?}, edges: {:?}, vertices_data: {:?}, chapters: {:?}, strings: {:?} }}",
            self.meta, self.vertices, self.edges, self.vertices_data, self.chapters, self.strings
        )
    }
}
