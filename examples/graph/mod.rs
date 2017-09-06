use std::cell::RefCell;
use std::convert;
use std::fmt;
use std::rc::Rc;

use flatdata::*;

pub struct Vertex {
    data: StreamType,
}

impl Vertex {
    pub fn x(&self) -> u32 {
        read_bytes!(u32, self.data, 0, 16)
    }

    pub fn y(&self) -> u32 {
        read_bytes!(u32, self.data, 16, 16)
    }
}

impl ArchiveElement for Vertex {
    const NAME: &'static str = "Vertex";
    const SCHEMA: &'static str = r#"namespace graph { /**
 * A vertex is point in a plane.
 */
struct Vertex {
    x : u32 : 16;
    y : u32 : 16;
} }
namespace graph { vertices : vector< Vertex >; }"#;
}

impl ArchiveType for Vertex {
    const SIZE_IN_BYTES: usize = 4;
}

impl convert::From<StreamType> for Vertex {
    fn from(data: StreamType) -> Vertex {
        Vertex { data: data }
    }
}

impl fmt::Debug for Vertex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vertex {{ x: {}, y: {} }}", self.x(), self.y())
    }
}

pub struct Edge {
    data: StreamType,
}

impl Edge {
    pub fn from_ref(&self) -> u32 {
        read_bytes!(u32, self.data, 0, 16)
    }

    pub fn to_ref(&self) -> u32 {
        read_bytes!(u32, self.data, 16, 16)
    }
}

impl ArchiveElement for Edge {
    const NAME: &'static str = "Edge";
    const SCHEMA: &'static str = r#"namespace graph { /**
 * An edge connects two vertices by referencing their indexes.
 */
struct Edge {
    from_ref : u32 : 16;
    to_ref : u32 : 16;
} }
namespace graph { @explicit_reference( Edge.from_ref, vertices )
    @explicit_reference( Edge.to_ref, vertices )
    edges : vector< Edge >; }"#;
}

impl ArchiveType for Edge {
    const SIZE_IN_BYTES: usize = 4;
}

impl convert::From<StreamType> for Edge {
    fn from(data: StreamType) -> Edge {
        Edge { data: data }
    }
}

impl fmt::Debug for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Edge {{ from_ref: {}, to_ref: {} }}", self.from_ref(), self.to_ref())
    }
}

pub struct Graph {
    /// Holds memory mapped files alive.
    _storage: Rc<RefCell<ResourceStorage>>,
    signature: Option<MemoryDescriptor>,
    // generared
    vertices: Option<ArrayView<Vertex>>,
    edges: Option<ArrayView<Edge>>,
}

impl Graph {
    fn read_resource<R>(
        storage: Rc<RefCell<ResourceStorage>>,
        name: &str,
        schema: &str,
    ) -> Option<R>
    where
        R: From<MemoryDescriptor>,
    {
        let mut res_storage = storage.borrow_mut();
        res_storage.read(name, schema).map(R::from)
    }

    pub fn vertices(&self) -> &ArrayView<Vertex> {
        self.vertices.as_ref().unwrap()
    }

    pub fn edges(&self) -> &ArrayView<Edge> {
        self.edges.as_ref().unwrap()
    }
}

fn signature_name(archive_name: &str) -> String {
    format!("{}.archive", archive_name)
}

impl ArchiveElement for Graph {
    const NAME: &'static str = "Graph";
    const SCHEMA: &'static str = r#"namespace graph { /**
 * A vertex is point in a plane.
 */
struct Vertex {
    x : u32 : 16;
    y : u32 : 16;
} }
namespace graph { /**
 * An edge connects two vertices by referencing their indexes.
 */
struct Edge {
    from_ref : u32 : 16;
    to_ref : u32 : 16;
} }
namespace graph { struct EdgeWeight {
    value: u32 : 16;
} }
namespace graph { struct EdgeAttribute {
    is_bidirectional : u8 : 1;
    multiplicity : u8 : 7;
} }
namespace _builtin.multivector { struct IndexType32 { value : u64 : 32; } }
namespace graph { archive Graph {
    vertices : vector< Vertex >;

    @explicit_reference( Edge.from_ref, vertices )
    @explicit_reference( Edge.to_ref, vertices )
    edges : vector< Edge >;

    edge_attributes: multivector< 32, EdgeWeight, EdgeAttribute >;
} }"#;
}

impl Archive for Graph {
    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Self {
        let signature;
        {
            let mut res_storage = storage.borrow_mut();
            signature = res_storage.read(&signature_name(Self::NAME), Self::SCHEMA);
        }
        let vertices = Self::read_resource(storage.clone(), "vertices", Vertex::SCHEMA)
            .map(ArrayView::new);
        let edges = Self::read_resource(storage.clone(), "edges", Edge::SCHEMA).map(ArrayView::new);
        Self {
            _storage: storage,
            signature: signature,
            vertices: vertices,
            edges: edges,
        }
    }

    fn is_open(&self) -> bool {
        self.signature.is_some() && self.vertices.is_some() && self.edges.is_some()
    }

    fn describe(&self) -> String {
        String::from("TODO")
    }
}

impl fmt::Debug for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Graph {{ vertices: {:?}, edges: {:?} }}", self.vertices, self.edges)
    }
}
