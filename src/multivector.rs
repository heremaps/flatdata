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

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice;
    use std::str;
    use std::convert;

    use super::super::{ArrayView, MemoryResourceStorage, MultiArrayView};
    use super::super::storage::{create_multi_vector, ResourceStorage};
    use super::super::archive::Struct;

    #[derive(Debug, PartialEq)]
    #[repr(C)]
    struct Idx {
        first_byte: u8,
    }

    impl Struct for Idx {
        const SCHEMA: &'static str = "Index";
        const SIZE_IN_BYTES: usize = 4;
    }

    impl Index for Idx {
        fn value(&self) -> usize {
            read_bytes!(usize, &self.first_byte, 0, 32)
        }

        fn set_value(&mut self, value: usize) {
            let buffer =
                unsafe { slice::from_raw_parts_mut(&mut self.first_byte, Self::SIZE_IN_BYTES) };
            write_bytes!(u32; value as u32, buffer, 0, 32);
        }
    }

    #[derive(Debug, PartialEq)]
    #[repr(C)]
    struct A {
        first_byte: u8,
    }

    impl A {
        pub fn x(&self) -> u32 {
            read_bytes!(u32, &self.first_byte, 0, 16)
        }

        pub fn y(&self) -> u32 {
            read_bytes!(u32, &self.first_byte, 16, 16)
        }

        pub fn set_x(&mut self, value: u32) {
            let buffer =
                unsafe { slice::from_raw_parts_mut(&mut self.first_byte, Self::SIZE_IN_BYTES) };
            write_bytes!(u32; value, buffer, 0, 16);
        }

        pub fn set_y(&mut self, value: u32) {
            let buffer =
                unsafe { slice::from_raw_parts_mut(&mut self.first_byte, Self::SIZE_IN_BYTES) };
            write_bytes!(u32; value, buffer, 16, 16);
        }
    }

    impl Struct for A {
        const SCHEMA: &'static str = "struct A { }";
        const SIZE_IN_BYTES: usize = 4;
    }

    #[derive(Debug, PartialEq)]
    enum Variant<'a> {
        A(&'a A),
    }

    impl<'a> convert::From<(u8, *const u8)> for Variant<'a> {
        fn from((type_index, data): (u8, *const u8)) -> Variant<'a> {
            match type_index {
                0 => unsafe { Variant::A(&*(data as *const A)) },
                _ => panic!("invalid index"),
            }
        }
    }

    impl<'a> VariadicStruct for Variant<'a> {
        type ItemBuilder = VariantBuilder<'a>;
        fn size_in_bytes(&self) -> usize {
            match *self {
                Variant::A(_) => A::SIZE_IN_BYTES,
            }
        }
    }

    struct VariantBuilder<'a> {
        data: &'a mut Vec<u8>,
    }

    impl<'a> VariantBuilder<'a> {
        pub fn add_a(&mut self) -> &mut A {
            let old_len = self.data.len();
            let increment = 1 + A::SIZE_IN_BYTES;
            self.data.resize(old_len + increment, 0);
            self.data[old_len - memory::PADDING_SIZE] = 0;
            unsafe {
                &mut *(&mut self.data[1 + old_len - memory::PADDING_SIZE] as *mut _ as *mut A)
            }
        }
    }

    impl<'a> convert::From<*mut Vec<u8>> for VariantBuilder<'a> {
        fn from(data: *mut Vec<u8>) -> Self {
            Self {
                data: unsafe { &mut *data },
            }
        }
    }

    #[test]
    fn test_multi_vector() {
        let mut storage = MemoryResourceStorage::new("/root/resources".into());
        {
            let mut mv =
                create_multi_vector::<Idx, Variant>(&mut storage, "multivector", "Some schema")
                    .expect("failed to create ExternalVector");
            let mut item = mv.grow().expect("grow failed");
            {
                let a = item.add_a();
                a.set_x(1);
                a.set_y(2);
            }
            {
                let b = item.add_a();
                b.set_x(3);
                b.set_y(4);
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
        match a {
            Variant::A(a) => {
                assert_eq!(a.x(), 1);
                assert_eq!(a.y(), 2);
            }
        }
        let b = item.next().unwrap();
        match b {
            Variant::A(a) => {
                assert_eq!(a.x(), 3);
                assert_eq!(a.y(), 4);
            }
        }
    }
}
