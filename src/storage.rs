use archive::{ArchiveBuilder, IndexStruct, Struct, VariadicStruct};
use error::ResourceStorageError;
use memory::{SizeType, PADDING_SIZE};
use multivector::MultiVector;
use vector::ExternalVector;

use std::cell::RefCell;
use std::fmt;
use std::io::{self, Seek, Write};
use std::mem;
use std::ops::DerefMut;
use std::ptr;
use std::rc::Rc;
use std::slice;
use std::str;
use std::thread;

pub trait Stream: Write + Seek {}

/// Hierarchical Resource Storage
///
/// Manages and returns resources corresponding to their keys. Keys can be
/// slash-separated('/'). Manages schema for each resource and checks it on
/// query. Resource storage is expected to provide read-write access to
/// resources.
pub trait ResourceStorage {
    /// Open a flatdata resource with given name and schema for reading.
    ///
    /// Also checks if the schema matches the stored schema in the storage. The
    /// schema is expected to be stored in the storage as another resource
    /// with name `{resource_name}.schema`.
    fn read(&self, resource_name: &str, schema: &str) -> Result<&[u8], ResourceStorageError> {
        self.read_and_check_schema(resource_name, schema)
    }

    /// Writes data of a flatdata resource with given name and schema to
    /// storage.
    ///
    /// The schema will be stored as another resource under the name
    /// `{resource_name}.schema`.
    fn write(&self, resource_name: &str, schema: &str, data: &[u8]) -> io::Result<()> {
        // write data
        let stream = self.create_output_stream(resource_name)?;
        let mut mut_stream = stream.borrow_mut();
        write_to_stream(data, mut_stream.deref_mut())?;
        // write schema
        let schema_name = format!("{}.schema", resource_name);
        let stream = self.create_output_stream(&schema_name)?;
        let mut mut_stream = stream.borrow_mut();
        write_schema(schema, mut_stream.deref_mut())
    }

    //
    // Virtual
    //

    /// Creates a resource storage at a given subdirectory.
    fn subdir(&self, dir: &str) -> Rc<ResourceStorage>;

    /// Returns `true` if resource exists in the storage.
    fn exists(&self, resource_name: &str) -> bool;

    /// Reads a resource in storage and returns a pointer to its raw data.
    ///
    /// This is a low level facility for opening and reading resources. Cf.
    /// [`read`] for opening flatdata resources and checking the
    /// corresponding schema.
    ///
    /// [`read`]: #method.read
    fn read_resource(&self, resource_name: &str) -> Result<&[u8], io::Error>;

    /// Creates a resource with given name and returns an output stream for
    /// writing to it.
    fn create_output_stream(&self, resource_name: &str) -> io::Result<Rc<RefCell<Stream>>>;

    //
    // Implementation helper
    //

    /// Implementation helper for [`read`].
    ///
    /// Uses the required method [`read_resource`] for open the corresponding
    /// resource and its schema. It checks the integrity of data by
    /// verifying that the size of resource matched the size specified in
    /// the header. Also checks that the stored schema matches the provided
    /// schema.
    ///
    /// [`read`]: #method.read
    /// [`read_resource`]: #tymethod.read_resource
    fn read_and_check_schema(
        &self,
        resource_name: &str,
        expected_schema: &str,
    ) -> Result<&[u8], ResourceStorageError> {
        let data = self
            .read_resource(resource_name)
            .map_err(|e| ResourceStorageError::from_io_error(e, resource_name.into()))?;

        let schema_name = format!("{}.schema", resource_name);
        let schema = self
            .read_resource(&schema_name)
            .map_err(|e| ResourceStorageError::from_io_error(e, resource_name.into()))?;

        if data.len() < mem::size_of::<SizeType>() + PADDING_SIZE {
            return Err(ResourceStorageError::UnexpectedDataSize);
        }

        let size = read_bytes!(SizeType, data.as_ptr()) as usize;
        if size + mem::size_of::<SizeType>() + PADDING_SIZE != data.len() {
            return Err(ResourceStorageError::UnexpectedDataSize);
        }

        let stored_schema_slice: &[u8] = schema;
        let stored_schema =
            str::from_utf8(stored_schema_slice).map_err(ResourceStorageError::Utf8Error)?;
        if stored_schema != expected_schema {
            return Err(ResourceStorageError::WrongSignature {
                resource_name: resource_name.into(),
                diff: diff(stored_schema, expected_schema),
            });
        }

        Ok(&data[mem::size_of::<SizeType>()..][..size])
    }
}

//
// Resource factory helpers
//

/// Helper for creating an external vector in the given resource storage.
///
/// Creates a new resource with given name and schema in storage, and returns
/// an [`ExternalVector`] using this resource for writing and flushing data to
/// storage.
pub fn create_external_vector<'a, T>(
    storage: &'a ResourceStorage,
    resource_name: &str,
    schema: &str,
) -> io::Result<ExternalVector<'a, T>>
where
    T: for<'b> Struct<'b>,
{
    // write schema
    let schema_name = format!("{}.schema", resource_name);
    let stream = storage.create_output_stream(&schema_name)?;
    stream.borrow_mut().write_all(schema.as_bytes())?;

    // create external vector
    let data_writer = storage.create_output_stream(resource_name)?;
    let handle = ResourceHandle::new(storage, resource_name.into(), schema.into(), data_writer)?;
    Ok(ExternalVector::new(handle))
}

/// Helper for creating a multivector in the given resource storage.
///
/// Creates a new resource with given name and schema in storage, and returns
/// an [`MultiVector`] using this resource for writing and flushing data to
/// storage.
pub fn create_multi_vector<'a, Idx, Ts>(
    storage: &'a ResourceStorage,
    resource_name: &str,
    schema: &str,
) -> io::Result<MultiVector<'a, Idx, Ts>>
where
    Idx: for<'b> IndexStruct<'b>,
    Ts: for<'b> VariadicStruct<'b>,
{
    // create index
    let index_name = format!("{}_index", resource_name);
    let index_schema = format!("index({})", schema);
    let index = create_external_vector(storage, &index_name, &index_schema)?;

    // write schema
    let schema_name = format!("{}.schema", resource_name);
    let stream = storage.create_output_stream(&schema_name)?;
    stream.borrow_mut().write_all(schema.as_bytes())?;

    // create multi vector
    let data_writer = storage.create_output_stream(resource_name)?;
    let handle = ResourceHandle::new(storage, resource_name.into(), schema.into(), data_writer)?;
    Ok(MultiVector::new(index, handle))
}

/// Creates a new archive in resource storage.
///
/// A resource with name `T::NAME` is created in the storage. Its content is
/// the signature of the archive, i.e. `T::SCHEMA`.
///
/// # Errors
///
/// If an archive with the same name already exists in the storage, then an IO
/// error of kind [`AlreadyExists`] is returned.
///
/// [`AlreadyExists`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#AlreadyExists.v
pub fn create_archive<T: ArchiveBuilder>(
    storage: &Rc<ResourceStorage>,
) -> Result<(), ResourceStorageError> {
    let signature_name = format!("{}.archive", T::NAME);
    {
        // existing archive yields an error
        if storage.exists(&signature_name) {
            return Err(ResourceStorageError::from_io_error(
                io::Error::new(io::ErrorKind::AlreadyExists, signature_name.clone()),
                signature_name,
            ));
        }
    }
    {
        // write empty signature and schema
        storage
            .write(&signature_name, T::SCHEMA, &[])
            .map_err(|e| ResourceStorageError::from_io_error(e, signature_name))?;
    }
    Ok(())
}

/// Describes a chunk of memory
#[derive(Debug, Clone)]
pub struct MemoryDescriptor {
    ptr: *const u8,
    size: usize,
}

impl Default for MemoryDescriptor {
    fn default() -> MemoryDescriptor {
        MemoryDescriptor {
            ptr: ptr::null(),
            size: 0,
        }
    }
}

/// Describes a contiguous constant chunk of memory without tracking lifetime
/// Used to enabling caching files retrieved from ResourceStorage without
/// Sibling-Borrow woes
impl MemoryDescriptor {
    /// Creates a new memory descriptor from a pointer and its size in bytes.
    pub fn new(data: &[u8]) -> MemoryDescriptor {
        MemoryDescriptor {
            ptr: data.as_ptr(),
            size: data.len(),
        }
    }

    /// Converts back to bytes (lifetime corresponds to the descriptor's)
    /// Inherently unsafe, since there is no guarantee that the original buffer
    /// is still alive
    pub unsafe fn as_bytes(&self) -> &[u8] {
        slice::from_raw_parts(self.ptr, self.size)
    }
}

/// A handle to a resource for writing to it.
///
/// Wraps a `Stream` returned by [`create_output_stream`].
///
/// [`create_output_stream`]: trait.ResourceStorage.html#tycreate_output_stream
#[derive(Clone)]
pub struct ResourceHandle<'a> {
    stream: Option<Rc<RefCell<Stream>>>,
    size_in_bytes: usize,
    storage: &'a ResourceStorage,
    name: String,
    schema: String,
}

impl<'a> ResourceHandle<'a> {
    /// Create a new resource handle from a stream.
    pub fn new(
        storage: &'a ResourceStorage,
        name: String,
        schema: String,
        stream: Rc<RefCell<Stream>>,
    ) -> io::Result<Self> {
        // Reserve space for size in the beginning of the stream, which will be updated
        // later.
        {
            let mut mut_stream = stream.borrow_mut();
            write_size(0u64, mut_stream.deref_mut())?;
        }
        Ok(Self {
            stream: Some(stream),
            size_in_bytes: 0,
            storage,
            name,
            schema,
        })
    }

    /// Returns `true` if the underlying is still open for writing.
    pub fn is_open(&self) -> bool {
        self.stream.is_some()
    }

    /// Writes data to the underlying stream.
    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        let stream = self
            .stream
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "stream closed"))?;

        let res = stream.borrow_mut().write_all(data);
        if res.is_ok() {
            self.size_in_bytes += data.len();
        }
        res
    }

    /// Close the underlying stream and write the header containing the size in
    /// bytes of written data.
    pub fn close(&mut self) -> io::Result<()> {
        {
            let stream = self
                .stream
                .as_ref()
                .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "stream closed"))?;

            let mut mut_stream = stream.borrow_mut();
            write_padding(mut_stream.deref_mut())?;

            // Update size in the beginning of the file
            mut_stream.seek(io::SeekFrom::Start(0u64))?;
            write_size(self.size_in_bytes as u64, mut_stream.deref_mut())?;
        }
        self.stream = None;
        Ok(())
    }
}

impl<'a> Drop for ResourceHandle<'a> {
    fn drop(&mut self) {
        if !thread::panicking() {
            // Only panic in case we are dropping the handle normally
            assert!(!self.is_open(), "Resource not closed");
        }
    }
}

impl<'a> fmt::Debug for ResourceHandle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ResourceHandle {{ is_open: {}, size_in_bytes: {} }}",
            self.is_open(),
            self.size_in_bytes,
        )
    }
}

fn diff(left: &str, right: &str) -> String {
    use diff;
    diff::lines(left, right)
        .into_iter()
        .map(|l| match l {
            diff::Result::Left(l) => format!("-{}", l),
            diff::Result::Both(l, _) => format!(" {}", l),
            diff::Result::Right(r) => format!("+{}", r),
        })
        .collect::<Vec<_>>()
        .join("\n")
}

//
// Write helpers
//

fn write_to_stream(data: &[u8], stream: &mut Stream) -> io::Result<()> {
    write_size(data.len() as u64, stream)?;
    stream.write_all(data)?;
    write_padding(stream)
}

fn write_schema(schema: &str, stream: &mut Stream) -> io::Result<()> {
    stream.write_all(schema.as_bytes())
}

fn write_size(value: SizeType, stream: &mut Stream) -> io::Result<()> {
    const SIZE_OF_SIZE_TYPE: usize = mem::size_of::<SizeType>();
    let mut buffer: [u8; SIZE_OF_SIZE_TYPE] = [0; SIZE_OF_SIZE_TYPE];
    write_bytes!(SizeType; value, &mut buffer, 0, SIZE_OF_SIZE_TYPE * 8);
    stream.write_all(&buffer)
}

fn write_padding(stream: &mut Stream) -> io::Result<()> {
    let zeroes: [u8; PADDING_SIZE] = [0; PADDING_SIZE];
    stream.write_all(&zeroes)
}

#[cfg(test)]
mod test {
    use super::*;
    use memstorage::MemoryResourceStorage;
    use std::io::Cursor;

    #[test]
    #[should_panic]
    fn test_panick_on_leak() {
        let stream = Rc::new(RefCell::new(Cursor::new(Vec::new())));
        let storage = MemoryResourceStorage::new("/root/extvec".into());
        let _h = ResourceHandle::new(&*storage, "".into(), "".into(), stream);
        // leak h without closing it -> should panic
    }

    #[test]
    fn test_not_panick_on_close() -> io::Result<()> {
        let stream = Rc::new(RefCell::new(Cursor::new(Vec::new())));
        let storage = MemoryResourceStorage::new("/root/extvec".into());
        let mut h = ResourceHandle::new(&*storage, "".into(), "".into(), stream)?;
        h.close()
    }
}
