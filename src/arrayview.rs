use archive::Struct;

use std::fmt;
use std::iter;
use std::marker;

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
    T: for<'b> Struct<'b>,
{
    data: &'a [u8],
    _phantom: marker::PhantomData<T>,
}

impl<'a, T> ArrayView<'a, T>
where
    T: for<'b> Struct<'b>,
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
    pub fn at(&self, index: usize) -> <T as Struct>::Item {
        let index = index * <T as Struct>::SIZE_IN_BYTES;
        assert!(index + <T as Struct>::SIZE_IN_BYTES <= self.data.len());
        T::create(&self.data[index..])
    }

    /// Returns an iterator to the elements of the array.
    pub fn iter(&'a self) -> ArrayViewIter<T> {
        ArrayViewIter {
            view: self,
            next_pos: 0,
        }
    }

    /// Returns a raw bytes representation of the underlying array data.
    pub fn as_bytes(&self) -> &[u8] {
        self.data
    }
}

impl<'a, T> fmt::Debug for ArrayView<'a, T>
where
    T: for<'b> Struct<'b>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let preview: Vec<_> = self.iter().take(super::DEBUG_PREVIEW_LEN).collect();
        write!(
            f,
            "ArrayView {{ len: {}, data: {:?}{} }}",
            self.len(),
            preview,
            if self.len() <= super::DEBUG_PREVIEW_LEN {
                ""
            } else {
                "..."
            }
        )
    }
}

impl<'a, T> AsRef<[u8]> for ArrayView<'a, T>
where
    T: for<'b> Struct<'b>,
{
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Iterator through elements of `ArrayView`.
#[derive(Clone)]
pub struct ArrayViewIter<'a, T: 'a>
where
    T: for<'b> Struct<'b>,
{
    view: &'a ArrayView<'a, T>,
    next_pos: usize,
}

impl<'a, T> iter::Iterator for ArrayViewIter<'a, T>
where
    T: for<'b> Struct<'b>,
{
    type Item = <T as Struct<'a>>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next_pos < self.view.len() {
            let element = self.view.at(self.next_pos);
            self.next_pos += 1;
            Some(element)
        } else {
            None
        }
    }
}

impl<'a, T> iter::ExactSizeIterator for ArrayViewIter<'a, T>
where
    T: for<'b> Struct<'b>,
{
    fn len(&self) -> usize {
        self.view.len()
    }
}

impl<'a, T> fmt::Debug for ArrayViewIter<'a, T>
where
    T: for<'b> Struct<'b>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let preview = self
            .view
            .iter()
            .skip(self.next_pos)
            .take(super::DEBUG_PREVIEW_LEN);
        write!(
            f,
            "ArrayViewIter {{ data: {:?}{} }}",
            preview,
            if self.view.len() - self.next_pos <= super::DEBUG_PREVIEW_LEN {
                ""
            } else {
                "..."
            }
        )
    }
}

#[cfg(test)]
mod test {
    use archive::Struct;
    use memory;

    #[test]
    #[allow(dead_code)]
    fn test() {
        define_struct!(
            A,
            RefA,
            RefMutA,
            "no_schema",
            4,
            (x, set_x, u32, 0, 16),
            (y, set_y, u32, 16, 16)
        );

        let mut buffer = vec![255_u8; 4];
        buffer.extend(vec![0_u8; A::SIZE_IN_BYTES * 10 + memory::PADDING_SIZE]);
        let data = &buffer[..buffer.len() - memory::PADDING_SIZE];
        let view: super::ArrayView<A> = super::ArrayView::new(&data);
        assert_eq!(11, view.len());
        let first = view.at(0);
        assert_eq!(65535, first.x());
        assert_eq!(65535, first.y());
        for x in view.iter().skip(1) {
            assert_eq!(0, x.x());
            assert_eq!(0, x.y());
        }

        let _view_copy = view.clone();
    }
}
