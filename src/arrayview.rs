use archive::Struct;
use handle::Handle;
use storage::MemoryDescriptor;

use std::fmt;
use std::iter;
use std::marker;
use std::slice;

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
/// define_struct!(A, AMut, "no_schema", 4,
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
pub struct ArrayView<'a, T: 'a> {
    data: &'a [u8],
    _phantom: marker::PhantomData<T>,
}

impl<'a, T: Struct> ArrayView<'a, T> {
    /// Creates a new `ArrayView` to the data at the given address.
    ///
    /// The returned array view does not own the data.
    pub fn new(mem_descr: &MemoryDescriptor) -> Self {
        Self {
            data: unsafe { slice::from_raw_parts(mem_descr.data(), mem_descr.size_in_bytes()) },
            _phantom: marker::PhantomData,
        }
    }

    /// Number of elements in the array.
    pub fn len(&self) -> usize {
        self.data.len() / T::SIZE_IN_BYTES
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
    pub fn at(&self, index: usize) -> Handle<'a, T> {
        let index = index * T::SIZE_IN_BYTES;
        assert!(index + T::SIZE_IN_BYTES <= self.data.len());
        Handle::new(T::from(&self.data[index]))
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

impl<'a, T: Struct> fmt::Debug for ArrayView<'a, T> {
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

impl<'a, T: Struct> AsRef<[u8]> for ArrayView<'a, T> {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

/// Iterator through elements of `ArrayView`.
#[derive(Clone)]
pub struct ArrayViewIter<'a, T: 'a + Struct> {
    view: &'a ArrayView<'a, T>,
    next_pos: usize,
}

impl<'a, T: Struct> iter::Iterator for ArrayViewIter<'a, T> {
    type Item = Handle<'a, T>;
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

impl<'a, T: Struct> iter::ExactSizeIterator for ArrayViewIter<'a, T> {
    fn len(&self) -> usize {
        self.view.len()
    }
}

impl<'a, T: Struct> fmt::Debug for ArrayViewIter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let preview = self.view
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
