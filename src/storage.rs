use std::ptr;
use std::mem;
use std::slice;
use std::str;
use std::convert;

type SizeType = u64;
const PADDING_SIZE: usize = 8;

/// Hierarchical Resource Storage.
///
/// Manages and returns resources corresponding to their keys. Keys can be slash-separated('/').
/// Manages schema for each resource and checks it on query.
/// Resource storage is expected to provide read-write access to resources
pub trait ResourceStorage {
    fn read<T>(&self, resource_name: &str, schema: &str) -> Option<T>
    where
        T: convert::From<MemoryDescriptor>,
    {
        let data = self.read_and_check_schema(resource_name, schema);
        if !data.is_valid() {
            None
        } else {
            Some(T::from(data))
        }
    }

    // fn write<T>(resource_name: &str, schema:&str, data: T);
    // fn create_external_vector<T>(resources_name: &str, schema: &str) -> ExternalVector<T>;
    // fn create_multi_vector<Index, Args>(resource_name: &str, schema: &str) -> MultiVector<Index, Args>;

    // virtual
    fn read_resource(&self, resource_name: &str) -> MemoryDescriptor;

    //
    // Impl Helper
    //

    fn read_and_check_schema(
        &self,
        resource_name: &str,
        expected_schema: &str,
    ) -> MemoryDescriptor {
        let data = self.read_resource(resource_name);
        if !data.is_valid() {
            return MemoryDescriptor::default();
        }

        let schema = self.read_resource(&format!("{}.schema", resource_name));
        if !schema.is_valid() {
            return MemoryDescriptor::default();
        }

        if data.size_in_bytes() < mem::size_of::<SizeType>() + PADDING_SIZE {
            return MemoryDescriptor::default();
        }

        let size = read_bytes!(SizeType, data.data()) as usize;
        if size + mem::size_of::<SizeType>() + PADDING_SIZE != data.size_in_bytes() {
            return MemoryDescriptor::default();
        }

        // Note: len is size in bytes since we constructing u8 slice.
        let stored_schema_slice: &[u8] =
            unsafe { slice::from_raw_parts(schema.data(), schema.size_in_bytes()) };
        let stored_schema = match str::from_utf8(stored_schema_slice) {
            Ok(s) => s,
            Err(_) => return MemoryDescriptor::default(),
        };
        if stored_schema != expected_schema {
            return MemoryDescriptor::default();
        }

        MemoryDescriptor::new(
            unsafe { data.data().offset(mem::size_of::<SizeType>() as isize) },
            size,
        )
    }
}

/// Describes a chunk of memory
struct MemoryDescriptor {
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

    pub fn is_valid(&self) -> bool {
        self.ptr != ptr::null()
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
