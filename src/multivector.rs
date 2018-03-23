use archive::{Index, VariadicStruct};
use memory;
use storage::ResourceHandle;
use vector::ExternalVector;

use std::marker;
use std::io;
use std::borrow::BorrowMut;

pub struct MultiVector<Idx, Ts> {
    index: ExternalVector<Idx>,
    data: Vec<u8>,
    data_handle: ResourceHandle,
    size_flushed: usize,
    _phantom: marker::PhantomData<Ts>,
}

impl<Idx: Index, Ts: VariadicStruct> MultiVector<Idx, Ts> {
    pub fn new(index: ExternalVector<Idx>, data_handle: ResourceHandle) -> Self {
        Self {
            index,
            data: vec![0; memory::PADDING_SIZE],
            data_handle,
            size_flushed: 0,
            _phantom: marker::PhantomData,
        }
    }

    pub fn grow(&mut self) -> io::Result<Ts::ItemBuilder> {
        if self.data.len() > 1024 * 1024 * 32 {
            self.flush()?;
        }
        self.add_to_index()?;
        Ok(Ts::ItemBuilder::from(&mut self.data))
    }

    fn flush(&mut self) -> io::Result<()> {
        self.data_handle
            .borrow_mut()
            .write(&self.data[..self.data.len() - memory::PADDING_SIZE])?;
        self.size_flushed += self.data.len() - memory::PADDING_SIZE;
        self.data.clear();
        self.data.resize(memory::PADDING_SIZE, 0);
        Ok(())
    }

    fn add_to_index(&mut self) -> io::Result<()> {
        self.index
            .grow()?
            .set_value(self.size_flushed + self.data.len() - memory::PADDING_SIZE);
        Ok(())
    }

    pub fn close(&mut self) -> io::Result<()> {
        self.add_to_index()?; // sentinel for last item
        self.index.close()?;
        self.flush()?;
        self.data_handle.borrow_mut().close()
    }
}

impl<Idx, Ts> Drop for MultiVector<Idx, Ts> {
    fn drop(&mut self) {
        debug_assert!(!self.data_handle.is_open(), "MultiVector not closed")
    }
}
