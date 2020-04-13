}#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bar {
    Value = 0,
}

impl flatdata::helper::Int for Bar {
    const IS_SIGNED: bool = false;
}
}

#[allow(missing_docs)]
pub mod b {

#[doc(hidden)]
pub mod schema {
pub mod structs {
}

}#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bar {
    Value = 0,
}

impl flatdata::helper::Int for Bar {
    const IS_SIGNED: bool = false;
}
}

#[allow(missing_docs)]
pub mod n {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const FOO: &str = r#"namespace a {
enum Bar : u8
{
    VALUE = 0,
}
}

namespace n {
struct Foo
{
    f : .a.Bar : 8;
}
}

"#;
}

}#[repr(transparent)]
#[derive(Clone)]
pub struct Foo {
    data: [u8; 1],
}

impl Foo {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for Foo {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::FOO;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl Foo {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 1 {
            assert_eq!(data.len(), 1);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 1];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 1 {
            assert_eq!(data.len(), 1);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 1];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 1] {
        &self.data
    }
}

impl Default for Foo {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for Foo {}

impl Foo {
    #[inline]
    pub fn f(&self) -> super::a::Bar {
        let value = flatdata_read_bytes!(u8, self.data.as_ptr(), 0, 8);
        unsafe { std::mem::transmute::<u8, super::a::Bar>(value) }
    }

}

impl std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Foo")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for Foo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl Foo {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::a::Bar) {
        flatdata_write_bytes!(u8; value, self.data, 0, 8)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &Foo) {
        self.set_f(other.f());
    }
}
}

#[allow(missing_docs)]
pub mod m {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const FOO: &str = r#"namespace b {
enum Bar : u8
{
    VALUE = 0,
}
}

namespace m {
struct Foo
{
    f : .b.Bar : 8;
}
}

"#;
}

}#[repr(transparent)]
#[derive(Clone)]
pub struct Foo {
    data: [u8; 1],
}

impl Foo {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for Foo {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::FOO;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl Foo {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 1 {
            assert_eq!(data.len(), 1);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 1];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 1 {
            assert_eq!(data.len(), 1);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 1];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 1] {
        &self.data
    }
}

impl Default for Foo {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for Foo {}

impl Foo {
    #[inline]
    pub fn f(&self) -> super::b::Bar {
        let value = flatdata_read_bytes!(u8, self.data.as_ptr(), 0, 8);
        unsafe { std::mem::transmute::<u8, super::b::Bar>(value) }
    }

}

impl std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Foo")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for Foo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl Foo {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::b::Bar) {
        flatdata_write_bytes!(u8; value, self.data, 0, 8)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &Foo) {
        self.set_f(other.f());
    }
}
}
