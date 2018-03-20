use archive::Struct;
use memory;
use bytewriter;
use bytereader;

use std::marker;

pub struct Vector<T> {
    data: Vec<bytewriter::StreamType>,
    _phantom: marker::PhantomData<T>,
}

impl<T> Vector<T>
where
    T: Struct,
{
    pub fn new(len: usize) -> Self {
        let size = Self::size(len);
        let mut data = Vec::with_capacity(size);
        data.resize(size, 0 as bytewriter::StreamType);
        Self {
            data,
            _phantom: marker::PhantomData,
        }
    }

    pub fn size_in_bytes(&self) -> usize {
        self.data.len() - memory::PADDING_SIZE
    }

    pub fn len(&self) -> usize {
        self.size_in_bytes() / T::SIZE_IN_BYTES
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn reserve(&mut self, len: usize) {
        self.data.reserve(Self::size(len))
    }

    /// Calculate size in bytes (with padding) needed to store len-many elements.
    fn size(len: usize) -> usize {
        len * T::SIZE_IN_BYTES + memory::PADDING_SIZE
    }

    pub fn at(&self, idx: usize) -> &T {
        unsafe { &*(&self.data[idx * T::SIZE_IN_BYTES] as bytereader::StreamType as *const T) }
    }

    pub fn at_mut(&mut self, idx: usize) -> &mut T {
        unsafe {
            &mut *(&mut self.data[idx * T::SIZE_IN_BYTES] as *mut bytewriter::StreamType as *mut T)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::slice;

    #[derive(Debug, PartialEq)]
    struct A {
        first_byte: u8,
    }

    impl A {
        pub fn x(&self) -> u32 {
            read_bytes!(u32, &self.first_byte, 0, 16)
        }

        pub fn y(&self) -> u32 {
            read_bytes!(u32, &self.first_byte, 16, 32)
        }

        pub fn set_x(&mut self, value: u32) {
            let buffer =
                unsafe { slice::from_raw_parts_mut(&mut self.first_byte, Self::SIZE_IN_BYTES) };
            write_bytes!(u32; value, buffer, 0, 16);
        }

        pub fn set_y(&mut self, value: u32) {
            let buffer =
                unsafe { slice::from_raw_parts_mut(&mut self.first_byte, Self::SIZE_IN_BYTES) };
            write_bytes!(u32; value, buffer, 16, 32);
        }
    }

    impl Struct for A {
        const SCHEMA: &'static str = "struct A { }";
        const SIZE_IN_BYTES: usize = 4;
    }

    #[test]
    fn test_vector_mut_at() {
        let mut v: Vector<A> = Vector::new(1);
        {
            let a = v.at_mut(0);
            a.set_x(1);
            assert_eq!(a.x(), 1);
            a.set_y(2);
            assert_eq!(a.y(), 2);
        }
        let a = v.at(0);
        assert_eq!(a.x(), 1);
        assert_eq!(a.y(), 2);
    }
}
