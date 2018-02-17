use archive::Struct;
use bytereader::StreamType;
use storage::MemoryDescriptor;

use std::iter;
use std::marker;
use std::fmt;

#[derive(Clone)]
pub struct ArrayView<T> {
    data: StreamType,
    len: usize,
    _phantom: marker::PhantomData<T>,
}

impl<'a, T> ArrayView<T>
where
    T: Struct<'a>,
{
    pub fn new(mem_descr: &MemoryDescriptor) -> Self {
        Self {
            data: mem_descr.data(),
            len: mem_descr.size_in_bytes() / T::SIZE_IN_BYTES,
            _phantom: marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    // Note: It is not possible to use std::ops::Index here, since Index::index has to return a
    // ref, however we need to return a value.
    pub fn at(&self, idx: usize) -> T {
        T::from(unsafe { self.data.offset((idx * T::SIZE_IN_BYTES) as isize) })
    }

    pub fn iter(&'a self) -> ArrayViewIter<T> {
        ArrayViewIter {
            view: self,
            next_pos: 0,
        }
    }
}

impl<T> fmt::Debug for ArrayView<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ArrayView {{ data: {:?}, len: {} }}",
            self.data, self.len
        )
    }
}

pub struct ArrayViewIter<'a, T: 'a + Struct<'a>> {
    view: &'a ArrayView<T>,
    next_pos: usize,
}

impl<'a, T: Struct<'a>> iter::Iterator for ArrayViewIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_pos < self.view.len() {
            let element = self.view.at(self.next_pos);
            self.next_pos += 1;
            Some(element)
        } else {
            None
        }
    }
}

impl<'a, T: Struct<'a>> iter::ExactSizeIterator for ArrayViewIter<'a, T> {
    fn len(&self) -> usize {
        self.view.len()
    }
}
