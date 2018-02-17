use archive::Struct;
use memory;
use bytewriter;
use bytereader;

use std::marker;

pub struct Vector<T> {
    data: Vec<bytewriter::StreamType>,
    _phantom: marker::PhantomData<T>,
}

impl<'a, T> Vector<T>
where
    T: Struct<'a>,
{
    pub fn new(len: usize) -> Self {
        let size = Self::size(len);
        let mut data = Vec::with_capacity(size);
        data.resize(Self::size(len), 0 as bytewriter::StreamType);

        Self {
            data: data,
            _phantom: marker::PhantomData,
        }
    }

    pub fn size_in_bytes(&self) -> usize {
        self.data.len() - memory::PADDING_SIZE
    }

    pub fn len(&self) -> usize {
        self.size_in_bytes() / T::SIZE_IN_BYTES
    }

    pub fn reserve(&mut self, len: usize) {
        self.data.reserve(Self::size(len))
    }

    fn size(len: usize) -> usize {
        len * T::SIZE_IN_BYTES + memory::PADDING_SIZE
    }

    pub fn at(&self, idx: usize) -> T {
        let addr = &self.data[idx * T::SIZE_IN_BYTES] as bytereader::StreamType;
        T::from(addr)
    }

    pub fn at_mut(&'a mut self, idx: usize) -> T::Mutator {
        let start = idx * T::SIZE_IN_BYTES;
        let end = start + T::SIZE_IN_BYTES;
        T::Mutator::from(&mut self.data[start..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::MutStruct;
    use std::convert;

    #[derive(Debug, Clone, PartialEq)]
    struct A {
        data: bytereader::StreamType,
    }

    impl A {
        pub fn x(&self) -> u32 {
            read_bytes!(u32, self.data, 0, 16)
        }

        pub fn y(&self) -> u32 {
            read_bytes!(u32, self.data, 16, 32)
        }
    }

    impl<'a> Struct<'a> for A {
        const SCHEMA: &'static str = "struct A { }";
        const SIZE_IN_BYTES: usize = 4;
        type Mutator = MutA<'a>;
    }

    impl convert::From<bytereader::StreamType> for A {
        fn from(data: bytereader::StreamType) -> A {
            Self { data: data }
        }
    }

    #[derive(Debug, PartialEq)]
    struct MutA<'a> {
        data: &'a mut [bytewriter::StreamType],
    }

    impl<'a> MutStruct<'a> for MutA<'a> {}

    impl<'a> convert::From<&'a mut [bytewriter::StreamType]> for MutA<'a> {
        fn from(data: &'a mut [bytewriter::StreamType]) -> MutA<'a> {
            Self { data: data }
        }
    }

    impl<'a> MutA<'a> {
        pub fn x(&self) -> u32 {
            read_bytes!(u32, &self.data[0], 0, 16)
        }

        pub fn y(&self) -> u32 {
            read_bytes!(u32, &self.data[0], 16, 32)
        }

        pub fn set_x(&mut self, value: u32) {
            write_bytes!(u32; value, self.data, 0, 16);
        }

        pub fn set_y(&mut self, value: u32) {
            write_bytes!(u32; value, self.data, 16, 32);
        }
    }

    #[test]
    fn test_vector_mut_at() {
        let mut v: Vector<A> = Vector::new(1);

        // THIS should not be possible
        let a = v.at(0);

        {
            let mut a = v.at_mut(0);
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
