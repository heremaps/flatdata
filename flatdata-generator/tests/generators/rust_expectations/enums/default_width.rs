#[repr(transparent)]
#[derive(Clone)]
pub struct StructEnumI8 {
    data: [u8; 1],
}

impl StructEnumI8 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for StructEnumI8 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMI8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl StructEnumI8 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since StructEnumI8 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since StructEnumI8 is repr(transparent)
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

impl Default for StructEnumI8 {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for StructEnumI8 {}

impl StructEnumI8 {
    #[inline]
    pub fn f(&self) -> super::n::EnumI8 {
        let value = flatdata_read_bytes!(i8, self.data.as_ptr(), 0, 1);
        unsafe { std::mem::transmute::<i8, super::n::EnumI8>(value) }
    }

}

impl std::fmt::Debug for StructEnumI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumI8")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for StructEnumI8 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl StructEnumI8 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::n::EnumI8) {
        flatdata_write_bytes!(i8; value, self.data, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumI8) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct StructEnumU8 {
    data: [u8; 1],
}

impl StructEnumU8 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for StructEnumU8 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMU8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl StructEnumU8 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since StructEnumU8 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since StructEnumU8 is repr(transparent)
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

impl Default for StructEnumU8 {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for StructEnumU8 {}

impl StructEnumU8 {
    #[inline]
    pub fn f(&self) -> super::n::EnumU8 {
        let value = flatdata_read_bytes!(u8, self.data.as_ptr(), 0, 1);
        unsafe { std::mem::transmute::<u8, super::n::EnumU8>(value) }
    }

}

impl std::fmt::Debug for StructEnumU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumU8")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for StructEnumU8 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl StructEnumU8 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::n::EnumU8) {
        flatdata_write_bytes!(u8; value, self.data, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumU8) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct StructEnumI16 {
    data: [u8; 1],
}

impl StructEnumI16 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for StructEnumI16 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMI16;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl StructEnumI16 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since StructEnumI16 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since StructEnumI16 is repr(transparent)
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

impl Default for StructEnumI16 {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for StructEnumI16 {}

impl StructEnumI16 {
    #[inline]
    pub fn f(&self) -> super::n::EnumI16 {
        let value = flatdata_read_bytes!(i16, self.data.as_ptr(), 0, 1);
        unsafe { std::mem::transmute::<i16, super::n::EnumI16>(value) }
    }

}

impl std::fmt::Debug for StructEnumI16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumI16")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for StructEnumI16 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl StructEnumI16 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::n::EnumI16) {
        flatdata_write_bytes!(i16; value, self.data, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumI16) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct StructEnumU16 {
    data: [u8; 1],
}

impl StructEnumU16 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for StructEnumU16 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMU16;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl StructEnumU16 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since StructEnumU16 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since StructEnumU16 is repr(transparent)
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

impl Default for StructEnumU16 {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for StructEnumU16 {}

impl StructEnumU16 {
    #[inline]
    pub fn f(&self) -> super::n::EnumU16 {
        let value = flatdata_read_bytes!(u16, self.data.as_ptr(), 0, 1);
        unsafe { std::mem::transmute::<u16, super::n::EnumU16>(value) }
    }

}

impl std::fmt::Debug for StructEnumU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumU16")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for StructEnumU16 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl StructEnumU16 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::n::EnumU16) {
        flatdata_write_bytes!(u16; value, self.data, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumU16) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct StructEnumI32 {
    data: [u8; 1],
}

impl StructEnumI32 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for StructEnumI32 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMI32;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl StructEnumI32 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since StructEnumI32 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since StructEnumI32 is repr(transparent)
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

impl Default for StructEnumI32 {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for StructEnumI32 {}

impl StructEnumI32 {
    #[inline]
    pub fn f(&self) -> super::n::EnumI32 {
        let value = flatdata_read_bytes!(i32, self.data.as_ptr(), 0, 1);
        unsafe { std::mem::transmute::<i32, super::n::EnumI32>(value) }
    }

}

impl std::fmt::Debug for StructEnumI32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumI32")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for StructEnumI32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl StructEnumI32 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::n::EnumI32) {
        flatdata_write_bytes!(i32; value, self.data, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumI32) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct StructEnumU32 {
    data: [u8; 1],
}

impl StructEnumU32 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for StructEnumU32 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMU32;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl StructEnumU32 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since StructEnumU32 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since StructEnumU32 is repr(transparent)
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

impl Default for StructEnumU32 {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for StructEnumU32 {}

impl StructEnumU32 {
    #[inline]
    pub fn f(&self) -> super::n::EnumU32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 1);
        unsafe { std::mem::transmute::<u32, super::n::EnumU32>(value) }
    }

}

impl std::fmt::Debug for StructEnumU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumU32")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for StructEnumU32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl StructEnumU32 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::n::EnumU32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumU32) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct StructEnumI64 {
    data: [u8; 1],
}

impl StructEnumI64 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for StructEnumI64 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMI64;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl StructEnumI64 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since StructEnumI64 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since StructEnumI64 is repr(transparent)
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

impl Default for StructEnumI64 {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for StructEnumI64 {}

impl StructEnumI64 {
    #[inline]
    pub fn f(&self) -> super::n::EnumI64 {
        let value = flatdata_read_bytes!(i64, self.data.as_ptr(), 0, 1);
        unsafe { std::mem::transmute::<i64, super::n::EnumI64>(value) }
    }

}

impl std::fmt::Debug for StructEnumI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumI64")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for StructEnumI64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl StructEnumI64 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::n::EnumI64) {
        flatdata_write_bytes!(i64; value, self.data, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumI64) {
        self.set_f(other.f());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct StructEnumU64 {
    data: [u8; 1],
}

impl StructEnumU64 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for StructEnumU64 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMU64;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl StructEnumU64 {
    pub fn new( ) -> Self {
        Self{data : [0; 1]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 1]) -> &Self {
        // Safety: This is safe since StructEnumU64 is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 1]) -> &mut Self {
        // Safety: This is safe since StructEnumU64 is repr(transparent)
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

impl Default for StructEnumU64 {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for StructEnumU64 {}

impl StructEnumU64 {
    #[inline]
    pub fn f(&self) -> super::n::EnumU64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 1);
        unsafe { std::mem::transmute::<u64, super::n::EnumU64>(value) }
    }

}

impl std::fmt::Debug for StructEnumU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumU64")
            .field("f", &self.f())
            .finish()
    }
}

impl std::cmp::PartialEq for StructEnumU64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl StructEnumU64 {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_f(&mut self, value: super::n::EnumU64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumU64) {
        self.set_f(other.f());
    }
}
#[derive(Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum EnumI8 {
    Value = 0,
    #[doc(hidden)]
    UnknownValueMinus1 = -1,
}

impl flatdata::helper::Int for EnumI8 {
    const IS_SIGNED: bool = true;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnumU8 {
    Value = 0,
    #[doc(hidden)]
    UnknownValue1 = 1,
}

impl flatdata::helper::Int for EnumU8 {
    const IS_SIGNED: bool = false;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum EnumI16 {
    Value = 0,
    #[doc(hidden)]
    UnknownValueMinus1 = -1,
}

impl flatdata::helper::Int for EnumI16 {
    const IS_SIGNED: bool = true;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EnumU16 {
    Value = 0,
    #[doc(hidden)]
    UnknownValue1 = 1,
}

impl flatdata::helper::Int for EnumU16 {
    const IS_SIGNED: bool = false;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum EnumI32 {
    Value = 0,
    #[doc(hidden)]
    UnknownValueMinus1 = -1,
}

impl flatdata::helper::Int for EnumI32 {
    const IS_SIGNED: bool = true;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EnumU32 {
    Value = 0,
    #[doc(hidden)]
    UnknownValue1 = 1,
}

impl flatdata::helper::Int for EnumU32 {
    const IS_SIGNED: bool = false;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum EnumI64 {
    Value = 0,
    #[doc(hidden)]
    UnknownValueMinus1 = -1,
}

impl flatdata::helper::Int for EnumI64 {
    const IS_SIGNED: bool = true;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum EnumU64 {
    Value = 0,
    #[doc(hidden)]
    UnknownValue1 = 1,
}

impl flatdata::helper::Int for EnumU64 {
    const IS_SIGNED: bool = false;
}