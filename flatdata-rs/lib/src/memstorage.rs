use crate::storage::{ResourceStorage, StorageHandle, Stream};

use std::{
    collections::BTreeMap,
    fmt,
    io::{self, Cursor, Read, Seek, Write},
    path::PathBuf,
    slice,
    sync::{Arc, Mutex},
};

type MemoryStorageStream = Arc<Mutex<Cursor<Vec<u8>>>>;

/// Internal storage of data in memory.
#[derive(Default, Clone)]
struct MemoryStorage {
    // Streams of resources that were written.
    streams: Arc<Mutex<BTreeMap<PathBuf, MemoryStorageStream>>>,
    // Data of resources that were opened for reading.
    resources: Arc<Mutex<BTreeMap<PathBuf, Arc<Vec<u8>>>>>,
}

impl fmt::Debug for MemoryStorage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MemoryStorage {{ streams: {:?}, resources: {:?} }}",
            self.streams
                .lock()
                .unwrap()
                .iter()
                .map(|(path, _)| path.display())
                .collect::<Vec<_>>(),
            self.resources
                .lock()
                .unwrap()
                .iter()
                .map(|(path, _)| path.display())
                .collect::<Vec<_>>(),
        )
    }
}

/// Resource storage in memory.
///
/// Used to create and read archives from memory, e.g. for writing tests.
///
/// # Examples
///
/// ```rust
/// use flatdata::{MemoryResourceStorage,  Vector};
/// use flatdata::test::{X, XBuilder};
///
/// let storage = MemoryResourceStorage::new("/root/to/my/archive/in/memory");
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
    pub fn new<P: Into<PathBuf>>(path: P) -> Arc<Self> {
        Arc::new(Self {
            storage: MemoryStorage::default(),
            path: path.into(),
        })
    }
}

impl ResourceStorage for MemoryResourceStorage {
    fn subdir(&self, dir: &str) -> StorageHandle {
        Arc::new(Self {
            storage: self.storage.clone(),
            path: self.path.join(dir),
        })
    }

    fn exists(&self, resource_name: &str) -> bool {
        let resource_path = self.path.join(resource_name);
        self.storage
            .resources
            .lock()
            .unwrap()
            .contains_key(&resource_path)
            || self
                .storage
                .streams
                .lock()
                .unwrap()
                .contains_key(&resource_path)
    }

    fn read_resource(&self, resource_name: &str) -> Result<&[u8], io::Error> {
        let resource_path = self.path.join(resource_name);
        if !self
            .storage
            .resources
            .lock()
            .unwrap()
            .contains_key(&resource_path)
        {
            let streams = self.storage.streams.lock().unwrap();
            let stream = streams.get(&resource_path);
            match stream {
                Some(stream) => {
                    // Resource is not yet opened, but there is a stream it was written to
                    // => copy the stream as resource data.
                    let data = Arc::new(stream.lock().unwrap().get_ref().clone());
                    self.storage
                        .resources
                        .lock()
                        .unwrap()
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
        let data = &self.storage.resources.lock().unwrap()[&resource_path];
        // We cannot prove to Rust that the buffer will live as long as the storage
        // (we never delete mappings), so we need to manually extend lifetime
        let extended_lifetime_data = unsafe { slice::from_raw_parts(data.as_ptr(), data.len()) };
        Ok(&extended_lifetime_data)
    }

    fn create_output_stream(&self, resource_name: &str) -> Result<Box<dyn Stream>, io::Error> {
        let resource_path = self.path.join(resource_name);
        let stream = self
            .storage
            .streams
            .lock()
            .unwrap()
            .entry(resource_path)
            .or_insert_with(|| Arc::new(Mutex::new(Cursor::new(Vec::new()))))
            .clone();
        Ok(Box::new(StreamWrapper { stream }))
    }
}

struct StreamWrapper {
    stream: Arc<Mutex<Cursor<Vec<u8>>>>,
}

impl Read for StreamWrapper {
    fn read(&mut self, result: &mut [u8]) -> std::result::Result<usize, std::io::Error> {
        self.stream.lock().unwrap().read(result)
    }
}

impl Seek for StreamWrapper {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::result::Result<u64, std::io::Error> {
        self.stream.lock().unwrap().seek(pos)
    }
}

impl Write for StreamWrapper {
    fn write(&mut self, data: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.stream.lock().unwrap().write(data)
    }

    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        self.stream.lock().unwrap().flush()
    }
}
