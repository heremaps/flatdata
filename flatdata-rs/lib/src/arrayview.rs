use crate::{
    structs::{RefFactory, Struct},
    vector::Vector,
};

use std::{
    fmt, iter, marker,
    ops::{Bound, RangeBounds},
};

/// A read-only view on a contiguous sequence of flatdata structs of the same
/// type `T`.
///
/// The sequence is written using [`Vector`] or [`ExternalVector`]. For detailed
/// examples see either of the two.
///
/// An archive provides a getter for each vector resource, which returns an
/// array view.
///
/// [`Vector`]: struct.Vector.html
/// [`ExternalVector`]: struct.ExternalVector.html
#[derive(Clone)]
pub struct ArrayView<'a, T>
where
    T: RefFactory,
{
    data: &'a [u8],
    _phantom: marker::PhantomData<T>,
}

impl<'a, T> ArrayView<'a, T>
where
    T: RefFactory,
{
    /// Creates a new `ArrayView` to the data at the given address.
    ///
    /// The returned array view does not own the data.
    pub fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            _phantom: marker::PhantomData,
        }
    }

    /// Number of elements in the array.
    pub fn len(&self) -> usize {
        if <T as Struct>::IS_OVERLAPPING_WITH_NEXT {
            self.data.len() / <T as Struct>::SIZE_IN_BYTES - 1
        } else {
            self.data.len() / <T as Struct>::SIZE_IN_BYTES
        }
    }

    /// Returns `true` if the array is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a read-only handle to the element in the array at position
    /// `index`.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to `ArrayView::len()`.
    pub fn at(&self, index: usize) -> <T as Struct<'a>>::Item {
        assert!(index < self.len());
        let index = self.data_index(index);
        T::create(&self.data[index..])
    }

    /// Slices this array view by a given range.
    ///
    /// # Panics
    ///
    /// Panics if the range is outside of bounds of array view.
    pub fn slice<R: RangeBounds<usize>>(&self, range: R) -> Self {
        let data_start = match range.start_bound() {
            Bound::Included(&idx) => self.data_index(idx),
            Bound::Excluded(&idx) => self.data_index(idx + 1),
            Bound::Unbounded => 0,
        };
        let sentinel = <T as Struct>::IS_OVERLAPPING_WITH_NEXT as usize;
        let data_end = match range.end_bound() {
            Bound::Included(&idx) => self.data_index(idx + 1 + sentinel),
            Bound::Excluded(&idx) => self.data_index(idx + sentinel),
            Bound::Unbounded => self.data.len(),
        };
        Self::new(&self.data[data_start..data_end])
    }

    /// Returns an iterator to the elements of the array.
    pub fn iter(&self) -> ArrayViewIter<'a, T> {
        ArrayViewIter { view: self.clone() }
    }

    /// Returns a raw bytes representation of the underlying array data.
    pub fn as_bytes(&self) -> &[u8] {
        self.data
    }

    fn data_index(&self, index: usize) -> usize {
        index * <T as Struct>::SIZE_IN_BYTES
    }
}

impl<'a, T> From<&'a Vector<T>> for ArrayView<'a, T>
where
    T: RefFactory,
{
    fn from(v: &'a Vector<T>) -> Self {
        v.as_view()
    }
}

pub(crate) fn debug_format<'a, T>(
    name: &str,
    iter: ArrayViewIter<'a, T>,
    f: &mut fmt::Formatter,
) -> fmt::Result
where
    T: RefFactory,
{
    let len = iter.len();
    let preview: Vec<_> = iter.take(super::DEBUG_PREVIEW_LEN).collect();
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

impl<'a, T> fmt::Debug for ArrayView<'a, T>
where
    T: RefFactory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_format("ArrayView", self.iter(), f)
    }
}

impl<'a, T> IntoIterator for ArrayView<'a, T>
where
    T: RefFactory,
{
    type Item = <ArrayViewIter<'a, T> as Iterator>::Item;
    type IntoIter = ArrayViewIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &ArrayView<'a, T>
where
    T: RefFactory,
{
    type Item = <ArrayViewIter<'a, T> as Iterator>::Item;
    type IntoIter = ArrayViewIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> AsRef<[u8]> for ArrayView<'a, T>
where
    T: RefFactory,
{
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Iterator through elements of `ArrayView`.
#[derive(Clone)]
pub struct ArrayViewIter<'a, T>
where
    T: RefFactory,
{
    view: ArrayView<'a, T>,
}

impl<'a, T> iter::Iterator for ArrayViewIter<'a, T>
where
    T: RefFactory,
{
    type Item = <T as Struct<'a>>::Item;
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

impl<'a, T> iter::ExactSizeIterator for ArrayViewIter<'a, T>
where
    T: RefFactory,
{
    fn len(&self) -> usize {
        self.view.len()
    }
}

impl<'a, T> iter::DoubleEndedIterator for ArrayViewIter<'a, T>
where
    T: RefFactory,
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
impl<'a, T> iter::FusedIterator for ArrayViewIter<'a, T> where T: RefFactory {}

impl<'a, T> fmt::Debug for ArrayViewIter<'a, T>
where
    T: RefFactory,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        debug_format("ArrayViewIter", self.clone(), f)
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;
    use crate::{memory, structs::Struct};

    define_struct!(
        Value,
        RefValue,
        RefMutValue,
        "no_schema",
        4,
        (value, set_value, u32, u32, 0, 32)
    );

    define_struct!(
        Point,
        RefPoint,
        RefMutPoint,
        "no_schema",
        4,
        (x, set_x, u32, u32, 0, 16),
        (y, set_y, u32, u32, 16, 16)
    );

    define_struct!(
        R,
        RefR,
        RefMutR,
        "no_schema",
        4,
        (first_x, set_first_x, u32, u32, 0, 16),
        range(x, u32, 0, 16)
    );

    #[test]
    fn range() {
        let mut vec: Vector<R> = Vector::with_len(3);
        vec.at_mut(0).set_first_x(10);
        vec.at_mut(1).set_first_x(20);
        vec.at_mut(2).set_first_x(30);

        let view = vec.as_view();
        assert_eq!(view.len(), 2);
        assert_eq!(view.at(0).x(), 10..20);
        assert_eq!(view.at(1).x(), 20..30);

        assert_eq!(view.slice(0..1).len(), 1);
        assert_eq!(view.slice(0..1).at(0).x(), 10..20);
    }

    #[test]
    fn into_iter() {
        for _ in create_values(10).as_view() {}
        for _ in &create_values(10).as_view() {}
    }

    #[test]
    fn test() {
        let mut buffer = vec![255_u8; 4];
        buffer.extend(vec![0_u8; Point::SIZE_IN_BYTES * 10 + memory::PADDING_SIZE]);
        let data = &buffer[..buffer.len() - memory::PADDING_SIZE];
        let view: ArrayView<Point> = ArrayView::new(&data);
        assert_eq!(11, view.len());
        let first = view.at(0);
        assert_eq!(65535, first.x());
        assert_eq!(65535, first.y());
        for x in view.iter().skip(1) {
            assert_eq!(0, x.x());
            assert_eq!(0, x.y());
        }

        let x = {
            // test clone and lifetime of returned reference
            let view_copy = view.clone();
            view_copy.at(0)
        };
        assert_eq!(65535, x.x());
        assert_eq!(65535, x.y());

        let x = {
            // test clone and lifetime of returned reference
            let view_copy = view.clone();
            view_copy.iter().next().unwrap()
        };
        assert_eq!(65535, x.x());
        assert_eq!(65535, x.y());
    }

    fn create_values(size: usize) -> Vector<Value> {
        let mut v: Vector<Value> = Vector::with_len(size);
        for i in 0..size as u32 {
            let mut a = v.at_mut(i as usize);
            a.set_value(i);
        }
        v
    }

    #[test]
    fn from() {
        let v = create_values(10);
        let x: ArrayView<_> = (&v).into();
        assert_eq!(x.len(), 10);
    }

    #[test]
    fn reverse() {
        let v = create_values(10);
        let iter = v.as_view().iter().rev();
        let data: Vec<_> = iter.map(|x| x.value()).collect();
        assert_eq!(data, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
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
        let v = create_values(100);
        test_fused_iterator(v.as_view().iter(), 100);
        test_fused_iterator(v.as_view().iter().rev(), 100);
    }

    #[test]
    fn test_slice() {
        let v = create_values(10);
        let view: ArrayView<_> = (&v).into();

        assert_eq!(view.len(), 10);
        assert_eq!(view.slice(2..).len(), 8);
        assert_eq!(view.slice(2..).iter().next().unwrap().value(), 2);
        assert_eq!(view.slice(..8).len(), 8);
        assert_eq!(view.slice(..8).iter().next().unwrap().value(), 0);
        assert_eq!(view.slice(2..8).len(), 6);
        assert_eq!(view.slice(2..8).iter().next().unwrap().value(), 2);
    }

    #[test]
    fn debug() {
        let v = create_values(100);
        let view = v.as_view();

        let content = " { len: 100, data: [\
                       Value { value: 0 }, \
                       Value { value: 1 }, \
                       Value { value: 2 }, \
                       Value { value: 3 }, \
                       Value { value: 4 }, \
                       Value { value: 5 }, \
                       Value { value: 6 }, \
                       Value { value: 7 }, \
                       Value { value: 8 }, \
                       Value { value: 9 }\
                       ]... }";

        assert_eq!(format!("{:?}", view), "ArrayView".to_string() + content);
        assert_eq!(
            format!("{:?}", view.iter()),
            "ArrayViewIter".to_string() + content
        );
        let mut iter = view.iter();
        for _ in 0..99 {
            iter.next();
        }
        assert_eq!(
            format!("{:?}", iter),
            "ArrayViewIter { len: 1, data: [Value { value: 99 }] }"
        );
    }
}
