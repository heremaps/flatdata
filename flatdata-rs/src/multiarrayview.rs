use crate::archive::{
    IndexRefFactory, IndexStruct, VariadicRef, VariadicRefFactory, VariadicStruct,
};
use crate::arrayview::ArrayView;

use std::fmt;
use std::iter;
use std::marker;
use std::ops::{Bound, RangeBounds};

/// A read-only view on a multivector.
///
/// For the detailed description of multivector and examples, cf.
/// [`MultiVector`].
///
/// [`MultiVector`]: struct.MultiVector.html
#[derive(Clone)]
pub struct MultiArrayView<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    index: ArrayView<'a, Idx>,
    data: &'a [u8],
    _phantom: marker::PhantomData<Ts>,
}

impl<'a, Idx, Ts> MultiArrayView<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    /// Creates a new `MultiArrayView` to the data at the given address.
    ///
    /// The returned array view does not own the data.
    pub fn new(index: ArrayView<'a, Idx>, data_mem_descr: &'a [u8]) -> Self {
        Self {
            index,
            data: &data_mem_descr,
            _phantom: marker::PhantomData,
        }
    }

    /// Number of indexed items in the array.
    ///
    /// Note that this is not the *total* number of overall elements stored in
    /// the array. An item may be also empty.
    pub fn len(&self) -> usize {
        // last index element is a sentinel
        self.index.len() - 1
    }

    /// Returns `true` if no item is stored in the array.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a read-only iterator to the elements of the item at position
    /// `index`.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to `MultiArrayView::len()`.
    pub fn at(&self, index: usize) -> MultiArrayViewItemIter<'a, Ts> {
        let start = <Idx as IndexStruct>::index(self.index.at(index));
        let end = <Idx as IndexStruct>::index(self.index.at(index + 1));
        MultiArrayViewItemIter {
            data: &self.data[start..end],
            _phantom: marker::PhantomData,
        }
    }

    /// Slice this array view by a given range.
    ///
    /// # Panics
    ///
    /// Panics if the range is outside of bounds of array view.
    pub fn slice<R: RangeBounds<usize>>(&self, range: R) -> Self {
        let index_start = match range.start_bound() {
            Bound::Included(&idx) => idx,
            Bound::Excluded(&idx) => idx + 1,
            Bound::Unbounded => 0,
        };
        // include one more element (sentinel)
        let index_end = match range.end_bound() {
            Bound::Included(&idx) => idx + 2,
            Bound::Excluded(&idx) => idx + 1,
            Bound::Unbounded => self.index.len(),
        };
        Self::new(self.index.slice(index_start..index_end), self.data)
    }

    /// Returns an iterator through the indexed items of the array.
    pub fn iter(&self) -> MultiArrayViewIter<'a, Idx, Ts> {
        MultiArrayViewIter { view: self.clone() }
    }
}

/// Iterator through elements of an array item.
///
/// An item may be empty.
#[derive(Clone)]
pub struct MultiArrayViewItemIter<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    data: &'a [u8],
    _phantom: marker::PhantomData<&'a Ts>,
}

impl<'a, Ts> iter::Iterator for MultiArrayViewItemIter<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    type Item = <Ts as VariadicStruct<'a>>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.data.is_empty() {
            let type_index = self.data[0];
            self.data = &self.data[1..];
            let res = <Ts as VariadicStruct>::create(type_index, &self.data);
            self.data = &self.data[res.size_in_bytes()..];
            Some(res)
        } else {
            None
        }
    }
}

// we always check -> iterator is already fused
impl<'a, Ts> iter::FusedIterator for MultiArrayViewItemIter<'a, Ts> where Ts: VariadicRefFactory {}

impl<'a, Ts> fmt::Debug for MultiArrayViewItemIter<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let preview: Vec<_> = self.clone().collect();
        write!(f, "MultiArrayViewItemIter {{ data: {:?} }}", preview,)
    }
}

fn debug_format<'a, Idx, Ts>(
    name: &str,
    iter: MultiArrayViewIter<'a, Idx, Ts>,
    f: &mut fmt::Formatter,
) -> fmt::Result
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    let len = iter.len();
    let preview: Vec<(usize, Vec<_>)> = iter
        .take(super::DEBUG_PREVIEW_LEN)
        .enumerate()
        .map(|(index, item)| (index, item.collect()))
        .collect();
    write!(
        f,
        "{} {{ len: {}, data: {:?}{} }}",
        name,
        len,
        preview,
        if len <= super::DEBUG_PREVIEW_LEN {
            ""
        } else {
            "..."
        }
    )
}

impl<'a, Idx, Ts> fmt::Debug for MultiArrayView<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_format("MultiArrayView", self.iter(), f)
    }
}

impl<'a, Idx, Ts: 'a> IntoIterator for MultiArrayView<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    type Item = <MultiArrayViewIter<'a, Idx, Ts> as Iterator>::Item;
    type IntoIter = MultiArrayViewIter<'a, Idx, Ts>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, Idx, Ts: 'a> IntoIterator for &MultiArrayView<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    type Item = <MultiArrayViewIter<'a, Idx, Ts> as Iterator>::Item;
    type IntoIter = MultiArrayViewIter<'a, Idx, Ts>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator through items of an multivector.
#[derive(Clone)]
pub struct MultiArrayViewIter<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    view: MultiArrayView<'a, Idx, Ts>,
}

impl<'a, Idx, Ts> fmt::Debug for MultiArrayViewIter<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_format("MultiArrayViewIter", self.clone(), f)
    }
}

impl<'a, Idx, Ts: 'a> iter::Iterator for MultiArrayViewIter<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    type Item = MultiArrayViewItemIter<'a, Ts>;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.view.is_empty() {
            let element = self.view.at(0);
            self.view = self.view.slice(1..);
            Some(element)
        } else {
            None
        }
    }
}

impl<'a, Idx, Ts: 'a> iter::DoubleEndedIterator for MultiArrayViewIter<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if !self.view.is_empty() {
            let last_pos = self.view.len() - 1;
            let element = self.view.at(last_pos);
            self.view = self.view.slice(..last_pos);
            Some(element)
        } else {
            None
        }
    }
}

// we always check -> iterator is already fused
impl<'a, Idx, Ts: 'a> iter::FusedIterator for MultiArrayViewIter<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
}

impl<'a, Idx, Ts: 'a> iter::ExactSizeIterator for MultiArrayViewIter<'a, Idx, Ts>
where
    Idx: IndexRefFactory,
    Ts: VariadicRefFactory,
{
    fn len(&self) -> usize {
        self.view.len()
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use crate::memstorage::MemoryResourceStorage;
    use crate::storage::create_multi_vector;

    define_index!(Idx, RefIdx, RefMutIdx, "some_idx_schema", 4, 32);

    define_struct!(
        Value,
        RefValue,
        RefMutValue,
        "no_schema",
        3,
        (value, set_value, u32, 0, 24)
    );

    define_struct!(
        Point,
        RefPoint,
        RefMutPoint,
        "no_schema",
        4,
        (x, set_x, u32, 0, 24),
        (y, set_y, u32, 0, 24)
    );

    define_variadic_struct!(Variant, RefVariant, BuilderVariant, Idx, 0 => (Value, add_value), 1 => (Point, add_point) );

    fn create_view<'a>(
        storage: &'a MemoryResourceStorage,
        size: usize,
    ) -> MultiArrayView<'a, Idx, Variant> {
        let mut mv = create_multi_vector::<Idx, Variant>(&*storage, "multivector", "Some schema")
            .expect("failed to create MultiVector");

        for i in 0..size {
            let mut item = mv.grow().expect("grow failed");

            let mut a = item.add_value();
            a.set_value(i as u32);

            let mut b = item.add_point();
            b.set_x((i + size) as u32);
            b.set_y((i + 2 * size) as u32);
        }

        mv.close().expect("close failed")
    }

    #[test]
    fn slice() {
        let storage = MemoryResourceStorage::new("/root/resources");
        let view = create_view(&storage, 10);

        let value = |mut iter: MultiArrayViewItemIter<_>| match iter.next().unwrap() {
            RefVariant::Value(v) => v.value(),
            otherwise => panic!("unexpected value: {:?}", otherwise),
        };

        assert_eq!(view.len(), 10);
        assert_eq!(view.slice(2..).len(), 8);
        assert_eq!(value(view.slice(2..).iter().next().unwrap()), 2);
        assert_eq!(view.slice(..8).len(), 8);
        assert_eq!(value(view.slice(..8).iter().next().unwrap()), 0);
        assert_eq!(view.slice(2..8).len(), 6);
        assert_eq!(value(view.slice(2..8).iter().next().unwrap()), 2);
    }

    #[test]
    fn reverse() {
        let storage = MemoryResourceStorage::new("/root/resources");
        let view = create_view(&storage, 10);

        let value = |mut iter: MultiArrayViewItemIter<_>| match iter.next().unwrap() {
            RefVariant::Value(v) => v.value(),
            otherwise => panic!("unexpected value: {:?}", otherwise),
        };

        let values: Vec<_> = view.iter().rev().map(|x| value(x)).collect();
        assert_eq!(values, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn debug() {
        let storage = MemoryResourceStorage::new("/root/resources");
        let view = create_view(&storage, 100);
        let content = " { len: 100, data: [\
                       (0, [Value { value: 0 }, Point { x: 200, y: 200 }]), \
                       (1, [Value { value: 1 }, Point { x: 201, y: 201 }]), \
                       (2, [Value { value: 2 }, Point { x: 202, y: 202 }]), \
                       (3, [Value { value: 3 }, Point { x: 203, y: 203 }]), \
                       (4, [Value { value: 4 }, Point { x: 204, y: 204 }]), \
                       (5, [Value { value: 5 }, Point { x: 205, y: 205 }]), \
                       (6, [Value { value: 6 }, Point { x: 206, y: 206 }]), \
                       (7, [Value { value: 7 }, Point { x: 207, y: 207 }]), \
                       (8, [Value { value: 8 }, Point { x: 208, y: 208 }]), \
                       (9, [Value { value: 9 }, Point { x: 209, y: 209 }])]... }";

        assert_eq!(
            format!("{:?}", view),
            "MultiArrayView".to_string() + content
        );
        assert_eq!(
            format!("{:?}", view.iter()),
            "MultiArrayViewIter".to_string() + content
        );
        assert_eq!(
            format!("{:?}", view.at(5)),
            "MultiArrayViewItemIter { data: [Value { value: 5 }, Point { x: 205, y: 205 }] }"
        );
        let mut iter = view.iter();
        for _ in 0..99 {
            iter.next();
        }
        assert_eq!(
            format!("{:?}", iter),
            "MultiArrayViewIter { len: 1, data: \
             [(0, [Value { value: 99 }, Point { x: 299, y: 299 }])] }"
        );
    }

    fn test_fused_iterator(mut iter: impl Iterator, size: usize) {
        for _ in 0..size {
            iter.next().unwrap();
        }
        if let Some(_) = iter.next() {
            assert!(false, "Iterator did not end properly");
        }
        if let Some(_) = iter.next() {
            assert!(false, "Iterator did not fuse properly");
        }
    }

    #[test]
    fn fused() {
        let storage = MemoryResourceStorage::new("/root/resources");
        let view = create_view(&storage, 100);

        test_fused_iterator(view.iter(), 100);
        test_fused_iterator(view.at(66), 2);
    }

    #[test]
    fn into_iter() {
        let storage = MemoryResourceStorage::new("/root/resources");
        let view = create_view(&storage, 100);

        for _ in &view {}
        for _ in view {}
    }
}
