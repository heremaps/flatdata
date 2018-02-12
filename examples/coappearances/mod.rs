use std::cell::RefCell;
use std::convert;
use std::fmt;
use std::rc::Rc;

use flatdata::*;

define_resource_type!(
    Vertex,
    "Vertex",
    r#"namespace graph { /**
 * A vertex is point in a plane.
 */
struct Vertex {
    x : u32 : 16;
    y : u32 : 16;
} }
namespace graph { vertices : vector< Vertex >; }"#,
    4,
    (x, u32, 0, 16),
    (y, u32, 16, 16)
);

define_resource_type!(
    Edge,
    "Edge",
    r#"namespace graph { /**
 * An edge connects two vertices by referencing their indexes.
 */
struct Edge {
    from_ref : u32 : 16;
    to_ref : u32 : 16;
} }
namespace graph { @explicit_reference( Edge.from_ref, vertices )
    @explicit_reference( Edge.to_ref, vertices )
    edges : vector< Edge >; }"#,
    4,
    (from_ref, u32, 0, 16),
    (to_ref, u32, 16, 16)
);

pub struct Graph {
    /// Holds memory mapped files alive.
    _storage: Rc<RefCell<ResourceStorage>>,
    // generated
    vertices: ArrayView<Vertex>,
    edges: ArrayView<Edge>,
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

    pub fn vertices(&self) -> &ArrayView<Vertex> {
        &self.vertices
    }

    pub fn edges(&self) -> &ArrayView<Edge> {
        &self.edges
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
    fn open(storage: Rc<RefCell<ResourceStorage>>) -> Result<Self, ResourceStorageError> {
        let vertices;
        let edges;
        {
            let res_storage = &mut *storage.borrow_mut();
            res_storage.read(&signature_name(Self::NAME), Self::SCHEMA)?;
            vertices = Self::read_resource(res_storage, "vertices", Vertex::SCHEMA)
                .map(|mem| ArrayView::new(&mem))?;
            edges = Self::read_resource(res_storage, "edges", Edge::SCHEMA)
                .map(|mem| ArrayView::new(&mem))?;
        }
        Ok(Self {
            _storage: storage,
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
