use archive::Struct;
use storage::MemoryDescriptor;

use std::iter;
use std::fmt;
use std::slice;
use std::ops::Index;

#[derive(Clone)]
pub struct ArrayView<'a, T: 'a> {
    data: &'a [T],
    len: usize,
}

impl<'a, T> ArrayView<'a, T>
where
    T: Struct,
{
    pub fn new(mem_descr: &MemoryDescriptor) -> Self {
        let data = unsafe {
            let data = &*(mem_descr.data() as *const T);
            slice::from_raw_parts(data, mem_descr.size_in_bytes())
        };
        Self {
            data,
            len: mem_descr.size_in_bytes() / T::SIZE_IN_BYTES,
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
        unsafe { slice::from_raw_parts(self.data.as_ptr() as *const u8, self.data.len()) }
    }
}

impl<'a, T> Index<usize> for ArrayView<'a, T>
where
    T: Struct,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let pos = index * T::SIZE_IN_BYTES;
        assert!(pos + T::SIZE_IN_BYTES <= self.data.len());
        self.data.index(pos)
    }
}

impl<'a, T> fmt::Debug for ArrayView<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ArrayView {{ data: {:?}, len: {} }}",
            &self.data[0] as *const T, self.len
        )
    }
}

pub struct ArrayViewIter<'a, T: 'a + Struct> {
    view: &'a ArrayView<'a, T>,
    next_pos: usize,
}

impl<'a, T: Struct> iter::Iterator for ArrayViewIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_pos < self.view.len() {
            let element = &self.view[self.next_pos];
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
