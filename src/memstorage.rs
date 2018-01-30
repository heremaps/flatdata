use std::collections::BTreeMap;
use std::path;

use storage::{MemoryDescriptor, ResourceStorage};

/// Storage of data in memory
struct MemoryStorage {
    streams: BTreeMap<path::PathBuf, Vec<u8>>,
    resources: BTreeMap<path::PathBuf, Vec<u8>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            streams: BTreeMap::new(),
            resources: BTreeMap::new(),
        }
    }
}

/// Resource storage in memory
pub struct MemoryResourceStorage {
    storage: MemoryStorage,
    path: path::PathBuf,
}

impl MemoryResourceStorage {
    pub fn new(path: path::PathBuf) -> Self {
        Self {
            storage: MemoryStorage::new(),
            path: path,
        }
    }
}

impl ResourceStorage for MemoryResourceStorage {
    fn read_resource(&mut self, resource_name: &str) -> MemoryDescriptor {
        let resource_path = self.path.join(resource_name);
        if !self.storage.resources.contains_key(&resource_path) {
            let stream = self.storage.streams.get(&resource_path);
            match stream {
                Some(stream) => {
                    self.storage
                        .resources
                        .insert(resource_path.clone(), stream.clone());
                }
                None => {
                    return MemoryDescriptor::default();
                }
            }
        }
        let data: &[u8] = &self.storage.resources[&resource_path];
        MemoryDescriptor::new(&data[0], data.len())
    }
}
