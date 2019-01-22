use crate::storage::{ResourceStorage, Stream};

use memmap::Mmap;

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io;
use std::path::PathBuf;
use std::rc::Rc;
use std::slice;

/// Internal storage of data as files.
#[derive(Debug, Default)]
struct MemoryMappedFileStorage {
    maps: RefCell<BTreeMap<String, Mmap>>,
}

impl MemoryMappedFileStorage {
    pub fn read(&self, path: &str) -> Result<&[u8], io::Error> {
        if !self.maps.borrow().contains_key(path) {
            let file = File::open(path)?;
            let file_mmap = unsafe { Mmap::map(&file)? };
            self.maps.borrow_mut().insert(path.into(), file_mmap);
        }
        let data = &self.maps.borrow()[path];
        // We cannot prove to Rust that the buffer will live as long as the storage
        // (we never delete mappings), so we need to manually extend lifetime
        let extended_lifetime_data = unsafe { slice::from_raw_parts(data.as_ptr(), data.len()) };
        Ok(&extended_lifetime_data)
    }
}

impl Stream for File {}

/// Resource storage on disk using memory mapped files.
#[derive(Debug)]
pub struct FileResourceStorage {
    storage: MemoryMappedFileStorage,
    path: PathBuf,
}

impl FileResourceStorage {
    /// Create an empty memory mapped file storage at a given path.
    #[allow(clippy::new_ret_no_self)]
    pub fn new<P: Into<PathBuf>>(path: P) -> Rc<Self> {
        Rc::new(Self {
            storage: MemoryMappedFileStorage::default(),
            path: path.into(),
        })
    }
}

impl ResourceStorage for FileResourceStorage {
    fn subdir(&self, dir: &str) -> Rc<ResourceStorage> {
        Self::new(self.path.join(dir))
    }

    fn exists(&self, resource_name: &str) -> bool {
        self.path.join(resource_name).exists()
    }

    fn read_resource(&self, resource_name: &str) -> Result<&[u8], io::Error> {
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

    fn create_output_stream(&self, resource_name: &str) -> Result<Rc<RefCell<Stream>>, io::Error> {
        if !self.path.exists() {
            fs::create_dir_all(self.path.clone())?;
        }
        let resource_path = self.path.join(resource_name);
        let file = File::create(resource_path)?;
        Ok(Rc::new(RefCell::new(file)))
    }
}
