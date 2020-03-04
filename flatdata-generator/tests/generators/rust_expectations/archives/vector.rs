/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`SRef`] for the read-only access, and
/// * [`SMut`] for the mutable access
///
/// to the `S` data.
///
/// [`SRef`]: struct.SRef.html
/// [`SMut`]: struct.SMut.html
#[derive(Clone, Debug)]
pub struct S {}

/// Read-only access to [`S`].
///
/// [`S`]: struct.S.html
#[derive(Clone, Copy)]
pub struct SRef<'a> {
    pub(crate) data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for S
{
    const SCHEMA: &'static str = schema::structs::S;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = SRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = SMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for S {}

impl<'a> SRef<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl<'a> std::fmt::Debug for SRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("S")
            .field("x", &self.x())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for SRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()     }
}

impl<'a> flatdata::Ref for SRef<'a> {}

/// Mutable access to [`S`].
///
/// [`S`]: struct.S.html
pub struct SMut<'a> {
    pub(crate) data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> SMut<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_x(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &SRef) {
        self.set_x(other.x());
    }
}

impl<'a> std::fmt::Debug for SMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        SRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for SMut<'a> {}



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
    pub fn data(&self) -> flatdata::ArrayView<super::n::S>
    {
        flatdata::ArrayView::new(&unsafe {self.data.as_bytes()})
    }

    #[inline]
    pub fn optional_data(&self) -> Option<flatdata::ArrayView<super::n::S>>
    {
        self.optional_data.as_ref().map(|x|flatdata::ArrayView::new(unsafe {x.as_bytes()}))
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
    #[inline]
    /// Stores [`data`] in the archive.
    ///
    /// [`data`]: struct.A.html#method.data
    pub fn set_data(&self, vector: &flatdata::ArrayView<super::n::S>) -> ::std::io::Result<()> {
        self.storage.write("data", schema::a::resources::DATA, vector.as_ref())
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

    #[inline]
    /// Stores [`optional_data`] in the archive.
    ///
    /// [`optional_data`]: struct.A.html#method.optional_data
    pub fn set_optional_data(&self, vector: &flatdata::ArrayView<super::n::S>) -> ::std::io::Result<()> {
        self.storage.write("optional_data", schema::a::resources::OPTIONAL_DATA, vector.as_ref())
    }

    /// Opens [`optional_data`] in the archive for buffered writing.
    ///
    /// Elements can be added to the vector until the [`ExternalVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`optional_data`]: struct.A.html#method.optional_data
    /// [`ExternalVector::close`]: flatdata/struct.ExternalVector.html#method.close
    #[inline]
    pub fn start_optional_data(&self) -> ::std::io::Result<flatdata::ExternalVector<super::n::S>> {
        flatdata::create_external_vector(&*self.storage, "optional_data", schema::a::resources::OPTIONAL_DATA)
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
