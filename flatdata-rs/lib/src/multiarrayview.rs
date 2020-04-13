use crate::structs::{IndexStruct, VariadicRef, VariadicRefFactory, VariadicStruct};

use std::{fmt, iter, marker};

/// A read-only view on a multivector.
///
/// For the detailed description of multivector and examples, cf.
/// [`MultiVector`].
///
/// [`MultiVector`]: struct.MultiVector.html
pub struct MultiArrayView<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    index: &'a [Ts::Index],
    data: &'a [u8],
}

impl<'a, Ts> Clone for MultiArrayView<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    fn clone(&self) -> Self {
        Self { ..*self }
    }
}

impl<'a, Ts> MultiArrayView<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    /// Creates a new `MultiArrayView` to the data at the given address.
    ///
    /// The returned array view does not own the data.
    pub fn new(index: &'a [Ts::Index], data: &'a [u8]) -> Self {
        Self { index, data }
    }

    /// Number of indexed items in the array.
    ///
    /// Note that this is not the *total* number of overall elements stored in
    /// the array. An item may be also empty.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// Returns `true` if no item is stored in the array.
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Returns a read-only iterator to the elements of the item at position
    /// `index`.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to `MultiArrayView::len()`.
    pub fn at(&self, index: usize) -> MultiArrayViewItemIter<'a, Ts> {
        let range = <Ts::Index>::range(&self.index[index]);
        MultiArrayViewItemIter {
            data: &self.data[range],
            _phantom: marker::PhantomData,
        }
    }

    /// Slice this array view by a given range.
    ///
    /// # Panics
    ///
    /// Panics if the range is outside of bounds of array view.
    pub fn slice<R: std::slice::SliceIndex<[Ts::Index], Output = [Ts::Index]>>(
        &self,
        range: R,
    ) -> Self {
        Self::new(&self.index[range], self.data)
    }

    /// Returns an iterator through the indexed items of the array.
    pub fn iter(&self) -> MultiArrayViewIter<'a, Ts> {
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

fn debug_format<'a, Ts: 'a>(
    name: &str,
    iter: MultiArrayViewIter<'a, Ts>,
    f: &mut fmt::Formatter,
) -> fmt::Result
where
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

impl<'a, Ts: 'a> fmt::Debug for MultiArrayView<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_format("MultiArrayView", self.iter(), f)
    }
}

impl<'a, Ts: 'a> IntoIterator for MultiArrayView<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    type Item = <MultiArrayViewIter<'a, Ts> as Iterator>::Item;
    type IntoIter = MultiArrayViewIter<'a, Ts>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, Ts: 'a> IntoIterator for &MultiArrayView<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    type Item = <MultiArrayViewIter<'a, Ts> as Iterator>::Item;
    type IntoIter = MultiArrayViewIter<'a, Ts>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Iterator through items of an multivector.
#[derive(Clone)]
pub struct MultiArrayViewIter<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    view: MultiArrayView<'a, Ts>,
}

impl<'a, Ts: 'a> fmt::Debug for MultiArrayViewIter<'a, Ts>
where
    Ts: VariadicRefFactory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_format("MultiArrayViewIter", self.clone(), f)
    }
}

impl<'a, Ts: 'a> iter::Iterator for MultiArrayViewIter<'a, Ts>
where
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

impl<'a, Ts: 'a> iter::DoubleEndedIterator for MultiArrayViewIter<'a, Ts>
where
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
impl<'a, Ts: 'a> iter::FusedIterator for MultiArrayViewIter<'a, Ts> where Ts: VariadicRefFactory {}

impl<'a, Ts: 'a> iter::ExactSizeIterator for MultiArrayViewIter<'a, Ts>
where
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
    use crate::{
        memstorage::MemoryResourceStorage,
        storage::create_multi_vector,
        test::{Ab, AbRef},
    };

    fn create_view<'a>(storage: &'a MemoryResourceStorage, size: usize) -> MultiArrayView<'a, Ab> {
        let mut mv = create_multi_vector::<Ab>(&*storage, "multivector", "Some schema")
            .expect("failed to create MultiVector");

        for i in 0..size {
            let mut item = mv.grow().expect("grow failed");

            let b = item.add_b();
            b.set_id(i as u32);

            let a = item.add_a();
            a.set_x((i + size) as u32);
            a.set_y((i + 2 * size) as u32);
        }

        mv.close().expect("close failed")
    }

    #[test]
    fn slice() {
        let storage = MemoryResourceStorage::new("/root/resources");
        let view = create_view(&storage, 10);

        let value = |mut iter: MultiArrayViewItemIter<_>| match iter.next().unwrap() {
            AbRef::B(v) => v.id(),
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
            AbRef::B(v) => v.id(),
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
                       (0, [B { id: 0 }, A { x: 100, y: 200, e: Value }]), \
                       (1, [B { id: 1 }, A { x: 101, y: 201, e: Value }]), \
                       (2, [B { id: 2 }, A { x: 102, y: 202, e: Value }]), \
                       (3, [B { id: 3 }, A { x: 103, y: 203, e: Value }]), \
                       (4, [B { id: 4 }, A { x: 104, y: 204, e: Value }]), \
                       (5, [B { id: 5 }, A { x: 105, y: 205, e: Value }]), \
                       (6, [B { id: 6 }, A { x: 106, y: 206, e: Value }]), \
                       (7, [B { id: 7 }, A { x: 107, y: 207, e: Value }]), \
                       (8, [B { id: 8 }, A { x: 108, y: 208, e: Value }]), \
                       (9, [B { id: 9 }, A { x: 109, y: 209, e: Value }])]... }";

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
            "MultiArrayViewItemIter { data: [B { id: 5 }, A { x: 105, y: 205, e: Value }] }"
        );
        let mut iter = view.iter();
        for _ in 0..99 {
            iter.next();
        }
        assert_eq!(
            format!("{:?}", iter),
            "MultiArrayViewIter { len: 1, data: \
             [(0, [B { id: 99 }, A { x: 199, y: 299, e: Value }])] }"
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
