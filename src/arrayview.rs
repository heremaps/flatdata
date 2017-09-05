use std::iter;
use std::marker;

use archive::ArchiveType;
use bytereader::StreamType;
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

    pub fn iter(&self) -> ArrayViewIter<T> {
        ArrayViewIter {
            view: self,
            next_pos: 0,
        }
    }
}

pub struct ArrayViewIter<'a, T: 'a + ArchiveType> {
    view: &'a ArrayView<T>,
    next_pos: usize,
}

impl<'a, T: ArchiveType> iter::Iterator for ArrayViewIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_pos < self.view.size() {
            let element = self.view.index(self.next_pos);
            self.next_pos += 1;
            Some(element)
        } else {
            None
        }
    }
}
