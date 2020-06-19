use crate::{
    error::ResourceStorageError,
    memory::{SizeType, PADDING_SIZE},
    multivector::MultiVector,
    structs::{Struct, VariadicRefFactory},
    vector::ExternalVector,
};

use std::{
    fmt,
    io::{self, Seek, Write},
    mem, str,
    sync::Arc,
};

use diff;

/// A handle to a resource storage used by archives
pub type StorageHandle = Arc<dyn ResourceStorage + std::marker::Sync + std::marker::Send>;

pub trait Stream: Write + Seek {}
impl<T: Write + Seek> Stream for T {}

/// Hierarchical Resource Storage
///
/// Manages and returns resources corresponding to their keys. Keys can be
/// slash-separated('/'). Manages schema for each resource and checks it on
/// query. Resource storage is expected to provide read-write access to
/// resources.
pub trait ResourceStorage: std::fmt::Debug {
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
        let mut stream = self.create_output_stream(resource_name)?;
        write_to_stream(data, &mut stream)?;
        // write schema
        let schema_name = format!("{}.schema", resource_name);
        let mut stream = self.create_output_stream(&schema_name)?;
        write_schema(schema, &mut stream)
    }

    //
    // Virtual
    //

    /// Creates a resource storage at a given subdirectory.
    fn subdir(&self, dir: &str) -> StorageHandle;

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
    fn create_output_stream(&self, resource_name: &str) -> io::Result<Box<dyn Stream>>;

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

        let size = flatdata_read_bytes!(SizeType, data.as_ptr()) as usize;
        if size + mem::size_of::<SizeType>() + PADDING_SIZE != data.len() {
            return Err(ResourceStorageError::UnexpectedDataSize);
        }

        let stored_schema_slice: &[u8] = schema;
        let stored_schema =
            str::from_utf8(stored_schema_slice).map_err(ResourceStorageError::Utf8Error)?;
        if stored_schema != expected_schema {
            return Err(ResourceStorageError::WrongSignature {
                resource_name: resource_name.into(),
                diff: compute_diff(stored_schema, expected_schema),
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
#[doc(hidden)]
pub fn create_external_vector<'a, T>(
    storage: &'a dyn ResourceStorage,
    resource_name: &str,
    schema: &str,
) -> io::Result<ExternalVector<'a, T>>
where
    T: Struct,
{
    // write schema
    let schema_name = format!("{}.schema", resource_name);
    let mut stream = storage.create_output_stream(&schema_name)?;
    stream.write_all(schema.as_bytes())?;

    // create external vector
    let data_writer = storage.create_output_stream(resource_name)?;
    let handle =
        ResourceHandle::try_new(storage, resource_name.into(), schema.into(), data_writer)?;
    Ok(ExternalVector::new(handle))
}

/// Helper for creating a multivector in the given resource storage.
///
/// Creates a new resource with given name and schema in storage, and returns
/// an [`MultiVector`] using this resource for writing and flushing data to
/// storage.
#[doc(hidden)]
pub fn create_multi_vector<'a, Ts>(
    storage: &'a dyn ResourceStorage,
    resource_name: &str,
    schema: &str,
) -> io::Result<MultiVector<'a, Ts>>
where
    Ts: VariadicRefFactory,
{
    // create index
    let index_name = format!("{}_index", resource_name);
    let index_schema = format!("index({})", schema);
    let index = create_external_vector(storage, &index_name, &index_schema)?;

    // write schema
    let schema_name = format!("{}.schema", resource_name);
    let mut stream = storage.create_output_stream(&schema_name)?;
    stream.write_all(schema.as_bytes())?;

    // create multi vector
    let data_writer = storage.create_output_stream(resource_name)?;
    let handle =
        ResourceHandle::try_new(storage, resource_name.into(), schema.into(), data_writer)?;
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
#[doc(hidden)]
pub fn create_archive(
    name: &str,
    schema: &str,
    storage: &StorageHandle,
) -> Result<(), ResourceStorageError> {
    let signature_name = format!("{}.archive", name);
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
            .write(&signature_name, schema, &[])
            .map_err(|e| ResourceStorageError::from_io_error(e, signature_name))?;
    }
    Ok(())
}

/// A handle to a resource for writing to it.
///
/// Wraps a `Stream` returned by [`create_output_stream`].
///
/// [`create_output_stream`]: trait.ResourceStorage.html#tycreate_output_stream
pub struct ResourceHandle<'a> {
    stream: Box<dyn Stream>,
    size_in_bytes: usize,
    storage: &'a dyn ResourceStorage,
    name: String,
    schema: String,
    finalized: bool,
}

impl<'a> ResourceHandle<'a> {
    /// Try to create a new resource handle from a stream.
    ///
    /// ## Errors
    ///
    /// Resource storage will try to reserve space in the beginning of the
    /// stream for the size of the resource, will may result in an `io::Error`.
    pub(crate) fn try_new(
        storage: &'a dyn ResourceStorage,
        name: String,
        schema: String,
        mut stream: Box<dyn Stream>,
    ) -> io::Result<Self> {
        // Reserve space for size in the beginning of the stream, which will be updated
        // later.
        {
            write_size(0u64, &mut stream)?;
        }
        Ok(Self {
            stream,
            size_in_bytes: 0,
            storage,
            name,
            schema,
            finalized: false,
        })
    }

    /// Writes data to the underlying stream.
    pub(crate) fn write(&mut self, data: &[u8]) -> io::Result<()> {
        let res = self.stream.write_all(data);
        if res.is_ok() {
            self.size_in_bytes += data.len();
        }
        res
    }

    /// Close the underlying stream and write the header containing the size in
    /// bytes of written data.
    pub(crate) fn close(mut self) -> Result<&'a [u8], ResourceStorageError> {
        self.finalize()?;

        // return underlying memory descriptor to the written data
        self.storage.read(&self.name, &self.schema)
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    fn finalize(&mut self) -> Result<(), ResourceStorageError> {
        assert!(!self.finalized);
        self.finalized = true;

        let resource_name = self.name.clone();
        let into_storage_error = |e| ResourceStorageError::from_io_error(e, resource_name.clone());

        write_padding(&mut self.stream).map_err(into_storage_error)?;

        // Update size in the beginning of the file
        self.stream
            .seek(io::SeekFrom::Start(0u64))
            .map_err(into_storage_error)?;
        write_size(self.size_in_bytes as u64, &mut self.stream).map_err(into_storage_error)?;

        Ok(())
    }
}

impl<'a> fmt::Debug for ResourceHandle<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ResourceHandle {{ name: {}, size_in_bytes: {} }}",
            self.name, self.size_in_bytes,
        )
    }
}

fn compute_diff(left: &str, right: &str) -> String {
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

fn write_to_stream(data: &[u8], stream: &mut dyn Stream) -> io::Result<()> {
    write_size(data.len() as u64, stream)?;
    stream.write_all(data)?;
    write_padding(stream)
}

fn write_schema(schema: &str, stream: &mut dyn Stream) -> io::Result<()> {
    stream.write_all(schema.as_bytes())
}

fn write_size(value: SizeType, stream: &mut dyn Stream) -> io::Result<()> {
    const SIZE_OF_SIZE_TYPE: usize = mem::size_of::<SizeType>();
    let mut buffer: [u8; SIZE_OF_SIZE_TYPE] = [0; SIZE_OF_SIZE_TYPE];
    flatdata_write_bytes!(SizeType; value, &mut buffer, 0, SIZE_OF_SIZE_TYPE * 8);
    stream.write_all(&buffer)
}

fn write_padding(stream: &mut dyn Stream) -> io::Result<()> {
    let zeroes: [u8; PADDING_SIZE] = [0; PADDING_SIZE];
    stream.write_all(&zeroes)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::memstorage::MemoryResourceStorage;

    #[test]
    fn test_not_panic_on_close() -> Result<(), ResourceStorageError> {
        let storage = MemoryResourceStorage::new("/root/extvec");

        let mut stream = storage
            .create_output_stream("/root/extvec/blubb.schema")
            .unwrap();
        stream.write_all("myschema".as_bytes()).unwrap();

        let stream = storage.create_output_stream("/root/extvec/blubb").unwrap();
        let h = ResourceHandle::try_new(
            &*storage,
            "/root/extvec/blubb".into(),
            "myschema".into(),
            stream,
        )
        .unwrap();
        h.close().map(|_| ())
    }
}
