/// Note: This module is called `structbuf` in contrast to `struct` in the C++ implementation,
/// since Rust does not allow module names to be one of the language keywords.

use archive::Struct;
use memory;

use std::marker;
use std::ops;
use std::slice;

pub struct StructBuf<T> {
    data: Vec<u8>,
    _phantom: marker::PhantomData<T>,
}

impl<T: Struct> StructBuf<T> {
    pub fn new() -> Self {
        Self {
            data: vec![0; T::SIZE_IN_BYTES + memory::PADDING_SIZE],
            _phantom: marker::PhantomData,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.data.as_ptr(), T::SIZE_IN_BYTES) }
    }
}

impl<T: Struct> ops::Deref for StructBuf<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*(&self.data[0] as *const _ as *const T) }
    }
}

impl<T: Struct> ops::DerefMut for StructBuf<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *(&mut self.data[0] as *mut _ as *mut T) }
    }
}

impl<T: Struct> AsRef<[u8]> for StructBuf<T> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}
