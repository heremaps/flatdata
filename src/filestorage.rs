use std::collections::BTreeMap;
use std::path;
use std::fs::File;
use std::io;
use std::rc::Rc;
use std::cell::RefCell;

use memmap::Mmap;

use storage::{MemoryDescriptor, ResourceStorage, Stream};

/// Storage of data in memory mapped files
struct MemoryMappedFileStorage {
    maps: BTreeMap<String, Mmap>,
}

impl MemoryMappedFileStorage {
    pub fn new() -> Self {
        Self {
            maps: BTreeMap::new(),
        }
    }

    pub fn read(&mut self, path: &str) -> Result<MemoryDescriptor, io::Error> {
        if let Some(mapping) = self.maps.get(path) {
            return Ok(MemoryDescriptor::new(mapping.as_ptr(), mapping.len()));
        }

        let file = File::open(path)?;

        let file_mmap = unsafe { Mmap::map(&file)? };

        let mem_descr = MemoryDescriptor::new(file_mmap.as_ptr(), file_mmap.len());
        self.maps.insert(path.into(), file_mmap);

        Ok(mem_descr)
    }
}

impl Stream for File {}

/// Resource storage on disk
pub struct FileResourceStorage {
    storage: MemoryMappedFileStorage,
    path: path::PathBuf,
}

impl FileResourceStorage {
    pub fn new(path: path::PathBuf) -> Self {
        Self {
            storage: MemoryMappedFileStorage::new(),
            path,
        }
    }

    pub fn path(&self) -> &path::Path {
        &self.path
    }
}

impl ResourceStorage for FileResourceStorage {
    fn read_resource(&mut self, resource_name: &str) -> Result<MemoryDescriptor, io::Error> {
        let resource_path = self.path.join(resource_name);
        if !resource_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                String::from(resource_path.to_str().unwrap_or(resource_name)),
            ));
        }

        match resource_path.to_str() {
            Some(p) => self.storage.read(p),
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                String::from(resource_path.to_str().unwrap_or(resource_name)),
            )),
        }
    }

    fn create_output_stream(
        &mut self,
        resource_name: &str,
    ) -> Result<Rc<RefCell<Stream>>, io::Error> {
        let resource_path = self.path.join(resource_name);
        if !resource_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                String::from(resource_path.to_str().unwrap_or(resource_name)),
            ));
        }
        let file = File::create(resource_path)?;
        Ok(Rc::new(RefCell::new(file)))
    }
}
