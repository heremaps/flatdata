use std::collections::BTreeMap;
use std::path;

use memmap::{Mmap, Protection};

use storage::{MemoryDescriptor, ResourceStorage};

/// Storage of data in memory mapped files
struct MemoryMappedFileStorage {
    maps: BTreeMap<String, Mmap>,
}

impl MemoryMappedFileStorage {
    pub fn new() -> Self {
        return Self { maps: BTreeMap::new() };
    }

    pub fn read(&mut self, path: &str) -> MemoryDescriptor {
        if let Some(mapping) = self.maps.get(path) {
            return MemoryDescriptor::new(mapping.ptr(), mapping.len());
        }

        let file_mmap = match Mmap::open_path(path, Protection::Read) {
            Ok(mmap) => mmap,
            Err(_) => return MemoryDescriptor::default(),
        };

        let mem_descr = MemoryDescriptor::new(file_mmap.ptr(), file_mmap.len());
        self.maps.insert(String::from(path), file_mmap);
        mem_descr
    }
}

/// Resource storage on disk
pub struct FileResourceStorage {
    storage: MemoryMappedFileStorage,
    path: path::PathBuf,
}

impl FileResourceStorage {
    pub fn new(path: path::PathBuf) -> Self {
        Self {
            storage: MemoryMappedFileStorage::new(),
            path: path,
        }
    }

    pub fn path(&self) -> &path::Path {
        &self.path
    }
}

impl ResourceStorage for FileResourceStorage {
    fn read_resource(&mut self, resource_name: &str) -> MemoryDescriptor {
        let resource_path = self.path.join(resource_name);
        if !resource_path.exists() {
            return MemoryDescriptor::default();
        }

        match resource_path.to_str() {
            Some(p) => self.storage.read(p),
            None => MemoryDescriptor::default(),
        }
    }
}
