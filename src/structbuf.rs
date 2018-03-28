/// Note: This module is called `structbuf` in contrast to `struct` in the C++ implementation,
/// since Rust does not allow module names to be one of the language keywords.
use archive::Struct;
use memory;

use std::ops;
use std::slice;

pub struct StructBuf<T: Struct> {
    buffer: Vec<u8>,
    data: T::Mut,
}

impl<T: Struct> StructBuf<T> {
    pub fn new() -> Self {
        let mut buffer = vec![0; T::SIZE_IN_BYTES + memory::PADDING_SIZE];
        let ptr = buffer.as_mut_ptr();
        Self {
            buffer,
            data: T::Mut::from(ptr),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.buffer.as_ptr(), T::SIZE_IN_BYTES) }
    }
}

impl<T: Struct> ops::Deref for StructBuf<T> {
    type Target = T::Mut;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Struct> ops::DerefMut for StructBuf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl<T: Struct> AsRef<[u8]> for StructBuf<T> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
