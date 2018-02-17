use archive::{Index, VariadicStruct};
use arrayview::ArrayView;
use bytereader::StreamType;
use storage::MemoryDescriptor;

use std::fmt;
use std::iter;
use std::marker;

/// Ts is a variadic type describing the types of an element in `MultiArrayView`.
#[derive(Clone)]
pub struct MultiArrayView<Idx, Ts> {
    index: ArrayView<Idx>,
    data: StreamType,
    _phantom: marker::PhantomData<Ts>,
}

impl<Idx, Ts> MultiArrayView<Idx, Ts>
where
    Idx: Index,
    Ts: VariadicStruct,
{
    pub fn new(index: ArrayView<Idx>, data_mem_descr: &MemoryDescriptor) -> Self {
        Self {
            index: index,
            data: data_mem_descr.data(),
            _phantom: marker::PhantomData,
        }
    }

    pub fn at(&self, idx: usize) -> MultiArrayViewItemIter<Ts> {
        let start = self.index.at(idx).value();
        let end = self.index.at(idx + 1).value();
        MultiArrayViewItemIter {
            data: unsafe { self.data.offset(start as isize) },
            end: unsafe { self.data.offset(end as isize) },
            _phantom: marker::PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        // last index element is a sentinel
        self.index.len() - 1
    }

    pub fn iter(&self) -> MultiArrayViewIter<Idx, Ts> {
        MultiArrayViewIter {
            view: self,
            next_pos: 0,
        }
    }
}

pub struct MultiArrayViewItemIter<'a, Ts: 'a> {
    data: StreamType,
    end: StreamType,
    _phantom: marker::PhantomData<&'a Ts>,
}

impl<'a, Ts: 'a + VariadicStruct> iter::Iterator for MultiArrayViewItemIter<'a, Ts> {
    type Item = Ts;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data < self.end {
            let type_index;
            unsafe {
                type_index = *self.data;
                self.data = self.data.offset(1);
            }
            let res = Ts::from((type_index, self.data));
            self.data = unsafe { self.data.offset(res.size_in_bytes() as isize) };
            Some(res)
        } else {
            None
        }
    }
}

impl<Idx, Ts> fmt::Debug for MultiArrayView<Idx, Ts>
where
    Idx: Index,
    Ts: VariadicStruct,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MultiArrayView {{ data: {:?}, len: {} }}",
            self.data,
            self.len()
        )
    }
}

pub struct MultiArrayViewIter<'a, Idx: 'a + Index, Ts: 'a + VariadicStruct> {
    view: &'a MultiArrayView<Idx, Ts>,
    next_pos: usize,
}

impl<'a, Idx: 'a + Index, Ts: 'a + VariadicStruct> iter::Iterator
    for MultiArrayViewIter<'a, Idx, Ts>
{
    type Item = MultiArrayViewItemIter<'a, Ts>;
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

impl<'a, Idx: 'a + Index, Ts: 'a + VariadicStruct> iter::ExactSizeIterator
    for MultiArrayViewIter<'a, Idx, Ts>
{
    fn len(&self) -> usize {
        self.view.len()
    }
}
