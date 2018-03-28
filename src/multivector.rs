use archive::{Index, IndexMut, StructMut, VariadicStruct};
use memory;
use storage::ResourceHandle;
use vector::ExternalVector;

use std::borrow::BorrowMut;
use std::io;
use std::marker;

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
        let mut idx_mut = Idx::IndexMut::from(self.index.grow()?.as_mut_ptr());
        idx_mut.set_value(self.size_flushed + self.data.len() - memory::PADDING_SIZE);
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

#[cfg(test)]
mod tests {
    use super::super::test_structs::*;
    use arrayview::ArrayView;
    use memstorage::MemoryResourceStorage;
    use multiarrayview::MultiArrayView;
    use storage::ResourceStorage;
    use storage::create_multi_vector;

    #[test]
    fn test_multi_vector() {
        let mut storage = MemoryResourceStorage::new("/root/resources".into());
        {
            let mut mv =
                create_multi_vector::<Idx, Variant>(&mut storage, "multivector", "Some schema")
                    .expect("failed to create ExternalVector");
            let mut item = mv.grow().expect("grow failed");
            {
                let mut a = item.add_a();
                a.set_x(1);
                a.set_y(2);
                assert_eq!(a.x(), 1);
                assert_eq!(a.y(), 2);
            }
            {
                let mut b = item.add_a();
                b.set_x(3);
                b.set_y(4);
                assert_eq!(b.x(), 3);
                assert_eq!(b.y(), 4);
            }
            mv.close().expect("close failed");
        }

        let index_resource = storage
            .read_and_check_schema("multivector_index", "index(Some schema)")
            .expect("read_and_check_schema failed");
        let index: ArrayView<Idx> = ArrayView::new(&index_resource);
        let resource = storage
            .read_and_check_schema("multivector", "Some schema")
            .expect("read_and_check_schema failed");
        let mv: MultiArrayView<Idx, Variant> = MultiArrayView::new(index, &resource);

        assert_eq!(mv.len(), 1);
        let mut item = mv.at(0);
        let a = item.next().unwrap();
        match *a {
            Variant::A(ref a) => {
                assert_eq!(a.x(), 1);
                assert_eq!(a.y(), 2);
            }
        }
        let b = item.next().unwrap();
        match *b {
            Variant::A(ref a) => {
                assert_eq!(a.x(), 3);
                assert_eq!(a.y(), 4);
            }
        }
    }
}
