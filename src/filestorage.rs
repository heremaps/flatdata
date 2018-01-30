use std::collections::BTreeMap;
use std::path;
use std::fs::File;

use memmap::Mmap;

use storage::{MemoryDescriptor, ResourceStorage};

/// Storage of data in memory mapped files
struct MemoryMappedFileStorage {
    maps: BTreeMap<String, Mmap>,
}

impl MemoryMappedFileStorage {
    pub fn new() -> Self {
        return Self {
            maps: BTreeMap::new(),
        };
    }

    pub fn read(&mut self, path: &str) -> MemoryDescriptor {
        if let Some(mapping) = self.maps.get(path) {
            return MemoryDescriptor::new(mapping.as_ptr(), mapping.len());
        }

        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return MemoryDescriptor::default(),
        };

        let file_mmap = unsafe {
            match Mmap::map(&file) {
                Ok(mmap) => mmap,
                Err(_) => return MemoryDescriptor::default(),
            }
        };

        let mem_descr = MemoryDescriptor::new(file_mmap.as_ptr(), file_mmap.len());
        self.maps.insert(path.into(), file_mmap);

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
