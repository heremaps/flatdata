#[repr(transparent)]
#[derive(Clone)]
pub struct U8 {
    data: [u8; 1],
}

impl U8 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for U8 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::U8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl U8 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since U8 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since U8 is repr(transparent)
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

impl Default for U8 {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for U8 {}

impl U8 {
    #[inline]
    pub fn f(&self) -> u8 {
        let value = flatdata_read_bytes!(u8, self.data.as_ptr(), 0, 8);
        unsafe { std::mem::transmute::<u8, u8>(value) }
    }

}

impl std::fmt::Debug for U8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("U8")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for U8 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl U8 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: u8) {
        flatdata_write_bytes!(u8; value, self.data, 0, 8)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &U8) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct I8 {
    data: [u8; 1],
}

impl I8 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for I8 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::I8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl I8 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since I8 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since I8 is repr(transparent)
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

impl Default for I8 {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for I8 {}

impl I8 {
    #[inline]
    pub fn f(&self) -> i8 {
        let value = flatdata_read_bytes!(i8, self.data.as_ptr(), 0, 8);
        unsafe { std::mem::transmute::<i8, i8>(value) }
    }

}

impl std::fmt::Debug for I8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("I8")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for I8 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl I8 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: i8) {
        flatdata_write_bytes!(i8; value, self.data, 0, 8)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &I8) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct U16 {
    data: [u8; 2],
}

impl U16 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }
}

impl flatdata::Struct for U16 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }

    const SCHEMA: &'static str = schema::structs::U16;
    const SIZE_IN_BYTES: usize = 2;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl U16 {
    pub fn new( ) -> Self {
        Self{data : [0; 2]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 2]) -> &Self {
        // Safety: This is safe since U16 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 2]) -> &mut Self {
        // Safety: This is safe since U16 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 2 {
            assert_eq!(data.len(), 2);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 2];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 2 {
            assert_eq!(data.len(), 2);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 2];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 2] {
        &self.data
    }
}

impl Default for U16 {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for U16 {}

impl U16 {
    #[inline]
    pub fn f(&self) -> u16 {
        let value = flatdata_read_bytes!(u16, self.data.as_ptr(), 0, 16);
        unsafe { std::mem::transmute::<u16, u16>(value) }
    }

}

impl std::fmt::Debug for U16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("U16")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for U16 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl U16 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: u16) {
        flatdata_write_bytes!(u16; value, self.data, 0, 16)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &U16) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct I16 {
    data: [u8; 2],
}

impl I16 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }
}

impl flatdata::Struct for I16 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }

    const SCHEMA: &'static str = schema::structs::I16;
    const SIZE_IN_BYTES: usize = 2;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl I16 {
    pub fn new( ) -> Self {
        Self{data : [0; 2]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 2]) -> &Self {
        // Safety: This is safe since I16 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 2]) -> &mut Self {
        // Safety: This is safe since I16 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 2 {
            assert_eq!(data.len(), 2);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 2];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 2 {
            assert_eq!(data.len(), 2);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 2];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 2] {
        &self.data
    }
}

impl Default for I16 {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for I16 {}

impl I16 {
    #[inline]
    pub fn f(&self) -> i16 {
        let value = flatdata_read_bytes!(i16, self.data.as_ptr(), 0, 16);
        unsafe { std::mem::transmute::<i16, i16>(value) }
    }

}

impl std::fmt::Debug for I16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("I16")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for I16 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl I16 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: i16) {
        flatdata_write_bytes!(i16; value, self.data, 0, 16)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &I16) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct U32 {
    data: [u8; 4],
}

impl U32 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }
}

impl flatdata::Struct for U32 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }

    const SCHEMA: &'static str = schema::structs::U32;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl U32 {
    pub fn new( ) -> Self {
        Self{data : [0; 4]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 4]) -> &Self {
        // Safety: This is safe since U32 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 4]) -> &mut Self {
        // Safety: This is safe since U32 is repr(transparent)
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

impl Default for U32 {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for U32 {}

impl U32 {
    #[inline]
    pub fn f(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

}

impl std::fmt::Debug for U32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("U32")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for U32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl U32 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &U32) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct I32 {
    data: [u8; 4],
}

impl I32 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }
}

impl flatdata::Struct for I32 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }

    const SCHEMA: &'static str = schema::structs::I32;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl I32 {
    pub fn new( ) -> Self {
        Self{data : [0; 4]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 4]) -> &Self {
        // Safety: This is safe since I32 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 4]) -> &mut Self {
        // Safety: This is safe since I32 is repr(transparent)
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

impl Default for I32 {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for I32 {}

impl I32 {
    #[inline]
    pub fn f(&self) -> i32 {
        let value = flatdata_read_bytes!(i32, self.data.as_ptr(), 0, 32);
        unsafe { std::mem::transmute::<i32, i32>(value) }
    }

}

impl std::fmt::Debug for I32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("I32")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for I32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl I32 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: i32) {
        flatdata_write_bytes!(i32; value, self.data, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &I32) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct U64 {
    data: [u8; 8],
}

impl U64 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }
}

impl flatdata::Struct for U64 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }

    const SCHEMA: &'static str = schema::structs::U64;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl U64 {
    pub fn new( ) -> Self {
        Self{data : [0; 8]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 8]) -> &Self {
        // Safety: This is safe since U64 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 8]) -> &mut Self {
        // Safety: This is safe since U64 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 8 {
            assert_eq!(data.len(), 8);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 8];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 8 {
            assert_eq!(data.len(), 8);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 8];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.data
    }
}

impl Default for U64 {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for U64 {}

impl U64 {
    #[inline]
    pub fn f(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl std::fmt::Debug for U64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("U64")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for U64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl U64 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &U64) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct I64 {
    data: [u8; 8],
}

impl I64 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }
}

impl flatdata::Struct for I64 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }

    const SCHEMA: &'static str = schema::structs::I64;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl I64 {
    pub fn new( ) -> Self {
        Self{data : [0; 8]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 8]) -> &Self {
        // Safety: This is safe since I64 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 8]) -> &mut Self {
        // Safety: This is safe since I64 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array
    pub fn from_bytes_slice(data: &[u8]) -> Result<&Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 8 {
            assert_eq!(data.len(), 8);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *const [u8; 8];
        // Safety: We checked length before
        Ok(Self::from_bytes(unsafe { &*ptr }))
    }

    /// Create reference from byte array
    pub fn from_bytes_slice_mut(data: &mut [u8]) -> Result<&mut Self, flatdata::ResourceStorageError> {
        // We cannot rely on TryFrom here, since it does not yet support > 33 bytes
        if data.len() < 8 {
            assert_eq!(data.len(), 8);
            return Err(flatdata::ResourceStorageError::UnexpectedDataSize);
        }
        let ptr = data.as_ptr() as *mut [u8; 8];
        // Safety: We checked length before
        Ok(Self::from_bytes_mut(unsafe { &mut *ptr }))
    }

    pub fn as_bytes(&self) -> &[u8; 8] {
        &self.data
    }
}

impl Default for I64 {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for I64 {}

impl I64 {
    #[inline]
    pub fn f(&self) -> i64 {
        let value = flatdata_read_bytes!(i64, self.data.as_ptr(), 0, 64);
        unsafe { std::mem::transmute::<i64, i64>(value) }
    }

}

impl std::fmt::Debug for I64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("I64")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for I64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl I64 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: i64) {
        flatdata_write_bytes!(i64; value, self.data, 0, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &I64) {
        self.set_f(other.f());
    }
}
}
