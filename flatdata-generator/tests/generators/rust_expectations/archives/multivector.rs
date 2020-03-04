///
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

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`TRef`] for the read-only access, and
/// * [`TMut`] for the mutable access
///
/// to the `T` data.
///
/// [`TRef`]: struct.TRef.html
/// [`TMut`]: struct.TMut.html
#[derive(Clone, Debug)]
pub struct T {}

/// Read-only access to [`T`].
///
/// [`T`]: struct.T.html
#[derive(Clone, Copy)]
pub struct TRef<'a> {
    pub(crate) data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for T
{
    const SCHEMA: &'static str = schema::structs::T;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = TRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = TMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for T {}

impl<'a> TRef<'a> {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl<'a> std::fmt::Debug for TRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("T")
            .field("x", &self.x())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for TRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()     }
}

impl<'a> flatdata::Ref for TRef<'a> {}

/// Mutable access to [`T`].
///
/// [`T`]: struct.T.html
pub struct TMut<'a> {
    pub(crate) data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> TMut<'a> {
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
    pub fn fill_from(&mut self, other: &TRef) {
        self.set_x(other.x());
    }
}

impl<'a> std::fmt::Debug for TMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        TRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for TMut<'a> {}


/// Enum for read-only heterogeneous access to elements in a
/// bucket of the [`data`] resource.
///
/// [`data`]: struct.Archive{.n.A}.html#method.data
#[derive(Clone, PartialEq)]
pub enum DataRef<'a> {
    #[allow(missing_docs)]
    S(<super::n::S as flatdata::Struct<'a>>::Item),    #[allow(missing_docs)]
    T(<super::n::T as flatdata::Struct<'a>>::Item),}

impl<'a> ::std::fmt::Debug for DataRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DataRef::S(ref inner) => write!(f, "{:?}", inner),
            DataRef::T(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> flatdata::VariadicRef for DataRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            DataRef::S(_) => <super::n::S as flatdata::Struct<'a>>::SIZE_IN_BYTES,
            DataRef::T(_) => <super::n::T as flatdata::Struct<'a>>::SIZE_IN_BYTES,
        }
    }
}

/// Builder of buckets in the [`data`] resource.
///
/// Refers to a single bucket in the [`data`] multivector and
/// provides methods for adding heterogeneous data to the bucket.
///
/// [`data`]: struct.Archive{.n.A}.html#method.data
pub struct DataBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> DataBuilder<'a> {
    /// Adds data of the type [`S`] to the bucket.
    ///
    /// [`S`]: struct.S.html
    #[inline]
    pub fn add_s<'b>(&'b mut self) -> <super::n::S as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 0;
        <super::n::S as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
    /// Adds data of the type [`T`] to the bucket.
    ///
    /// [`T`]: struct.T.html
    #[inline]
    pub fn add_t<'b>(&'b mut self) -> <super::n::T as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::T as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 1;
        <super::n::T as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
}

/// Variadic struct attached to the [`data`] archive resource.
///
/// It unifies the following data types:
//
/// * [`S`]
/// * [`T`]
///
/// ## Access pattern
///
/// This structure is used as a template parameter in [`data`] multivector/
/// multiarray view. It does not contain any data, instead it references
///
/// * [`DataRef`] for the read-only heterogeneous access, and
/// * [`DataBuilder`] for the mutable builder pattern access.
///
/// [`data`]: struct.Archive{.n.A}.html#method.data
/// [`DataRef`]: enum.DataRef.html
/// [`DataBuilder`]: struct.DataBuilder.html
/// [`S`]: struct.S.html
/// [`T`]: struct.T.html
#[derive(Clone)]
pub struct Data {}

impl<'a> flatdata::VariadicStruct<'a> for Data {
    type Index = super::_builtin::multivector::IndexType8;

    type Item = DataRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => DataRef::S(<super::n::S as flatdata::Struct<'a>>::create(data)),
                1 => DataRef::T(<super::n::T as flatdata::Struct<'a>>::create(data)),
            _ => panic!("invalid type index {} for variadic type DataRef", index),
        }
    }

    type ItemMut = DataBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}
/// Enum for read-only heterogeneous access to elements in a
/// bucket of the [`optional_data`] resource.
///
/// [`optional_data`]: struct.Archive{.n.A}.html#method.optional_data
#[derive(Clone, PartialEq)]
pub enum OptionalDataRef<'a> {
    #[allow(missing_docs)]
    S(<super::n::S as flatdata::Struct<'a>>::Item),    #[allow(missing_docs)]
    T(<super::n::T as flatdata::Struct<'a>>::Item),}

impl<'a> ::std::fmt::Debug for OptionalDataRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            OptionalDataRef::S(ref inner) => write!(f, "{:?}", inner),
            OptionalDataRef::T(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> flatdata::VariadicRef for OptionalDataRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            OptionalDataRef::S(_) => <super::n::S as flatdata::Struct<'a>>::SIZE_IN_BYTES,
            OptionalDataRef::T(_) => <super::n::T as flatdata::Struct<'a>>::SIZE_IN_BYTES,
        }
    }
}

/// Builder of buckets in the [`optional_data`] resource.
///
/// Refers to a single bucket in the [`optional_data`] multivector and
/// provides methods for adding heterogeneous data to the bucket.
///
/// [`optional_data`]: struct.Archive{.n.A}.html#method.optional_data
pub struct OptionalDataBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> OptionalDataBuilder<'a> {
    /// Adds data of the type [`S`] to the bucket.
    ///
    /// [`S`]: struct.S.html
    #[inline]
    pub fn add_s<'b>(&'b mut self) -> <super::n::S as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 0;
        <super::n::S as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
    /// Adds data of the type [`T`] to the bucket.
    ///
    /// [`T`]: struct.T.html
    #[inline]
    pub fn add_t<'b>(&'b mut self) -> <super::n::T as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::T as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 1;
        <super::n::T as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
}

/// Variadic struct attached to the [`optional_data`] archive resource.
///
/// It unifies the following data types:
//
/// * [`S`]
/// * [`T`]
///
/// ## Access pattern
///
/// This structure is used as a template parameter in [`optional_data`] multivector/
/// multiarray view. It does not contain any data, instead it references
///
/// * [`OptionalDataRef`] for the read-only heterogeneous access, and
/// * [`OptionalDataBuilder`] for the mutable builder pattern access.
///
/// [`optional_data`]: struct.Archive{.n.A}.html#method.optional_data
/// [`OptionalDataRef`]: enum.OptionalDataRef.html
/// [`OptionalDataBuilder`]: struct.OptionalDataBuilder.html
/// [`S`]: struct.S.html
/// [`T`]: struct.T.html
#[derive(Clone)]
pub struct OptionalData {}

impl<'a> flatdata::VariadicStruct<'a> for OptionalData {
    type Index = super::_builtin::multivector::IndexType16;

    type Item = OptionalDataRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => OptionalDataRef::S(<super::n::S as flatdata::Struct<'a>>::create(data)),
                1 => OptionalDataRef::T(<super::n::T as flatdata::Struct<'a>>::create(data)),
            _ => panic!("invalid type index {} for variadic type OptionalDataRef", index),
        }
    }

    type ItemMut = OptionalDataBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}
/// Enum for read-only heterogeneous access to elements in a
/// bucket of the [`data_u64_index`] resource.
///
/// [`data_u64_index`]: struct.Archive{.n.A}.html#method.data_u64_index
#[derive(Clone, PartialEq)]
pub enum DataU64IndexRef<'a> {
    #[allow(missing_docs)]
    S(<super::n::S as flatdata::Struct<'a>>::Item),    #[allow(missing_docs)]
    T(<super::n::T as flatdata::Struct<'a>>::Item),}

impl<'a> ::std::fmt::Debug for DataU64IndexRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            DataU64IndexRef::S(ref inner) => write!(f, "{:?}", inner),
            DataU64IndexRef::T(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> flatdata::VariadicRef for DataU64IndexRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            DataU64IndexRef::S(_) => <super::n::S as flatdata::Struct<'a>>::SIZE_IN_BYTES,
            DataU64IndexRef::T(_) => <super::n::T as flatdata::Struct<'a>>::SIZE_IN_BYTES,
        }
    }
}

/// Builder of buckets in the [`data_u64_index`] resource.
///
/// Refers to a single bucket in the [`data_u64_index`] multivector and
/// provides methods for adding heterogeneous data to the bucket.
///
/// [`data_u64_index`]: struct.Archive{.n.A}.html#method.data_u64_index
pub struct DataU64IndexBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> DataU64IndexBuilder<'a> {
    /// Adds data of the type [`S`] to the bucket.
    ///
    /// [`S`]: struct.S.html
    #[inline]
    pub fn add_s<'b>(&'b mut self) -> <super::n::S as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 0;
        <super::n::S as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
    /// Adds data of the type [`T`] to the bucket.
    ///
    /// [`T`]: struct.T.html
    #[inline]
    pub fn add_t<'b>(&'b mut self) -> <super::n::T as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::T as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 1;
        <super::n::T as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
    }
}

/// Variadic struct attached to the [`data_u64_index`] archive resource.
///
/// It unifies the following data types:
//
/// * [`S`]
/// * [`T`]
///
/// ## Access pattern
///
/// This structure is used as a template parameter in [`data_u64_index`] multivector/
/// multiarray view. It does not contain any data, instead it references
///
/// * [`DataU64IndexRef`] for the read-only heterogeneous access, and
/// * [`DataU64IndexBuilder`] for the mutable builder pattern access.
///
/// [`data_u64_index`]: struct.Archive{.n.A}.html#method.data_u64_index
/// [`DataU64IndexRef`]: enum.DataU64IndexRef.html
/// [`DataU64IndexBuilder`]: struct.DataU64IndexBuilder.html
/// [`S`]: struct.S.html
/// [`T`]: struct.T.html
#[derive(Clone)]
pub struct DataU64Index {}

impl<'a> flatdata::VariadicStruct<'a> for DataU64Index {
    type Index = super::_builtin::multivector::IndexType64;

    type Item = DataU64IndexRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => DataU64IndexRef::S(<super::n::S as flatdata::Struct<'a>>::create(data)),
                1 => DataU64IndexRef::T(<super::n::T as flatdata::Struct<'a>>::create(data)),
            _ => panic!("invalid type index {} for variadic type DataU64IndexRef", index),
        }
    }

    type ItemMut = DataU64IndexBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}

#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    data: (flatdata::MemoryDescriptor, flatdata::MemoryDescriptor),
    optional_data: Option<(flatdata::MemoryDescriptor, flatdata::MemoryDescriptor)>,
    data_u64_index: (flatdata::MemoryDescriptor, flatdata::MemoryDescriptor),
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
    pub fn data(&self) -> flatdata::MultiArrayView<Data>
    {
        flatdata::MultiArrayView::new(
            flatdata::ArrayView::new(&unsafe {self.data.0.as_bytes()}),
            &unsafe {self.data.1.as_bytes()},
        )
    }

    #[inline]
    pub fn optional_data(&self) -> Option<flatdata::MultiArrayView<OptionalData>>
    {
        self.optional_data.as_ref()
            .map(|(index, data)|{
                flatdata::MultiArrayView::new(flatdata::ArrayView::new(unsafe {index.as_bytes()}), unsafe {data.as_bytes()})
            })
    }

    #[inline]
    pub fn data_u64_index(&self) -> flatdata::MultiArrayView<DataU64Index>
    {
        flatdata::MultiArrayView::new(
            flatdata::ArrayView::new(&unsafe {self.data_u64_index.0.as_bytes()}),
            &unsafe {self.data_u64_index.1.as_bytes()},
        )
    }

}

impl ::std::fmt::Debug for A {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("A")
            .field("data", &self.data())
            .field("optional_data", &self.optional_data())
            .field("data_u64_index", &self.data_u64_index())
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

        let data = {
            let index_schema = &format!("index({})", schema::a::resources::DATA);
            let index = Self::read_resource(&*storage, "data_index", &index_schema)?;
            let data = Self::read_resource(&*storage, "data", schema::a::resources::DATA)?;            (index, data)
        };
        let optional_data = {
            let index_schema = &format!("index({})", schema::a::resources::OPTIONAL_DATA);
            let index = Self::read_resource(&*storage, "optional_data_index", &index_schema).ok();
            let data = Self::read_resource(&*storage, "optional_data", schema::a::resources::OPTIONAL_DATA).ok();            match (index, data) {
                (Some(index), Some(data)) => Some((index, data)),
                _ => None,
            }        };
        let data_u64_index = {
            let index_schema = &format!("index({})", schema::a::resources::DATA_U64_INDEX);
            let index = Self::read_resource(&*storage, "data_u64_index_index", &index_schema)?;
            let data = Self::read_resource(&*storage, "data_u64_index", schema::a::resources::DATA_U64_INDEX)?;            (index, data)
        };

        Ok(Self {
            _storage: storage,
            data,
            optional_data,
            data_u64_index,
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
    /// Opens [`data`] in the archive for buffered writing.
    ///
    /// Elements can be added to the multivector until the [`MultiVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`data`]: struct.A.html#method.data
    /// [`MultiVector::close`]: flatdata/struct.MultiVector.html#method.close
    #[inline]
    pub fn start_data(&self) -> ::std::io::Result<flatdata::MultiVector<Data>> {
        flatdata::create_multi_vector(&*self.storage, "data", schema::a::resources::DATA)
    }

    /// Opens [`optional_data`] in the archive for buffered writing.
    ///
    /// Elements can be added to the multivector until the [`MultiVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`optional_data`]: struct.A.html#method.optional_data
    /// [`MultiVector::close`]: flatdata/struct.MultiVector.html#method.close
    #[inline]
    pub fn start_optional_data(&self) -> ::std::io::Result<flatdata::MultiVector<OptionalData>> {
        flatdata::create_multi_vector(&*self.storage, "optional_data", schema::a::resources::OPTIONAL_DATA)
    }

    /// Opens [`data_u64_index`] in the archive for buffered writing.
    ///
    /// Elements can be added to the multivector until the [`MultiVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`data_u64_index`]: struct.A.html#method.data_u64_index
    /// [`MultiVector::close`]: flatdata/struct.MultiVector.html#method.close
    #[inline]
    pub fn start_data_u64_index(&self) -> ::std::io::Result<flatdata::MultiVector<DataU64Index>> {
        flatdata::create_multi_vector(&*self.storage, "data_u64_index", schema::a::resources::DATA_U64_INDEX)
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

}

#[doc(hidden)]
pub mod _builtin {

#[allow(missing_docs)]
pub mod multivector {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const INDEX_TYPE8: &str = r#""#;
pub const INDEX_TYPE16: &str = r#""#;
pub const INDEX_TYPE64: &str = r#""#;
}

}

/// Builtin type to for MultiVector index
///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`IndexType8Ref`] for the read-only access, and
/// * [`IndexType8Mut`] for the mutable access
///
/// to the `IndexType8` data.
///
/// [`IndexType8Ref`]: struct.IndexType8Ref.html
/// [`IndexType8Mut`]: struct.IndexType8Mut.html
#[derive(Clone, Debug)]
pub struct IndexType8 {}

/// Read-only access to [`IndexType8`].
///
/// [`IndexType8`]: struct.IndexType8.html
#[derive(Clone, Copy)]
pub struct IndexType8Ref<'a> {
    pub(crate) data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for IndexType8
{
    const SCHEMA: &'static str = schema::structs::INDEX_TYPE8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;

    type Item = IndexType8Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = IndexType8Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}


impl<'a> IndexType8Ref<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 8);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data, 0, 8);
        let end = flatdata_read_bytes!(u64, self.data, 0 + 1 * 8, 8);
        start..end
    }

}

impl<'a> std::fmt::Debug for IndexType8Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType8")
            .field("value", &self.value())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for IndexType8Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl<'a> flatdata::Ref for IndexType8Ref<'a> {}

/// Mutable access to [`IndexType8`].
///
/// [`IndexType8`]: struct.IndexType8.html
pub struct IndexType8Mut<'a> {
    pub(crate) data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> IndexType8Mut<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType8Ref.html#method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 8);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_value(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 8)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType8Ref) {
        self.set_value(other.value());
    }
}

impl<'a> std::fmt::Debug for IndexType8Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        IndexType8Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for IndexType8Mut<'a> {}

impl<'a> flatdata::IndexStruct<'a> for IndexType8 {
    #[inline]
    fn range(data: Self::Item) -> std::ops::Range<usize> {
        let range = data.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(mut data: Self::ItemMut, value: usize) {
        data.set_value(value as u64);
    }
}



/// Builtin type to for MultiVector index
///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`IndexType16Ref`] for the read-only access, and
/// * [`IndexType16Mut`] for the mutable access
///
/// to the `IndexType16` data.
///
/// [`IndexType16Ref`]: struct.IndexType16Ref.html
/// [`IndexType16Mut`]: struct.IndexType16Mut.html
#[derive(Clone, Debug)]
pub struct IndexType16 {}

/// Read-only access to [`IndexType16`].
///
/// [`IndexType16`]: struct.IndexType16.html
#[derive(Clone, Copy)]
pub struct IndexType16Ref<'a> {
    pub(crate) data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for IndexType16
{
    const SCHEMA: &'static str = schema::structs::INDEX_TYPE16;
    const SIZE_IN_BYTES: usize = 2;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;

    type Item = IndexType16Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = IndexType16Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}


impl<'a> IndexType16Ref<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 16);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data, 0, 16);
        let end = flatdata_read_bytes!(u64, self.data, 0 + 2 * 8, 16);
        start..end
    }

}

impl<'a> std::fmt::Debug for IndexType16Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType16")
            .field("value", &self.value())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for IndexType16Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl<'a> flatdata::Ref for IndexType16Ref<'a> {}

/// Mutable access to [`IndexType16`].
///
/// [`IndexType16`]: struct.IndexType16.html
pub struct IndexType16Mut<'a> {
    pub(crate) data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> IndexType16Mut<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType16Ref.html#method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 16);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_value(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 2)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 16)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType16Ref) {
        self.set_value(other.value());
    }
}

impl<'a> std::fmt::Debug for IndexType16Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        IndexType16Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for IndexType16Mut<'a> {}

impl<'a> flatdata::IndexStruct<'a> for IndexType16 {
    #[inline]
    fn range(data: Self::Item) -> std::ops::Range<usize> {
        let range = data.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(mut data: Self::ItemMut, value: usize) {
        data.set_value(value as u64);
    }
}



/// Builtin type to for MultiVector index
///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`IndexType64Ref`] for the read-only access, and
/// * [`IndexType64Mut`] for the mutable access
///
/// to the `IndexType64` data.
///
/// [`IndexType64Ref`]: struct.IndexType64Ref.html
/// [`IndexType64Mut`]: struct.IndexType64Mut.html
#[derive(Clone, Debug)]
pub struct IndexType64 {}

/// Read-only access to [`IndexType64`].
///
/// [`IndexType64`]: struct.IndexType64.html
#[derive(Clone, Copy)]
pub struct IndexType64Ref<'a> {
    pub(crate) data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for IndexType64
{
    const SCHEMA: &'static str = schema::structs::INDEX_TYPE64;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;

    type Item = IndexType64Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = IndexType64Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}


impl<'a> IndexType64Ref<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data, 0, 64);
        let end = flatdata_read_bytes!(u64, self.data, 0 + 8 * 8, 64);
        start..end
    }

}

impl<'a> std::fmt::Debug for IndexType64Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType64")
            .field("value", &self.value())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for IndexType64Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl<'a> flatdata::Ref for IndexType64Ref<'a> {}

/// Mutable access to [`IndexType64`].
///
/// [`IndexType64`]: struct.IndexType64.html
pub struct IndexType64Mut<'a> {
    pub(crate) data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> IndexType64Mut<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType64Ref.html#method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_value(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType64Ref) {
        self.set_value(other.value());
    }
}

impl<'a> std::fmt::Debug for IndexType64Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        IndexType64Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for IndexType64Mut<'a> {}

impl<'a> flatdata::IndexStruct<'a> for IndexType64 {
    #[inline]
    fn range(data: Self::Item) -> std::ops::Range<usize> {
        let range = data.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(mut data: Self::ItemMut, value: usize) {
        data.set_value(value as u64);
    }
}

}

#[doc(hidden)]
pub mod schema {
pub mod structs {
}