use crate::storage::{ResourceStorage, StorageHandle, Stream};

use memmap::Mmap;

use std::{
    collections::BTreeMap,
    fs::{self, File},
    io,
    path::PathBuf,
    slice,
    sync::{Arc, Mutex},
};

/// Internal storage of data as files.
#[derive(Debug, Default)]
struct MemoryMappedFileStorage {
    maps: Mutex<BTreeMap<String, Mmap>>,
}

impl MemoryMappedFileStorage {
    pub fn read(&self, path: &str) -> Result<&[u8], io::Error> {
        if !self.maps.lock().unwrap().contains_key(path) {
            let file = File::open(path)?;
            let file_mmap = unsafe { Mmap::map(&file)? };
            self.maps.lock().unwrap().insert(path.into(), file_mmap);
        }
        let data = &self.maps.lock().unwrap()[path];
        // We cannot prove to Rust that the buffer will live as long as the storage
        // (we never delete mappings), so we need to manually extend lifetime
        let extended_lifetime_data = unsafe { slice::from_raw_parts(data.as_ptr(), data.len()) };
        Ok(&extended_lifetime_data)
    }
}

/// Resource storage on disk using memory mapped files.
///
/// Used to create and read archives from the file system.
///
/// # Examples
///
/// ```rust,no_run
/// use flatdata::{FileResourceStorage,  Vector};
/// use flatdata::test::{X, XBuilder};
///
/// let storage = FileResourceStorage::new("/root/to/my/archive");
/// let builder = XBuilder::new(storage.clone()).expect("failed to create builder");
/// // Write some data and store it archive, e.g.
/// let v = Vector::new();
/// builder.set_data(&v.as_view());
///
/// let archive = X::open(storage).expect("failed to open");
/// // read data
/// archive.data();
/// ```
#[derive(Debug)]
pub struct FileResourceStorage {
    storage: MemoryMappedFileStorage,
    path: PathBuf,
}

impl FileResourceStorage {
    /// Create an empty memory mapped file storage at a given path.
    pub fn new<P: Into<PathBuf>>(path: P) -> Arc<Self> {
        Arc::new(Self {
            storage: MemoryMappedFileStorage::default(),
            path: path.into(),
        })
    }
}

impl ResourceStorage for FileResourceStorage {
    fn subdir(&self, dir: &str) -> StorageHandle {
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

    fn create_output_stream(&self, resource_name: &str) -> Result<Box<dyn Stream>, io::Error> {
        if !self.path.exists() {
            fs::create_dir_all(self.path.clone())?;
        }
        let resource_path = self.path.join(resource_name);
        let file = File::create(resource_path)?;
        Ok(Box::new(file))
    }
}
