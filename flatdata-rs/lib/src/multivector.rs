use crate::{
    error::ResourceStorageError,
    multiarrayview::MultiArrayView,
    storage::ResourceHandle,
    structs::{IndexStruct, VariadicRefFactory, VariadicStruct},
    vector::ExternalVector,
};

use std::{borrow::BorrowMut, fmt, io, marker};

/// A container for writing an indexed sequence of heterogeneous data items.
///
/// The concept of a multivector is used for storing and reading heterogeneous
/// flatdata structs in/from the same container. The data is indexed by
/// integers. Each index refers to a bucket which may contain a variable number
/// of items of different types unified in the same variant enum `Ts`.
/// Such bucket may also be empty, which allows to represent sparse data in a
/// multivector. For those who are familiar with C++'s `std::multimap` data
/// structure, a multivector can be considered as a `std::multimap` mapping
/// integers to sequences of variable length.
///
/// A `MultiVector` corresponds rather to [`ExternalVector`] than to
/// [`Vector`], in the sense that the items are flushed to storage whenever the
/// internal buffer is full. In particular, it is only possible to modify the
/// last bucket. There is no access to the buckets previously stored.
///
/// For accessing and reading the data stored by in multivector, cf.
/// [`MultiArrayView`].
///
/// A multivector *must* be closed, after the last element was written to it.
/// After closing, it can not be used anymore.
///
/// Internally data is stored like this:
///
/// * `Index`: `Vector<Idx>` - encodes start/end byte in `Data` array for each
/// element `i`. * `Data`: `Vec<u8>` - sequence of serialized (`Tag`,
/// `ItemData`) tuples, where `Tag` encodes the the variant type, and
/// `ItemData` contains the underlying variant data. `Tag` has size of 1 byte,
/// `ItemData` is of size `Ts::Type::SIZE_IN_BYTES`.
///
/// # Examples
/// ```flatdata
/// struct A {
///     x : u32 : 16;
///     y : u32 : 16;
/// }
///
/// struct B {
///     id : u32 : 16;
/// }
///
/// archive Z {
///    ab : multivector<16, A, B>;
/// }
/// ```
///
/// ```rust
/// use flatdata::MemoryResourceStorage;
/// use flatdata::test::{A, B, AbRef, Z, ZBuilder};
///
/// // create multivector and serialize some data
/// let mut storage = MemoryResourceStorage::new("/root/multivec");
/// let mut builder = ZBuilder::new(storage.clone()).expect("Fail to create builder");
/// let mut mv = builder.start_ab().expect("failed to create MultiVector");
/// let mut item = mv.grow().expect("grow failed");
/// let mut a = item.add_a();
/// a.set_x(1);
/// a.set_y(2);
///
/// let mut b = item.add_b();
/// b.set_id(42);
/// mv.close().expect("close failed");
///
/// // open multivector and read the data
/// let archive = Z::open(storage).expect("open failed");
/// let mv = archive.ab();
///
/// assert_eq!(mv.len(), 1);
///
/// // Items are iterators over `AbRef` enums with variants `A` and `B`.
/// // The name of the item type is the name of the multivector in the archive with the `Ref`
/// // suffix.
/// let mut item = mv.at(0);
/// match item.next().unwrap() {
///     AbRef::A(a) => assert_eq!((a.x(), a.y()), (1, 2)),
///     _ => assert!(false),
/// }
/// match item.next().unwrap() {
///     AbRef::B(b) => assert_eq!(b.id(), 42),
///     _ => assert!(false),
/// }
/// ```
///
/// [`ExternalVector`]: struct.ExternalVector.html
/// [`Vector`]: struct.Vector.html
/// [`MultiArrayView`]: struct.MultiArrayView.html
pub struct MultiVector<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    index: ExternalVector<'a, Ts::Index>,
    data: Vec<u8>,
    data_handle: ResourceHandle<'a>,
    size_flushed: usize,
    _phantom: marker::PhantomData<Ts>,
}

impl<'a, Ts> MultiVector<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    /// Creates an empty multivector.
    pub fn new(index: ExternalVector<'a, Ts::Index>, data_handle: ResourceHandle<'a>) -> Self {
        Self {
            index,
            data: Vec::new(),
            data_handle,
            size_flushed: 0,
            _phantom: marker::PhantomData,
        }
    }

    /// Appends a new item to the end of this multivector and returns a builder
    /// for it.
    ///
    /// The builder is used for storing different variants of `Ts` in the newly
    /// created item.
    ///
    /// Calling this method may flush data to storage (cf. [`flush`]), which
    /// may fail due to different IO reasons.
    ///
    /// [`flush`]: #method.flush
    pub fn grow(&mut self) -> io::Result<<Ts as VariadicStruct>::ItemMut> {
        if self.data.len() > 1024 * 1024 * 32 {
            self.flush()?;
        }
        self.add_to_index()?;
        Ok(<Ts as VariadicStruct>::create_mut(&mut self.data))
    }

    /// Flushes the not yet flushed content in this multivector to storage.
    ///
    /// Only data is flushed.
    fn flush(&mut self) -> io::Result<()> {
        self.data_handle.borrow_mut().write(&self.data)?;
        self.size_flushed += self.data.len();
        self.data.clear();
        Ok(())
    }

    fn add_to_index(&mut self) -> io::Result<()> {
        let idx_mut = self.index.grow()?;
        Ts::Index::set_index(idx_mut, self.size_flushed + self.data.len());
        Ok(())
    }

    /// Flushes the remaining not yet flushed elements in this multivector and
    /// finalizes the data inside the storage.
    ///
    /// A multivector *must* be closed
    pub fn close(mut self) -> Result<MultiArrayView<'a, Ts>, ResourceStorageError> {
        let name: String = self.data_handle.name().into();
        let into_storage_error = |e| ResourceStorageError::from_io_error(e, name.clone());
        self.add_to_index().map_err(into_storage_error)?; // sentinel for last item
        self.flush().map_err(into_storage_error)?;
        let index_view = self.index.close()?;
        let data = self.data_handle.close()?;
        Ok(MultiArrayView::new(index_view, data))
    }
}

impl<'a, Ts> fmt::Debug for MultiVector<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MultiVector {{ len: {} }}", self.index.len())
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{
        memstorage::MemoryResourceStorage,
        multiarrayview::MultiArrayView,
        storage::{create_multi_vector, ResourceStorage},
        test::{Ab, AbRef, _builtin::multivector::IndexType16},
    };

    #[test]
    fn test_multi_vector() {
        let storage = MemoryResourceStorage::new("/root/resources");
        {
            let mut mv = create_multi_vector::<Ab>(&*storage, "multivector", "Some schema")
                .expect("failed to create MultiVector");
            {
                let mut item = mv.grow().expect("grow failed");
                {
                    let a = item.add_a();
                    a.set_x(1);
                    a.set_y(2);
                    assert_eq!(a.x(), 1);
                    assert_eq!(a.y(), 2);
                }
                {
                    let b = item.add_a();
                    b.set_x(3);
                    b.set_y(4);
                    assert_eq!(b.x(), 3);
                    assert_eq!(b.y(), 4);
                }
            }
            let view = mv.close().expect("close failed");

            // view can also be used directly after closing
            assert_eq!(view.len(), 1);
            let mut item = view.at(0);
            let a = item.next().unwrap();
            match a {
                AbRef::A(a) => {
                    assert_eq!(a.x(), 1);
                    assert_eq!(a.y(), 2);
                }
                AbRef::B(_) => panic!("unexpected variant B"),
            }
        }

        let index_resource = storage
            .read_and_check_schema("multivector_index", "index(Some schema)")
            .expect("read_and_check_schema failed");
        use crate::SliceExt;
        let index = <&[IndexType16]>::from_bytes(index_resource).expect("Corrupted data");
        let resource = storage
            .read_and_check_schema("multivector", "Some schema")
            .expect("read_and_check_schema failed");
        let mv: MultiArrayView<Ab> = MultiArrayView::new(index, resource);

        assert_eq!(mv.len(), 1);
        let mut item = mv.at(0);
        let a = item.next().unwrap();
        match a {
            AbRef::A(a) => {
                assert_eq!(a.x(), 1);
                assert_eq!(a.y(), 2);
            }
            AbRef::B(_) => panic!("unexpected variant B"),
        }
        let b = item.next().unwrap();
        match b {
            AbRef::A(a) => {
                assert_eq!(a.x(), 3);
                assert_eq!(a.y(), 4);
            }
            AbRef::B(_) => panic!("unexpected variant B"),
        }

        let x = {
            // test clone and lifetime of returned reference
            let mv_copy = mv.clone();
            mv_copy.at(0).next().unwrap()
        };
        match x {
            AbRef::A(a) => {
                assert_eq!(a.x(), 1);
                assert_eq!(a.y(), 2);
            }
            AbRef::B(_) => panic!("unexpected variant B"),
        }

        let x = {
            // test clone and lifetime of returned reference
            let mv_copy = mv.clone();
            mv_copy.iter().next().unwrap().next().unwrap()
        };
        match x {
            AbRef::A(a) => {
                assert_eq!(a.x(), 1);
                assert_eq!(a.y(), 2);
            }
            AbRef::B(_) => panic!("unexpected variant B"),
        }
    }
}
