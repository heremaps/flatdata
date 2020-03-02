#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    data: flatdata::MemoryDescriptor,
    optional_data: Option<flatdata::MemoryDescriptor>,
}

impl A {
    fn read_resource(
        storage: &dyn flatdata::ResourceStorage,
        name: &str,
        schema: &str,
    ) -> Result<flatdata::MemoryDescriptor, flatdata::ResourceStorageError>
    {
        storage.read(name, schema).map(|x| flatdata::MemoryDescriptor::new(&x))
    }

    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> flatdata::RawData {
        flatdata::RawData::new(unsafe {self.data.as_bytes()})
    }

    #[inline]
    pub fn optional_data(&self) -> Option<flatdata::RawData> {
        self.optional_data.as_ref().map(|mem_desc| flatdata::RawData::new({unsafe {mem_desc.as_bytes()} }))
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
        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let data = Self::read_resource(&*storage, "data", schema::a::resources::DATA)?;
        let optional_data = Self::read_resource(&*storage, "optional_data", schema::a::resources::OPTIONAL_DATA).ok();

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
    pub fn set_data(&self, data: &[u8]) -> ::std::io::Result<()> {
        self.storage.write("data", schema::a::resources::DATA, data)
    }

    /// Stores [`optional_data`] in the archive.
    ///
    /// [`optional_data`]: struct.A.html#method.optional_data
    #[inline]
    pub fn set_optional_data(&self, data: &[u8]) -> ::std::io::Result<()> {
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