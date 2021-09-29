use crate::storage::{ResourceStorage, StorageHandle, Stream};

use memmap2::Mmap;
use std::{
    collections::HashMap,
    fs::File,
    io,
    ops::Range,
    path::{Path, PathBuf},
    slice,
    sync::Arc,
};

/// Internal storage of file entries in tar archive.
#[derive(Debug)]
struct MemoryMappedTarArchiveStorage {
    archive_map: Mmap,
    file_ranges: HashMap<PathBuf, Range<usize>>,
}

impl MemoryMappedTarArchiveStorage {
    pub fn new(tar_path: &Path) -> Result<Self, io::Error> {
        let file = File::open(tar_path)?;
        let archive_map = unsafe { Mmap::map(&file)? };
        let mut archive = tar::Archive::new(&archive_map[..]);

        let file_ranges = archive
            .entries()?
            .map(|entry| {
                let entry = entry?;
                let path = entry.path()?;
                let path = if let Ok(stripped_path) = path.strip_prefix(".") {
                    stripped_path.to_path_buf()
                } else {
                    path.to_path_buf()
                };
                let offset = entry.raw_file_position() as usize;
                let size = entry.size() as usize;
                if entry.header().entry_size()? != entry.size() {
                    // We can only memory-map contiguous files
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Sparse files are not supported",
                    ));
                }

                Ok((path, offset..offset + size))
            })
            .collect::<Result<HashMap<PathBuf, Range<usize>>, io::Error>>()?;

        Ok(Self {
            archive_map,
            file_ranges,
        })
    }

    pub fn read(&self, path: &Path) -> Option<&[u8]> {
        self.file_ranges.get(path).map(|range| {
            // We cannot prove to Rust that the buffer will live as long as the storage
            // (we never delete mappings), so we need to manually extend lifetime
            let extended_lifetime_archive_map =
                unsafe { slice::from_raw_parts(self.archive_map.as_ptr(), self.archive_map.len()) };

            &extended_lifetime_archive_map[range.clone()]
        })
    }
}

/// Read-only resource storage on disk using a memory mapped tar archive.
///
/// Used to read flatdata archives from a tar archive on disk.
///
/// # Examples
///
/// ```rust,no_run
/// use flatdata::{TarArchiveResourceStorage, Vector};
/// use flatdata::test::X;
///
/// let storage = TarArchiveResourceStorage::new("/root/to/my/archive.tar")
///     .expect("failed to read tar archive");
/// let archive = X::open(storage).expect("failed to open");
/// // read data
/// archive.data();
/// ```
#[derive(Debug)]
pub struct TarArchiveResourceStorage {
    storage: Arc<MemoryMappedTarArchiveStorage>,
    sub_path: PathBuf,
}

impl TarArchiveResourceStorage {
    /// Create a memory mapped tar archive resource storage for a tar archive at a given path.
    pub fn new<P: Into<PathBuf>>(tar_path: P) -> Result<Arc<Self>, io::Error> {
        Ok(Arc::new(Self {
            storage: Arc::new(MemoryMappedTarArchiveStorage::new(&tar_path.into())?),
            sub_path: PathBuf::new(),
        }))
    }
}

impl ResourceStorage for TarArchiveResourceStorage {
    fn subdir(&self, dir: &str) -> StorageHandle {
        Arc::new(Self {
            storage: self.storage.clone(),
            sub_path: self.sub_path.join(dir),
        })
    }

    fn exists(&self, resource_name: &str) -> bool {
        self.storage
            .read(&self.sub_path.join(resource_name))
            .is_some()
    }

    fn read_resource(&self, resource_name: &str) -> Result<&[u8], io::Error> {
        let resource_path = self.sub_path.join(resource_name);
        if let Some(data) = self.storage.read(&resource_path) {
            Ok(data)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                String::from(resource_path.to_str().unwrap_or(resource_name)),
            ))
        }
    }

    fn create_output_stream(&self, _resource_name: &str) -> Result<Box<dyn Stream>, io::Error> {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "Writing to tar archives is not supported",
        ))
    }
}
