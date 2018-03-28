use archive::Struct;
use handle::Handle;
use storage::MemoryDescriptor;

use std::fmt;
use std::iter;
use std::marker;
use std::slice;

#[derive(Clone)]
pub struct ArrayView<'a, T: 'a> {
    data: &'a [u8],
    len: usize,
    _phantom: marker::PhantomData<T>,
}

impl<'a, T: Struct> ArrayView<'a, T> {
    pub fn new(mem_descr: &MemoryDescriptor) -> Self {
        Self {
            data: unsafe { slice::from_raw_parts(mem_descr.data(), mem_descr.size_in_bytes()) },
            len: mem_descr.size_in_bytes() / T::SIZE_IN_BYTES,
            _phantom: marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&'a self) -> ArrayViewIter<T> {
        ArrayViewIter {
            view: self,
            next_pos: 0,
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.data
    }

    pub fn at(&self, index: usize) -> Handle<T> {
        let index = index * T::SIZE_IN_BYTES;
        assert!(index + T::SIZE_IN_BYTES <= self.data.len());
        Handle::new(T::from(&self.data[index]))
    }
}

impl<'a, T> fmt::Debug for ArrayView<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ArrayView {{ data: {:?}, len: {} }}",
            self.data, self.len
        )
    }
}

impl<'a, T: Struct> AsRef<[u8]> for ArrayView<'a, T> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

pub struct ArrayViewIter<'a, T: 'a + Struct> {
    view: &'a ArrayView<'a, T>,
    next_pos: usize,
}

impl<'a, T: Struct> iter::Iterator for ArrayViewIter<'a, T> {
    type Item = Handle<'a, T>;
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

impl<'a, T: Struct> iter::ExactSizeIterator for ArrayViewIter<'a, T> {
    fn len(&self) -> usize {
        self.view.len()
    }
}
