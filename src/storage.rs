use std::io::{self, Seek, Write};
use std::mem;
use std::ptr;
use std::slice;
use std::str;
use std::rc::Rc;
use std::cell::RefCell;
use std::ops::DerefMut;

use error::ResourceStorageError;
use memory::{SizeType, PADDING_SIZE};
use vector::ExternalVector;
use archive::Struct;

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

pub trait Stream: Write + Seek {}

/// Hierarchical Resource Storage
///
/// Manages and returns resources corresponding to their keys. Keys can be slash-separated('/').
/// Manages schema for each resource and checks it on query.
/// Resource storage is expected to provide read-write access to resources
pub trait ResourceStorage {
    // read interface

    fn read(
        &mut self,
        resource_name: &str,
        schema: &str,
    ) -> Result<MemoryDescriptor, ResourceStorageError> {
        self.read_and_check_schema(resource_name, schema)
    }

    // fn write<T: convert::Into<MemoryDescriptor>>(
    //     &mut self,
    //     resource_name: &str,
    //     schema: &str,
    //     data: T,
    // ) {
    //     self.write_to_stream(resource_name, schema, data.into())
    // }

    // fn create_multi_vector<Index, Args>(resource_name: &str, schema: &str) -> MultiVector<Index, Args>;

    // virtual
    fn read_resource(&mut self, resource_name: &str) -> Result<MemoryDescriptor, io::Error>;
    fn create_output_stream(
        &mut self,
        resource_name: &str,
    ) -> Result<Rc<RefCell<Stream>>, io::Error>;

    //
    // Impl Helper
    //

    fn read_and_check_schema(
        &mut self,
        resource_name: &str,
        expected_schema: &str,
    ) -> Result<MemoryDescriptor, ResourceStorageError> {
        let data = self.read_resource(resource_name)
            .map_err(|e| ResourceStorageError::from_io_error(e, resource_name.into()))?;

        let schema_name = format!("{}.schema", resource_name);
        let schema = self.read_resource(&schema_name)
            .map_err(|e| ResourceStorageError::from_io_error(e, resource_name.into()))?;

        if data.size_in_bytes() < mem::size_of::<SizeType>() + PADDING_SIZE {
            return Err(ResourceStorageError::UnexpectedDataSize);
        }

        let size = read_bytes!(SizeType, data.data()) as usize;
        if size + mem::size_of::<SizeType>() + PADDING_SIZE != data.size_in_bytes() {
            return Err(ResourceStorageError::UnexpectedDataSize);
        }

        // Note: len is size in bytes since we are constructing u8 slice.
        let stored_schema_slice: &[u8] =
            unsafe { slice::from_raw_parts(schema.data(), schema.size_in_bytes()) };
        let stored_schema =
            str::from_utf8(stored_schema_slice).map_err(ResourceStorageError::Utf8Error)?;
        if stored_schema != expected_schema {
            return Err(ResourceStorageError::WrongSignature {
                resource_name: resource_name.into(),
                diff: diff(stored_schema, expected_schema),
            });
        }

        Ok(MemoryDescriptor::new(
            unsafe { data.data().offset(mem::size_of::<SizeType>() as isize) },
            size,
        ))
    }

    // fn write_to_stream<T>(&mut self, resource_name: &str, schema: &str, data: T) {
    //     let stream = self.create_output_stream;
    //     stream.write();
    // }
}

pub fn create_external_vector<T: Struct>(
    storage: &mut ResourceStorage,
    resource_name: &str,
    schema: &str,
) -> Result<ExternalVector<T>, ResourceStorageError> {
    // write schema
    let schema_name = format!("{}.schema", resource_name);
    let stream = storage
        .create_output_stream(&schema_name)
        .map_err(|e| ResourceStorageError::from_io_error(e, schema_name))?;
    stream
        .borrow_mut()
        .write_all(schema.as_bytes())
        .map_err(|e| ResourceStorageError::from_io_error(e, resource_name.into()))?;

    // create external vector
    let data_writer = storage
        .create_output_stream(resource_name)
        .map_err(|e| ResourceStorageError::from_io_error(e, resource_name.into()))?;
    let handle = ResourceHandle::new(data_writer)
        .map_err(|e| ResourceStorageError::from_io_error(e, resource_name.into()))?;
    Ok(ExternalVector::new(handle))
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

impl MemoryDescriptor {
    pub fn new(ptr: *const u8, size: usize) -> MemoryDescriptor {
        MemoryDescriptor { ptr, size }
    }

    pub fn describe(&self) -> String {
        format!("Raw data of size {}", self.size)
    }

    pub fn data(&self) -> *const u8 {
        self.ptr
    }

    pub fn size_in_bytes(&self) -> usize {
        self.size
    }
}

#[derive(Clone)]
pub struct ResourceHandle {
    stream: Option<Rc<RefCell<Stream>>>,
    size_in_bytes: usize,
}

impl ResourceHandle {
    pub fn new(stream: Rc<RefCell<Stream>>) -> io::Result<Self> {
        // Reserve space for size in the beginning of the stream, which will be updated later.
        {
            let mut mut_stream = stream.borrow_mut();
            write_size(0u64, mut_stream.deref_mut())?;
        }
        Ok(Self {
            stream: Some(stream),
            size_in_bytes: 0,
        })
    }

    pub fn is_open(&self) -> bool {
        self.stream.is_some()
    }

    pub fn write(&mut self, data: &[u8]) -> io::Result<()> {
        let stream = self.stream
            .as_ref()
            .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "stream closed"))?;

        let res = stream.borrow_mut().write_all(data);
        if res.is_ok() {
            self.size_in_bytes += data.len();
        }
        res
    }

    pub fn close(&mut self) -> io::Result<()> {
        {
            let stream = self.stream
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
