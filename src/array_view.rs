

use archive::ArchiveType;
use byte_reader::StreamType;
use std::marker;
use std::mem;
use storage::MemoryDescriptor;

#[derive(Debug)]
pub struct ArrayView<T: ArchiveType> {
    data: StreamType,
    size: usize,
    _phantom: marker::PhantomData<T>,
}

impl<T> ArrayView<T>
where
    T: ArchiveType,
{
    pub fn new(mem_descr: MemoryDescriptor) -> Self {
        ArrayView {
            data: mem_descr.data(),
            size: mem_descr.size_in_bytes() / T::SIZE_IN_BYTES,
            _phantom: marker::PhantomData,
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn index(&self, idx: usize) -> T {
        T::from(unsafe { self.data.offset((idx * T::SIZE_IN_BYTES) as isize) })
    }
}
