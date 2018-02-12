use archive::{IndexType, VariadicArchiveType};
use arrayview::ArrayView;
use bytereader::StreamType;
use storage::MemoryDescriptor;

use std::fmt;
use std::iter;
use std::marker;

/// Ts is a type describing the variadic types of the MultiArrayView.
pub struct MultiArrayView<Idx, Ts> {
    index: ArrayView<Idx>,
    data: StreamType,
    _phantom: marker::PhantomData<Ts>,
}

impl<Idx, Ts> MultiArrayView<Idx, Ts>
where
    Idx: IndexType,
    Ts: VariadicArchiveType,
{
    pub fn new(index: ArrayView<Idx>, data_mem_descr: &MemoryDescriptor) -> Self {
        Self {
            index: index,
            data: data_mem_descr.data(),
            _phantom: marker::PhantomData,
        }
    }

    pub fn index(&self, idx: usize) -> MultiArrayViewItemIter<Ts> {
        let start = self.index.index(idx).value();
        let end = self.index.index(idx + 1).value();
        MultiArrayViewItemIter {
            data: unsafe { self.data.offset(start as isize) },
            end: unsafe { self.data.offset(end as isize) },
            _phantom: marker::PhantomData,
        }
    }

    pub fn size(&self) -> usize {
        // last index element is a sentinel
        self.index.size() - 1
    }
}

pub struct MultiArrayViewItemIter<Ts> {
    data: StreamType,
    end: StreamType,
    _phantom: marker::PhantomData<Ts>,
}

impl<Ts: VariadicArchiveType> iter::Iterator for MultiArrayViewItemIter<Ts> {
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
    Idx: IndexType,
    Ts: VariadicArchiveType,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "MultiArrayView{{ data: {:?}, size: {} }}",
            self.data,
            self.size()
        )
    }
}
