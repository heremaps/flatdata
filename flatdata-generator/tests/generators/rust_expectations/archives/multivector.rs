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
#[repr(transparent)]
#[derive(Clone)]
pub struct T {
    data: [u8; 8],
}

impl T {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }
}

impl flatdata::Struct for T {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }

    const SCHEMA: &'static str = schema::structs::T;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl T {
    pub fn new( ) -> Self {
        Self{data : [0; 8]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 8]) -> &Self {
        // Safety: This is safe since T is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 8]) -> &mut Self {
        // Safety: This is safe since T is repr(transparent)
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

impl Default for T {
    fn default( ) -> Self {
        Self::new( )
    }
}

impl flatdata::NoOverlap for T {}

impl T {
    #[inline]
    pub fn x(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl std::fmt::Debug for T {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("T")
            .field("x", &self.x())
            .finish()
    }
}

impl std::cmp::PartialEq for T {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.x() == other.x()     }
}

impl T {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_x(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &T) {
        self.set_x(other.x());
    }
}


/// Enum for read-only heterogeneous access to elements in a
/// bucket of the [`data`] resource.
///
/// [`data`]: struct.Archive{.n.A}.html#method.data
#[derive(Clone, PartialEq)]
pub enum DataRef<'a> {
    #[allow(missing_docs)]
    S(&'a super::n::S),    #[allow(missing_docs)]
    T(&'a super::n::T),}

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
            DataRef::S(_) => <super::n::S as flatdata::Struct>::SIZE_IN_BYTES,
            DataRef::T(_) => <super::n::T as flatdata::Struct>::SIZE_IN_BYTES,
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
    pub fn add_s<'b>(&'b mut self) -> &'b mut super::n::S {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len] = 0;
        let slice = &mut self.data[1 + old_len..];
        super::n::S::from_bytes_slice_mut(slice).expect("Logic error: Cannot create super::n::S from slice")
    }
    /// Adds data of the type [`T`] to the bucket.
    ///
    /// [`T`]: struct.T.html
    #[inline]
    pub fn add_t<'b>(&'b mut self) -> &'b mut super::n::T {
        let old_len = self.data.len();
        let increment = 1 + <super::n::T as flatdata::Struct>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len] = 1;
        let slice = &mut self.data[1 + old_len..];
        super::n::T::from_bytes_slice_mut(slice).expect("Logic error: Cannot create super::n::T from slice")
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

impl flatdata::VariadicIndex for Data {
    type Index = super::_builtin::multivector::IndexType8;
}

impl<'a> flatdata::VariadicStruct<'a> for Data {
    type Item = DataRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => DataRef::S(super::n::S::from_bytes_slice(&data).expect("Corrupted data")),
                1 => DataRef::T(super::n::T::from_bytes_slice(&data).expect("Corrupted data")),
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
    S(&'a super::n::S),    #[allow(missing_docs)]
    T(&'a super::n::T),}

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
            OptionalDataRef::S(_) => <super::n::S as flatdata::Struct>::SIZE_IN_BYTES,
            OptionalDataRef::T(_) => <super::n::T as flatdata::Struct>::SIZE_IN_BYTES,
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
    pub fn add_s<'b>(&'b mut self) -> &'b mut super::n::S {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len] = 0;
        let slice = &mut self.data[1 + old_len..];
        super::n::S::from_bytes_slice_mut(slice).expect("Logic error: Cannot create super::n::S from slice")
    }
    /// Adds data of the type [`T`] to the bucket.
    ///
    /// [`T`]: struct.T.html
    #[inline]
    pub fn add_t<'b>(&'b mut self) -> &'b mut super::n::T {
        let old_len = self.data.len();
        let increment = 1 + <super::n::T as flatdata::Struct>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len] = 1;
        let slice = &mut self.data[1 + old_len..];
        super::n::T::from_bytes_slice_mut(slice).expect("Logic error: Cannot create super::n::T from slice")
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

impl flatdata::VariadicIndex for OptionalData {
    type Index = super::_builtin::multivector::IndexType16;
}

impl<'a> flatdata::VariadicStruct<'a> for OptionalData {
    type Item = OptionalDataRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => OptionalDataRef::S(super::n::S::from_bytes_slice(&data).expect("Corrupted data")),
                1 => OptionalDataRef::T(super::n::T::from_bytes_slice(&data).expect("Corrupted data")),
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
    S(&'a super::n::S),    #[allow(missing_docs)]
    T(&'a super::n::T),}

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
            DataU64IndexRef::S(_) => <super::n::S as flatdata::Struct>::SIZE_IN_BYTES,
            DataU64IndexRef::T(_) => <super::n::T as flatdata::Struct>::SIZE_IN_BYTES,
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
    pub fn add_s<'b>(&'b mut self) -> &'b mut super::n::S {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len] = 0;
        let slice = &mut self.data[1 + old_len..];
        super::n::S::from_bytes_slice_mut(slice).expect("Logic error: Cannot create super::n::S from slice")
    }
    /// Adds data of the type [`T`] to the bucket.
    ///
    /// [`T`]: struct.T.html
    #[inline]
    pub fn add_t<'b>(&'b mut self) -> &'b mut super::n::T {
        let old_len = self.data.len();
        let increment = 1 + <super::n::T as flatdata::Struct>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len] = 1;
        let slice = &mut self.data[1 + old_len..];
        super::n::T::from_bytes_slice_mut(slice).expect("Logic error: Cannot create super::n::T from slice")
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

impl flatdata::VariadicIndex for DataU64Index {
    type Index = super::_builtin::multivector::IndexType64;
}

impl<'a> flatdata::VariadicStruct<'a> for DataU64Index {
    type Item = DataU64IndexRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => DataU64IndexRef::S(super::n::S::from_bytes_slice(&data).expect("Corrupted data")),
                1 => DataU64IndexRef::T(super::n::T::from_bytes_slice(&data).expect("Corrupted data")),
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
    data : flatdata::MultiArrayView<'static, Data>,
    optional_data : Option<flatdata::MultiArrayView<'static, OptionalData>>,
    data_u64_index : flatdata::MultiArrayView<'static, DataU64Index>,
}

impl A {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn data(&self) -> &flatdata::MultiArrayView<Data> {
        &self.data
    }

    #[inline]
    pub fn optional_data(&self) -> Option<&flatdata::MultiArrayView<OptionalData>> {
        self.optional_data.as_ref()
    }

    #[inline]
    pub fn data_u64_index(&self) -> &flatdata::MultiArrayView<DataU64Index> {
        &self.data_u64_index
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
        #[allow(unused_imports)]
        use flatdata::SliceExt;
        // extend lifetime since Rust cannot know that we reference a cache here
        #[allow(unused_variables)]
        let extend = |x : Result<&[u8], flatdata::ResourceStorageError>| -> Result<&'static [u8], flatdata::ResourceStorageError> {x.map(|x| unsafe{std::mem::transmute(x)})};

        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let data = {
            let index_schema = &format!("index({})", schema::a::resources::DATA);
            let index = extend(storage.read("data_index", &index_schema));
            let data = extend(storage.read("data", schema::a::resources::DATA));
            let result = match (index, data) {
                (Ok(index), Ok(data)) => {
                    Ok(flatdata::MultiArrayView::new(
                        <&[super::_builtin::multivector::IndexType8]>::from_bytes(index)?,
                        data
                    ))
                }
                (Ok(_), Err(x)) | (Err(x), Ok(_)) => {return Err(x);}
                (Err(x), Err(_)) => Err(x),
            };
            result?
        };
        let optional_data = {
            let index_schema = &format!("index({})", schema::a::resources::OPTIONAL_DATA);
            let index = extend(storage.read("optional_data_index", &index_schema));
            let data = extend(storage.read("optional_data", schema::a::resources::OPTIONAL_DATA));
            let result = match (index, data) {
                (Ok(index), Ok(data)) => {
                    Ok(flatdata::MultiArrayView::new(
                        <&[super::_builtin::multivector::IndexType16]>::from_bytes(index)?,
                        data
                    ))
                }
                (Ok(_), Err(x)) | (Err(x), Ok(_)) => {return Err(x);}
                (Err(x), Err(_)) => Err(x),
            };
            result.ok()
        };
        let data_u64_index = {
            let index_schema = &format!("index({})", schema::a::resources::DATA_U64_INDEX);
            let index = extend(storage.read("data_u64_index_index", &index_schema));
            let data = extend(storage.read("data_u64_index", schema::a::resources::DATA_U64_INDEX));
            let result = match (index, data) {
                (Ok(index), Ok(data)) => {
                    Ok(flatdata::MultiArrayView::new(
                        <&[super::_builtin::multivector::IndexType64]>::from_bytes(index)?,
                        data
                    ))
                }
                (Ok(_), Err(x)) | (Err(x), Ok(_)) => {return Err(x);}
                (Err(x), Err(_)) => Err(x),
            };
            result?
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
#[repr(transparent)]
pub struct IndexType8 {
    data: [u8; 1],
}

impl IndexType8 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }
}

impl flatdata::Struct for IndexType8 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 1]}
    }

    const SCHEMA: &'static str = schema::structs::INDEX_TYPE8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;
}


impl IndexType8 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 8);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 8);
        let end = flatdata_read_bytes!(u64, self.data.as_ptr(), 0 + 1 * 8, 8);
        start..end
    }

}

impl std::fmt::Debug for IndexType8 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType8")
            .field("value", &self.value())
            .finish()
    }
}

impl std::cmp::PartialEq for IndexType8 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl IndexType8 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType8Ref.html#method.range
    #[inline]
    #[allow(missing_docs)]
    pub fn set_value(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 8)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType8) {
        self.set_value(other.value());
    }
}

impl flatdata::IndexStruct for IndexType8 {
    #[inline]
    fn range(&self) -> std::ops::Range<usize> {
        let range = self.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(&mut self, value: usize) {
        self.set_value(value as u64);
    }
}


/// Builtin type to for MultiVector index
#[repr(transparent)]
pub struct IndexType16 {
    data: [u8; 2],
}

impl IndexType16 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }
}

impl flatdata::Struct for IndexType16 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 2]}
    }

    const SCHEMA: &'static str = schema::structs::INDEX_TYPE16;
    const SIZE_IN_BYTES: usize = 2;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;
}


impl IndexType16 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 16);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 16);
        let end = flatdata_read_bytes!(u64, self.data.as_ptr(), 0 + 2 * 8, 16);
        start..end
    }

}

impl std::fmt::Debug for IndexType16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType16")
            .field("value", &self.value())
            .finish()
    }
}

impl std::cmp::PartialEq for IndexType16 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl IndexType16 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType16Ref.html#method.range
    #[inline]
    #[allow(missing_docs)]
    pub fn set_value(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 16)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType16) {
        self.set_value(other.value());
    }
}

impl flatdata::IndexStruct for IndexType16 {
    #[inline]
    fn range(&self) -> std::ops::Range<usize> {
        let range = self.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(&mut self, value: usize) {
        self.set_value(value as u64);
    }
}


/// Builtin type to for MultiVector index
#[repr(transparent)]
pub struct IndexType64 {
    data: [u8; 8],
}

impl IndexType64 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }
}

impl flatdata::Struct for IndexType64 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 8]}
    }

    const SCHEMA: &'static str = schema::structs::INDEX_TYPE64;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;
}


impl IndexType64 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 64);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 64);
        let end = flatdata_read_bytes!(u64, self.data.as_ptr(), 0 + 8 * 8, 64);
        start..end
    }

}

impl std::fmt::Debug for IndexType64 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType64")
            .field("value", &self.value())
            .finish()
    }
}

impl std::cmp::PartialEq for IndexType64 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl IndexType64 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType64Ref.html#method.range
    #[inline]
    #[allow(missing_docs)]
    pub fn set_value(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 64)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType64) {
        self.set_value(other.value());
    }
}

impl flatdata::IndexStruct for IndexType64 {
    #[inline]
    fn range(&self) -> std::ops::Range<usize> {
        let range = self.range();
        range.start as usize..range.end as usize
    }

    #[inline]
    fn set_index(&mut self, value: usize) {
        self.set_value(value as u64);
    }
}

}

#[doc(hidden)]
pub mod schema {
pub mod structs {
}