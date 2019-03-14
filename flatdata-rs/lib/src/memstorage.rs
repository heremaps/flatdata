use crate::storage::{ResourceStorage, Stream};

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::io::{self, Cursor};
use std::path::PathBuf;
use std::rc::Rc;
use std::slice;

type MemoryStorageStream = Rc<RefCell<Cursor<Vec<u8>>>>;

/// Internal storage of data in memory.
#[derive(Default, Clone)]
struct MemoryStorage {
    // Streams of resources that were written.
    streams: Rc<RefCell<BTreeMap<PathBuf, MemoryStorageStream>>>,
    // Data of resources that were opened for reading.
    resources: Rc<RefCell<BTreeMap<PathBuf, Rc<Vec<u8>>>>>,
}

impl fmt::Debug for MemoryStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MemoryStorage {{ streams: {:?}, resources: {:?} }}",
            self.streams
                .borrow()
                .iter()
                .map(|(path, _)| path.display())
                .collect::<Vec<_>>(),
            self.resources
                .borrow()
                .iter()
                .map(|(path, _)| path.display())
                .collect::<Vec<_>>(),
        )
    }
}

/// Resource storage in memory.
///
/// Can be used to create archives in-memory, e.g. when writing tests.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate flatdata;
/// # fn main() {
/// # use flatdata::{ MemoryResourceStorage, Archive, ArchiveBuilder };
/// # define_archive!(X, XBuilder,
/// #     "Schema for X";
/// #     // struct resources
/// # ;
/// #     // vector resources
/// # ;
/// #     // multivector resources
/// # ;
/// #     // raw data resources
/// # ;
/// #     // subarchives
/// # );
/// let storage = MemoryResourceStorage::new("/root/extvec");
/// let _builder = XBuilder::new(storage.clone()).expect("failed to create builder");
/// // Write some data
///
/// let _archive = X::open(storage).expect("failed to open");
/// // read some data
/// # }
/// ```
#[derive(Debug)]
pub struct MemoryResourceStorage {
    storage: MemoryStorage,
    path: PathBuf,
}

impl MemoryResourceStorage {
    /// Create an empty memory resource storage at a given virtual path.
    ///
    /// Resources will be placed in ephemeral memory with prefix `path`. A path
    /// has to be provided to unify the interface with `FileResourceStorage`.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<P: Into<PathBuf>>(path: P) -> Rc<Self> {
        Rc::new(Self {
            storage: MemoryStorage::default(),
            path: path.into(),
        })
    }
}

impl Stream for Cursor<Vec<u8>> {}

impl ResourceStorage for MemoryResourceStorage {
    fn subdir(&self, dir: &str) -> Rc<ResourceStorage> {
        Rc::new(Self {
            storage: self.storage.clone(),
            path: self.path.join(dir),
        })
    }

    fn exists(&self, resource_name: &str) -> bool {
        let resource_path = self.path.join(resource_name);
        self.storage.resources.borrow().contains_key(&resource_path)
            || self.storage.streams.borrow().contains_key(&resource_path)
    }

    fn read_resource(&self, resource_name: &str) -> Result<&[u8], io::Error> {
        let resource_path = self.path.join(resource_name);
        if !self.storage.resources.borrow().contains_key(&resource_path) {
            let streams = self.storage.streams.borrow();
            let stream = streams.get(&resource_path);
            match stream {
                Some(stream) => {
                    // Resource is not yet opened, but there is a stream it was written to
                    // => copy the stream as resource data.
                    let data = Rc::new(stream.borrow().get_ref().clone());
                    self.storage
                        .resources
                        .borrow_mut()
                        .insert(resource_path.clone(), data);
                }
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        String::from(resource_path.to_str().unwrap_or(resource_name)),
                    ));
                }
            }
        }
        let data = &self.storage.resources.borrow()[&resource_path];
        // We cannot prove to Rust that the buffer will live as long as the storage
        // (we never delete mappings), so we need to manually extend lifetime
        let extended_lifetime_data = unsafe { slice::from_raw_parts(data.as_ptr(), data.len()) };
        Ok(&extended_lifetime_data)
    }

    fn create_output_stream(&self, resource_name: &str) -> Result<Rc<RefCell<Stream>>, io::Error> {
        let resource_path = self.path.join(resource_name);
        let stream = self
            .storage
            .streams
            .borrow_mut()
            .entry(resource_path)
            .or_insert_with(|| Rc::new(RefCell::new(Cursor::new(Vec::new()))))
            .clone();
        Ok(stream)
    }
}
