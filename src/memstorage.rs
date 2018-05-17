use storage::{MemoryDescriptor, ResourceStorage, Stream};

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::io::{self, Cursor};
use std::path;
use std::rc::Rc;

/// Internal storage of data in memory.
#[derive(Default)]
struct MemoryStorage {
    // Streams of resources that were written.
    streams: BTreeMap<path::PathBuf, Rc<RefCell<Cursor<Vec<u8>>>>>,
    // Data of resources that were opened for reading.
    resources: BTreeMap<path::PathBuf, Rc<Vec<u8>>>,
}

/// Resource storage in memory.
pub struct MemoryResourceStorage {
    storage: MemoryStorage,
    path: path::PathBuf,
}

impl MemoryResourceStorage {
    /// Create an empty memory resource storage.
    pub fn new(path: path::PathBuf) -> Self {
        Self {
            storage: MemoryStorage::default(),
            path,
        }
    }
}

impl Stream for Cursor<Vec<u8>> {}

impl ResourceStorage for MemoryResourceStorage {
    fn subdir(&self, dir: &str) -> Rc<RefCell<ResourceStorage>> {
        Rc::new(RefCell::new(Self::new(self.path.join(dir))))
    }

    fn exists(&self, resource_name: &str) -> bool {
        let resource_path = self.path.join(resource_name);
        self.storage.resources.contains_key(&resource_path)
            || self.storage.streams.contains_key(&resource_path)
    }

    fn read_resource(&mut self, resource_name: &str) -> Result<MemoryDescriptor, io::Error> {
        let resource_path = self.path.join(resource_name);
        if !self.storage.resources.contains_key(&resource_path) {
            let stream = self.storage.streams.get(&resource_path);
            match stream {
                Some(stream) => {
                    // Resource is not yet opened, but there is a stream it was written to
                    // => copy the stream as resource data.
                    let data = Rc::new(stream.borrow().get_ref().clone());
                    self.storage.resources.insert(resource_path.clone(), data);
                }
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::NotFound,
                        String::from(resource_path.to_str().unwrap_or(resource_name)),
                    ));
                }
            }
        }
        let data: &[u8] = &self.storage.resources[&resource_path];
        Ok(MemoryDescriptor::new(&data[0], data.len()))
    }

    fn create_output_stream(
        &mut self,
        resource_name: &str,
    ) -> Result<Rc<RefCell<Stream>>, io::Error> {
        let resource_path = self.path.join(resource_name);
        let stream = self.storage
            .streams
            .entry(resource_path)
            .or_insert_with(|| Rc::new(RefCell::new(Cursor::new(Vec::new()))));
        Ok(stream.clone())
    }
}
