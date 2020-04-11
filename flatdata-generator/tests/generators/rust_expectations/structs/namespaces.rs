#[repr(transparent)]
#[derive(Clone)]
pub struct Foo {
    data: [u8; 4],
}

impl Foo {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }
}

impl flatdata::Struct for Foo {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }

    const SCHEMA: &'static str = schema::structs::FOO;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl Foo {
    pub fn new( ) -> Self {
        Self{data : [0; 4]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 4]) -> &Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 4]) -> &mut Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 4 {
            assert_eq!(data.len(), 4);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 4];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 4 {
            assert_eq!(data.len(), 4);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 4];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 4] {
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
    pub fn f(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
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
    pub fn set_f(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &Foo) {
        self.set_f(other.f());
    }
}
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[allow(missing_docs)]
pub mod m {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const FOO: &str = r#"namespace m {
struct Foo
{
    f : u32 : 32;
}
}

"#;
}

}#[repr(transparent)]
#[derive(Clone)]
pub struct Foo {
    data: [u8; 4],
}

impl Foo {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }
}

impl flatdata::Struct for Foo {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }

    const SCHEMA: &'static str = schema::structs::FOO;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl Foo {
    pub fn new( ) -> Self {
        Self{data : [0; 4]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 4]) -> &Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 4]) -> &mut Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 4 {
            assert_eq!(data.len(), 4);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 4];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 4 {
            assert_eq!(data.len(), 4);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 4];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 4] {
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
    pub fn f(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
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
    pub fn set_f(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &Foo) {
        self.set_f(other.f());
    }
}
}
