use crate::error::ResourceStorageError;
use crate::structs::Struct;

/// Enhanced slices of flatdata Structs so that they can be created from bytes / converted to bytes
/// Note: TryFrom/AsRef cannot be used, since slice is a foreign type
pub trait SliceExt<'a>
where
    Self: Sized,
{
    /// Create a slice from an array of bytes
    fn from_bytes(data: &'a [u8]) -> Result<Self, ResourceStorageError>;

    /// Get byte representation of the slice
    fn as_bytes(&self) -> &'a [u8];
}

impl<'a, T> SliceExt<'a> for &'a [T]
where
    T: Struct,
{
    fn from_bytes(mut data: &[u8]) -> Result<Self, ResourceStorageError> {
        if data.len() % T::SIZE_IN_BYTES != 0 {
            return Err(ResourceStorageError::UnexpectedDataSize);
        }
        if T::IS_OVERLAPPING_WITH_NEXT {
            if data.len() < T::SIZE_IN_BYTES {
                return Err(ResourceStorageError::UnexpectedDataSize);
            }
            data = &data[..data.len() - T::SIZE_IN_BYTES];
        }
        unsafe {
            Ok(std::slice::from_raw_parts(
                data.as_ptr() as *const T,
                data.len() / T::SIZE_IN_BYTES,
            ))
        }
    }

    fn as_bytes(&self) -> &'a [u8] {
        let len = if T::IS_OVERLAPPING_WITH_NEXT {
            self.len() + 1
        } else {
            self.len()
        };
        unsafe { std::slice::from_raw_parts(self.as_ptr() as *const u8, len * T::SIZE_IN_BYTES) }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use super::*;
    use crate::{
        structs::Struct,
        test::{A, B, R},
        Vector,
    };

    #[test]
    #[allow(clippy::reversed_empty_ranges)]
    fn range() {
        let mut vec: Vector<R> = Vector::with_len(3);
        vec[0].set_first_x(10);
        vec[1].set_first_x(20);
        vec[2].set_first_x(30);

        assert_eq!(vec.len(), 3);
        assert_eq!(vec[0].x(), 10..20);
        assert_eq!(vec[1].x(), 20..30);
        assert_eq!(vec[2].x(), 30..0);

        assert_eq!(vec[0..1].len(), 1);
        assert_eq!(vec[0..1][0].x(), 10..20);

        let view = vec.as_view();
        assert_eq!(view.len(), 2);
        assert_eq!(view[0].x(), 10..20);
        assert_eq!(view[1].x(), 20..30);

        assert_eq!(vec[0..1].len(), 1);
        assert_eq!(vec[0..1][0].x(), 10..20);
    }

    #[test]
    fn into_iter() {
        for _ in create_values(10).as_view() {}
    }

    #[test]
    fn new_and_clone() {
        let mut buffer = vec![255_u8; A::SIZE_IN_BYTES];
        buffer.extend(vec![0_u8; A::SIZE_IN_BYTES * 10]);
        let data = &buffer[..];
        let view = <&[A]>::from_bytes(data).unwrap();
        assert_eq!(11, view.len());
        let first = &view[0];
        assert_eq!(65535, first.x());
        assert_eq!(65535, first.y());
        for x in view.iter().skip(1) {
            assert_eq!(0, x.x());
            assert_eq!(0, x.y());
        }
    }

    fn create_values(size: usize) -> Vector<B> {
        let mut v: Vector<B> = Vector::with_len(size);
        for i in 0..size as u32 {
            let a = &mut v[i as usize];
            a.set_id(i);
        }
        v
    }

    #[test]
    fn reverse() {
        let v = create_values(10);
        let iter = v.as_view().iter().rev();
        let data: Vec<_> = iter.map(|x| x.id()).collect();
        assert_eq!(data, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    fn test_fused_iterator(mut iter: impl Iterator, size: usize) {
        for _ in 0..size {
            iter.next().unwrap();
        }
        if iter.next().is_some() {
            panic!("Iterator did not end properly");
        }
        if iter.next().is_some() {
            panic!("Iterator did not fuse properly");
        }
    }

    #[test]
    fn fused() {
        let v = create_values(100);
        test_fused_iterator(v.as_view().iter(), 100);
        test_fused_iterator(v.as_view().iter().rev(), 100);
    }

    #[test]
    #[allow(clippy::iter_next_slice)]
    fn slice() {
        let v = create_values(10);
        let view = v.as_view();

        assert_eq!(view.len(), 10);
        assert_eq!(view[2..].len(), 8);
        assert_eq!(view[2..].iter().next().unwrap().id(), 2);
        assert_eq!(view[..8].len(), 8);
        assert_eq!(view[..8].iter().next().unwrap().id(), 0);
        assert_eq!(view[2..8].len(), 6);
        assert_eq!(view[2..8].iter().next().unwrap().id(), 2);
    }

    #[test]
    fn debug() {
        let v = create_values(10);
        let view = v.as_view();

        let content = "[\
                       B { id: 0 }, \
                       B { id: 1 }, \
                       B { id: 2 }, \
                       B { id: 3 }, \
                       B { id: 4 }, \
                       B { id: 5 }, \
                       B { id: 6 }, \
                       B { id: 7 }, \
                       B { id: 8 }, \
                       B { id: 9 }\
                       ]";

        assert_eq!(format!("{:?}", view), content);
        assert_eq!(
            format!("{:?}", view.iter()),
            "Iter(".to_string() + content + ")"
        );
        let mut iter = view.iter();
        for _ in 0..9 {
            iter.next();
        }
        assert_eq!(format!("{:?}", iter), "Iter([B { id: 9 }])");
    }
}
