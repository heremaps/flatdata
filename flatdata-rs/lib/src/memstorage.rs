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
#[derive(Default)]
struct MemoryStorage {
    // Streams of resources that were written.
    streams: RefCell<BTreeMap<PathBuf, MemoryStorageStream>>,
    // Data of resources that were opened for reading.
    resources: RefCell<BTreeMap<PathBuf, Rc<Vec<u8>>>>,
}

impl fmt::Debug for MemoryStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MemoryStorage {{ num_streams: {}, num_resources: {} }}",
            self.streams.borrow().len(),
            self.resources.borrow().len(),
        )
    }
}

/// Resource storage in memory.
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
        Self::new(self.path.join(dir))
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
