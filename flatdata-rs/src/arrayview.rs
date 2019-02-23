use crate::archive::{RefFactory, Struct};
use crate::vector::Vector;

use std::fmt;
use std::iter;
use std::marker;
use std::ops::{Bound, RangeBounds};

/// A read-only view on a contiguous sequence of flatdata structs of the same
/// type `T`.
///
/// The sequence is written using [`Vector`] or [`ExternalVector`]. The former
/// provides a method to create an `ArrayView` to it. Note that, that an array
/// view does not hold the data itself.
///
/// An archive provides a getter for each vector resource, which returns an
/// array view.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate flatdata;
/// # fn main() {
/// use flatdata::{ArrayView, Vector};
///
/// define_struct!(
///     A,
///     RefA,
///     RefMutA,
///     "no_schema",
///     4,
///     (x, set_x, u32, 0, 16),
///     (y, set_y, u32, 16, 16)
/// );
///
/// let mut v: Vector<A> = Vector::with_len(1);
/// {
///     let mut a = v.at_mut(0);
///     a.set_x(1);
///     a.set_y(2);
/// }
///
/// let view: ArrayView<_> = v.as_view();
/// let a = view.at(0);
/// assert_eq!(a.x(), 1);
/// assert_eq!(a.y(), 2);
/// # }
/// ```
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
        self.data.len() / <T as Struct>::SIZE_IN_BYTES
    }

    /// Return `true` if the array is empty.
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
        let index = self.data_index(index);
        assert!(index + <T as Struct>::SIZE_IN_BYTES <= self.data.len());
        T::create(&self.data[index..])
    }

    /// Slice this array view by a given range.
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
        let data_end = match range.end_bound() {
            Bound::Included(&idx) => self.data_index(idx + 1),
            Bound::Excluded(&idx) => self.data_index(idx),
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
    use crate::archive::Struct;
    use crate::memory;

    define_struct!(
        Value,
        RefValue,
        RefMutValue,
        "no_schema",
        4,
        (value, set_value, u32, 0, 32)
    );

    define_struct!(
        Point,
        RefPoint,
        RefMutPoint,
        "no_schema",
        4,
        (x, set_x, u32, 0, 16),
        (y, set_y, u32, 16, 16)
    );

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
