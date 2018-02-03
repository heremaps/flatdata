use std::io;
use std::mem;
use std::ptr;
use std::slice;
use std::str;

use error::ResourceStorageError;

type SizeType = u64;
const PADDING_SIZE: usize = 8;

/// Hierarchical Resource Storage
///
/// Manages and returns resources corresponding to their keys. Keys can be slash-separated('/').
/// Manages schema for each resource and checks it on query.
/// Resource storage is expected to provide read-write access to resources
pub trait ResourceStorage {
    fn read(
        &mut self,
        resource_name: &str,
        schema: &str,
    ) -> Result<MemoryDescriptor, ResourceStorageError> {
        self.read_and_check_schema(resource_name, schema)
    }

    // fn write<T>(resource_name: &str, schema:&str, data: T);
    // fn create_external_vector<T>(resources_name: &str, schema: &str) -> ExternalVector<T>;
    // fn create_multi_vector<Index, Args>(resource_name: &str, schema: &str) -> MultiVector<Index, Args>;

    // virtual
    fn read_resource(&mut self, resource_name: &str) -> Result<MemoryDescriptor, io::Error>;

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
            return Err(ResourceStorageError::WrongSignature);
        }

        Ok(MemoryDescriptor::new(
            unsafe { data.data().offset(mem::size_of::<SizeType>() as isize) },
            size,
        ))
    }
}

/// Describes a chunk of memory
#[derive(Debug)]
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
        MemoryDescriptor {
            ptr: ptr,
            size: size,
        }
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
