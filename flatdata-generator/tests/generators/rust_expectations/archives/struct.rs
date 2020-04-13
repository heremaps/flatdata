#[repr(transparent)]
#[derive(Clone)]
pub struct S {
    data: [u8; 8],
}

impl S {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }
}

impl flatdata::Struct for S {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }

    const SCHEMA: &'static str = schema::structs::S;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl S {
    pub fn new( ) -> Self {
        Self{data : [0; 8]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 8]) -> &Self {
        // Safety: This is safe since S is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 8]) -> &mut Self {
        // Safety: This is safe since S is repr(transparent)
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

impl Default for S {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for S {}

impl S {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl std::fmt::Debug for S {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("S")
            .field("x", &self.x())
            .finish()
    }
}

impl std::cmp::PartialEq for S {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()     }
}

impl S {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_x(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &S) {
        self.set_x(other.x());
    }
}



#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    data : &'static super::n::S,
    optional_data : Option<&'static super::n::S>,
}

impl A {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> &super::n::S {
        self.data
    }

    #[inline]
    pub fn optional_data(&self) -> Option<&super::n::S> {
        self.optional_data
    }

}

impl ::std::fmt::Debug for A {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("A")
            .field("data", &self.data())
            .field("optional_data", &self.optional_data())
            .finish()
    }
}

impl flatdata::Archive for A {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use flatdata::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], flatdata::ResourceStorageError>| -> Result<&'static [u8], flatdata::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let resource = extend(storage.read("data", schema::a::resources::DATA));
        let data = resource.map(|x| super::n::S::from_bytes_slice(x))??;
        let resource = extend(storage.read("optional_data", schema::a::resources::OPTIONAL_DATA));
        let optional_data = resource.map(|x| super::n::S::from_bytes_slice(x)).ok().transpose()?;

        Ok(Self {
            _storage: storage,
            data,
            optional_data,
        })
    }
}

/// Builder for creating [`A`] archives.
///
///[`A`]: struct.A.html
#[derive(Clone, Debug)]
pub struct ABuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl ABuilder {
    #[inline]
    /// Stores [`data`] in the archive.
    ///
    /// [`data`]: struct.A.html#method.data
    /// Stores [`data`] in the archive.
    pub fn set_data(&self, resource: &super::n::S) -> ::std::io::Result<()> {
        let data = resource.as_bytes();
        self.storage.write("data", schema::a::resources::DATA, data)
    }

    #[inline]
    /// Stores [`optional_data`] in the archive.
    ///
    /// [`optional_data`]: struct.A.html#method.optional_data
    /// Stores [`optional_data`] in the archive.
    pub fn set_optional_data(&self, resource: &super::n::S) -> ::std::io::Result<()> {
        let data = resource.as_bytes();
        self.storage.write("optional_data", schema::a::resources::OPTIONAL_DATA, data)
    }

}

impl flatdata::ArchiveBuilder for ABuilder {
    const NAME: &'static str = "A";
    const SCHEMA: &'static str = schema::a::A;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}