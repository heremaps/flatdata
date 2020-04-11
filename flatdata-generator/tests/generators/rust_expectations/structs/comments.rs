// This is a comment about Foo
#[repr(transparent)]
#[derive(Clone)]
pub struct Foo {
    data: [u8; 16],
}

impl Foo {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 16]}
    }
}

impl flatdata::Struct for Foo {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 16]}
    }

    const SCHEMA: &'static str = schema::structs::FOO;
    const SIZE_IN_BYTES: usize = 16;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl Foo {
    pub fn new( ) -> Self {
        Self{data : [0; 16]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 16]) -> &Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 16]) -> &mut Self {
        // Safety: This is safe since Foo is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 16 {
            assert_eq!(data.len(), 16);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 16];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 16 {
            assert_eq!(data.len(), 16);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 16];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
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
    // This is a comment about Foo.a
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    // This is a comment about Foo.b
    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl std::fmt::Debug for Foo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Foo")
            .field("a", &self.a())
            .field("b", &self.b())
            .finish()
    }
}

impl std::cmp::PartialEq for Foo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() &&        self.b() == other.b()     }
}

impl Foo {
    // This is a comment about Foo.a
    #[inline]
    #[allow(missing_docs)]
    pub fn set_a(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 64)
    }

    // This is a comment about Foo.b
    #[inline]
    #[allow(missing_docs)]
    pub fn set_b(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 64, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &Foo) {
        self.set_a(other.a());
        self.set_b(other.b());
    }
}
/// This is a comment about Bar
#[repr(transparent)]
#[derive(Clone)]
pub struct Bar {
    data: [u8; 16],
}

impl Bar {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 16]}
    }
}

impl flatdata::Struct for Bar {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 16]}
    }

    const SCHEMA: &'static str = schema::structs::BAR;
    const SIZE_IN_BYTES: usize = 16;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl Bar {
    pub fn new( ) -> Self {
        Self{data : [0; 16]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 16]) -> &Self {
        // Safety: This is safe since Bar is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 16]) -> &mut Self {
        // Safety: This is safe since Bar is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 16 {
            assert_eq!(data.len(), 16);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 16];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 16 {
            assert_eq!(data.len(), 16);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 16];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.data
    }
}

impl Default for Bar {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for Bar {}

impl Bar {
    /// This is a comment about Bar.a
    #[inline]
    pub fn a(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    /// This is a comment about Bar.b
    #[inline]
    pub fn b(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 64, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl std::fmt::Debug for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Bar")
            .field("a", &self.a())
            .field("b", &self.b())
            .finish()
    }
}

impl std::cmp::PartialEq for Bar {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.a() == other.a() &&        self.b() == other.b()     }
}

impl Bar {
    /// This is a comment about Bar.a
    #[inline]
    #[allow(missing_docs)]
    pub fn set_a(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 64)
    }

    /// This is a comment about Bar.b
    #[inline]
    #[allow(missing_docs)]
    pub fn set_b(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 64, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &Bar) {
        self.set_a(other.a());
        self.set_b(other.b());
    }
}
}
