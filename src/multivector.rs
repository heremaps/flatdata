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
    use super::*;
    use std::convert;
    use std::mem;
    use std::slice;
    use std::str;

    use super::super::{ArrayView, MemoryResourceStorage, MultiArrayView};
    use super::super::archive::{Struct, StructMut};
    use super::super::handle::HandleMut;
    use super::super::storage::{create_multi_vector, ResourceStorage};

    #[derive(Clone, Debug, PartialEq)]
    struct Idx {
        data: *const u8,
    }

    impl Struct for Idx {
        const SCHEMA: &'static str = "Index";
        const SIZE_IN_BYTES: usize = 4;
        type Mut = IdxMut;
        fn as_ptr(&self) -> *const u8 {
            self.data
        }
    }

    impl convert::From<*const u8> for Idx {
        fn from(data: *const u8) -> Self {
            Self { data }
        }
    }

    impl Index for Idx {
        type IndexMut = IdxMut;
        fn value(&self) -> usize {
            read_bytes!(usize, self.data, 0, 32)
        }
    }

    #[derive(Debug)]
    struct IdxMut {
        data: *mut u8,
    }

    impl convert::From<*mut u8> for IdxMut {
        fn from(data: *mut u8) -> Self {
            Self { data }
        }
    }

    impl StructMut for IdxMut {
        type Const = Idx;
        fn as_mut_ptr(&mut self) -> *mut u8 {
            self.data
        }
    }

    impl AsRef<Idx> for IdxMut {
        fn as_ref(&self) -> &Idx {
            unsafe { mem::transmute(self) }
        }
    }

    impl IndexMut for IdxMut {
        fn set_value(&mut self, value: usize) {
            let buffer =
                unsafe { slice::from_raw_parts_mut(self.data, Self::Const::SIZE_IN_BYTES) };
            write_bytes!(u32; value as u32, buffer, 0, 32);
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    struct A {
        data: *const u8,
    }

    impl A {
        pub fn x(&self) -> u32 {
            read_bytes!(u32, self.data, 0, 16)
        }

        pub fn y(&self) -> u32 {
            read_bytes!(u32, self.data, 16, 16)
        }
    }

    impl convert::From<*const u8> for A {
        fn from(data: *const u8) -> Self {
            Self { data }
        }
    }

    impl Struct for A {
        const SCHEMA: &'static str = "struct A { }";
        const SIZE_IN_BYTES: usize = 4;
        type Mut = AMut;
        fn as_ptr(&self) -> *const u8 {
            self.data
        }
    }

    #[derive(Debug)]
    struct AMut {
        data: *mut u8,
    }

    impl AMut {
        pub fn set_x(&mut self, value: u32) {
            let buffer = unsafe {
                slice::from_raw_parts_mut(self.data, <Self as StructMut>::Const::SIZE_IN_BYTES)
            };
            write_bytes!(u32; value, buffer, 0, 16);
        }

        pub fn set_y(&mut self, value: u32) {
            let buffer = unsafe {
                slice::from_raw_parts_mut(self.data, <Self as StructMut>::Const::SIZE_IN_BYTES)
            };
            write_bytes!(u32; value, buffer, 16, 16);
        }
    }

    impl convert::From<*mut u8> for AMut {
        fn from(data: *mut u8) -> Self {
            Self { data }
        }
    }

    impl AsRef<A> for AMut {
        fn as_ref(&self) -> &A {
            unsafe { mem::transmute(self) }
        }
    }

    impl StructMut for AMut {
        type Const = A;
        fn as_mut_ptr(&mut self) -> *mut u8 {
            self.data
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    enum Variant {
        A(A),
    }

    impl convert::From<(u8, *const u8)> for Variant {
        fn from((type_index, data): (u8, *const u8)) -> Variant {
            match type_index {
                0 => Variant::A(A::from(data)),
                _ => panic!("invalid index"),
            }
        }
    }

    impl VariadicStruct for Variant {
        type ItemBuilder = VariantBuilder;
        fn size_in_bytes(&self) -> usize {
            match *self {
                Variant::A(_) => A::SIZE_IN_BYTES,
            }
        }
    }

    struct VariantBuilder {
        data: *mut Vec<u8>,
    }

    impl VariantBuilder {
        pub fn add_a(&mut self) -> HandleMut<<A as Struct>::Mut> {
            let data = unsafe { &mut *self.data };
            let old_len = data.len();
            let increment = 1 + A::SIZE_IN_BYTES;
            data.resize(old_len + increment, 0);
            data[old_len - memory::PADDING_SIZE] = 0;
            HandleMut::new(<A as Struct>::Mut::from(
                &mut data[1 + old_len - memory::PADDING_SIZE] as *mut _,
            ))
        }
    }

    impl convert::From<*mut Vec<u8>> for VariantBuilder {
        fn from(data: *mut Vec<u8>) -> Self {
            Self { data }
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
                let mut a = item.add_a();
                a.set_x(1);
                a.set_y(2);
            }
            {
                let mut b = item.add_a();
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
