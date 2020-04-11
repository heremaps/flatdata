#[derive(Clone)]
pub struct X {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    payload : flatdata::RawData<'static>,
}

impl X {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn payload(&self) -> flatdata::RawData {
        self.payload
    }

}

impl ::std::fmt::Debug for X {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("X")
            .field("payload", &self.payload())
            .finish()
    }
}

impl flatdata::Archive for X {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn open(storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>)
        -> ::std::result::Result<Self, flatdata::ResourceStorageError>
    {
        #[allow(unused_imports)]
        use flatdata::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], flatdata::ResourceStorageError>| -> Result<&'static [u8], flatdata::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let resource = extend(storage.read("payload", schema::x::resources::PAYLOAD));
        let payload = resource.map(|x| flatdata::RawData::new(x))?;

        Ok(Self {
            _storage: storage,
            payload,
        })
    }
}

/// Builder for creating [`X`] archives.
///
///[`X`]: struct.X.html
#[derive(Clone, Debug)]
pub struct XBuilder {
    storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>
}

impl XBuilder {
    /// Stores [`payload`] in the archive.
    ///
    /// [`payload`]: struct.X.html#method.payload
    #[inline]
    pub fn set_payload(&self, data: &[u8]) -> ::std::io::Result<()> {
        self.storage.write("payload", schema::x::resources::PAYLOAD, data)
    }

}

impl flatdata::ArchiveBuilder for XBuilder {
    const NAME: &'static str = "X";
    const SCHEMA: &'static str = schema::x::X;

    fn new(
        storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    ) -> Result<Self, flatdata::ResourceStorageError> {
        flatdata::create_archive::<Self>(&storage)?;
        Ok(Self { storage })
    }
}




#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    data : super::n::X
,
    optional_data : Option<super::n::X
>,
}

impl A {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> &super::n::X {
        &self.data
    }

    #[inline]
    pub fn optional_data(&self) -> Option<&super::n::X> {
        self.optional_data.as_ref()
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

        let data = super::n::X::open(storage.subdir("data"))?;
        let optional_data = super::n::X::open(storage.subdir("optional_data")).ok();

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
    /// Stores [`data`] in the archive.
    ///
    /// [`data`]: struct.A.html#method.data
    #[inline]
    pub fn data(&self) -> Result<super::n::XBuilder, flatdata::ResourceStorageError> {
        use flatdata::ArchiveBuilder;
        let storage = self.storage.subdir("data");
        super::n::XBuilder::new(storage)
    }

    /// Stores [`optional_data`] in the archive.
    ///
    /// [`optional_data`]: struct.A.html#method.optional_data
    #[inline]
    pub fn optional_data(&self) -> Result<super::n::XBuilder, flatdata::ResourceStorageError> {
        use flatdata::ArchiveBuilder;
        let storage = self.storage.subdir("optional_data");
        super::n::XBuilder::new(storage)
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