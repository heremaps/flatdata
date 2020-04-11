#[repr(transparent)]
pub struct S {
    data: [u8; 10],
}

impl S {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 10]}
    }
}

impl flatdata::Struct for S {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 10]}
    }

    const SCHEMA: &'static str = schema::structs::S;
    const SIZE_IN_BYTES: usize = 10;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;
}


impl S {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    /// First element of the range [`y_range`].
    ///
    /// [`y_range`]: #method.y_range
    #[inline]
    pub fn first_y(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 64, 14);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[inline]
    pub fn y_range(&self) -> std::ops::Range<u32> {
        let start = flatdata_read_bytes!(u32, self.data.as_ptr(), 64, 14);
        let end = flatdata_read_bytes!(u32, self.data.as_ptr(), 64 + 10 * 8, 14);
        start..end
    }

}

impl std::fmt::Debug for S {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("S")
            .field("x", &self.x())
            .field("first_y", &self.first_y())
            .finish()
    }
}

impl std::cmp::PartialEq for S {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x() &&        self.first_y() == other.first_y()     }
}

impl S {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_x(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 64)
    }

    /// First element of the range [`y_range`].
    ///
    /// [`y_range`]: struct.SRef.html#method.y_range
    #[inline]
    #[allow(missing_docs)]
    pub fn set_first_y(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 64, 14)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &S) {
        self.set_x(other.x());
        self.set_first_y(other.first_y());
    }
}



#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    data : &'static [super::n::S],
}

impl A {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> &[super::n::S] {
        self.data
    }

}

impl ::std::fmt::Debug for A {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("A")
            .field("data", &self.data())
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
        let data = resource.map(|x| <&[super::n::S]>::from_bytes(x))??;

        Ok(Self {
            _storage: storage,
            data,
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
    pub fn set_data(&self, vector: &[super::n::S]) -> ::std::io::Result<()> {
        use flatdata::SliceExt;
        self.storage.write("data", schema::a::resources::DATA, vector.as_bytes())
    }

    /// Opens [`data`] in the archive for buffered writing.
    ///
    /// Elements can be added to the vector until the [`ExternalVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`data`]: struct.A.html#method.data
    /// [`ExternalVector::close`]: flatdata/struct.ExternalVector.html#method.close
    #[inline]
    pub fn start_data(&self) -> ::std::io::Result<flatdata::ExternalVector<super::n::S>> {
        flatdata::create_external_vector(&*self.storage, "data", schema::a::resources::DATA)
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