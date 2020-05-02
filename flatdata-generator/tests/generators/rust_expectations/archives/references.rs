#[repr(transparent)]
#[derive(Clone)]
pub struct S {
    data: [u8; 4],
}

impl S {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }
}

impl flatdata::Struct for S {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }

    const SCHEMA: &'static str = schema::structs::S;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl S {
    pub fn new( ) -> Self {
        Self{data : [0; 4]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 4]) -> &Self {
        // Safety: This is safe since S is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 4]) -> &mut Self {
        // Safety: This is safe since S is repr(transparent)
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

impl Default for S {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for S {}

impl S {
    #[inline]
    pub fn x(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
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
    pub fn set_x(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &S) {
        self.set_x(other.x());
    }
}
#[repr(transparent)]
#[derive(Clone)]
pub struct R {
    data: [u8; 4],
}

impl R {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }
}

impl flatdata::Struct for R {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }

    const SCHEMA: &'static str = schema::structs::R;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;
}

impl R {
    pub fn new( ) -> Self {
        Self{data : [0; 4]}
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes(data: &[u8; 4]) -> &Self {
        // Safety: This is safe since R is repr(transparent)
        unsafe{ std::mem::transmute( data ) }
    }

    /// Create reference from byte array of matching size
    pub fn from_bytes_mut(data: &mut [u8; 4]) -> &mut Self {
        // Safety: This is safe since R is repr(transparent)
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

impl Default for R {
    fn default( ) -> Self {
        Self::new( )
    }
}

unsafe impl flatdata::NoOverlap for R {}

impl R {
    #[inline]
    pub fn ref_(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data.as_ptr(), 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

}

impl std::fmt::Debug for R {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("R")
            .field("ref_", &self.ref_())
            .finish()
    }
}

impl std::cmp::PartialEq for R {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ref_() == other.ref_()     }
}

impl R {
    #[inline]
    #[allow(missing_docs)]
    pub fn set_ref_(&mut self, value: u32) {
        flatdata_write_bytes!(u32; value, self.data, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &R) {
        self.set_ref_(other.ref_());
    }
}


/// Enum for read-only heterogeneous access to elements in a
/// bucket of the [`multilist`] resource.
///
/// [`multilist`]: struct.Archive{.n.A}.html#method.multilist
#[derive(Clone, PartialEq)]
pub enum MultilistRef<'a> {
    #[allow(missing_docs)]
    S(&'a super::n::S),}

impl<'a> ::std::fmt::Debug for MultilistRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MultilistRef::S(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> flatdata::VariadicRef for MultilistRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            MultilistRef::S(_) => <super::n::S as flatdata::Struct>::SIZE_IN_BYTES,
        }
    }
}

/// Builder of buckets in the [`multilist`] resource.
///
/// Refers to a single bucket in the [`multilist`] multivector and
/// provides methods for adding heterogeneous data to the bucket.
///
/// [`multilist`]: struct.Archive{.n.A}.html#method.multilist
pub struct MultilistBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> MultilistBuilder<'a> {
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
}

/// Variadic struct attached to the [`multilist`] archive resource.
///
/// It unifies the following data types:
//
/// * [`S`]
///
/// ## Access pattern
///
/// This structure is used as a template parameter in [`multilist`] multivector/
/// multiarray view. It does not contain any data, instead it references
///
/// * [`MultilistRef`] for the read-only heterogeneous access, and
/// * [`MultilistBuilder`] for the mutable builder pattern access.
///
/// [`multilist`]: struct.Archive{.n.A}.html#method.multilist
/// [`MultilistRef`]: enum.MultilistRef.html
/// [`MultilistBuilder`]: struct.MultilistBuilder.html
/// [`S`]: struct.S.html
#[derive(Clone)]
pub struct Multilist {}

impl flatdata::VariadicIndex for Multilist {
    type Index = super::_builtin::multivector::IndexType32;
}

impl<'a> flatdata::VariadicStruct<'a> for Multilist {
    type Item = MultilistRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => MultilistRef::S(super::n::S::from_bytes_slice(&data).expect("Corrupted data")),
            _ => panic!("invalid type index {} for variadic type MultilistRef", index),
        }
    }

    type ItemMut = MultilistBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}
/// Enum for read-only heterogeneous access to elements in a
/// bucket of the [`multirefs`] resource.
///
/// [`multirefs`]: struct.Archive{.n.A}.html#method.multirefs
#[derive(Clone, PartialEq)]
pub enum MultirefsRef<'a> {
    #[allow(missing_docs)]
    R(&'a super::n::R),}

impl<'a> ::std::fmt::Debug for MultirefsRef<'a> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            MultirefsRef::R(ref inner) => write!(f, "{:?}", inner),
        }
    }
}

impl<'a> flatdata::VariadicRef for MultirefsRef<'a> {
    #[inline]
    fn size_in_bytes(&self) -> usize {
        match *self {
            MultirefsRef::R(_) => <super::n::R as flatdata::Struct>::SIZE_IN_BYTES,
        }
    }
}

/// Builder of buckets in the [`multirefs`] resource.
///
/// Refers to a single bucket in the [`multirefs`] multivector and
/// provides methods for adding heterogeneous data to the bucket.
///
/// [`multirefs`]: struct.Archive{.n.A}.html#method.multirefs
pub struct MultirefsBuilder<'a> {
    data: &'a mut Vec<u8>
}

impl<'a> MultirefsBuilder<'a> {
    /// Adds data of the type [`R`] to the bucket.
    ///
    /// [`R`]: struct.R.html
    #[inline]
    pub fn add_r<'b>(&'b mut self) -> &'b mut super::n::R {
        let old_len = self.data.len();
        let increment = 1 + <super::n::R as flatdata::Struct>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len] = 0;
        let slice = &mut self.data[1 + old_len..];
        super::n::R::from_bytes_slice_mut(slice).expect("Logic error: Cannot create super::n::R from slice")
    }
}

/// Variadic struct attached to the [`multirefs`] archive resource.
///
/// It unifies the following data types:
//
/// * [`R`]
///
/// ## Access pattern
///
/// This structure is used as a template parameter in [`multirefs`] multivector/
/// multiarray view. It does not contain any data, instead it references
///
/// * [`MultirefsRef`] for the read-only heterogeneous access, and
/// * [`MultirefsBuilder`] for the mutable builder pattern access.
///
/// [`multirefs`]: struct.Archive{.n.A}.html#method.multirefs
/// [`MultirefsRef`]: enum.MultirefsRef.html
/// [`MultirefsBuilder`]: struct.MultirefsBuilder.html
/// [`R`]: struct.R.html
#[derive(Clone)]
pub struct Multirefs {}

impl flatdata::VariadicIndex for Multirefs {
    type Index = super::_builtin::multivector::IndexType32;
}

impl<'a> flatdata::VariadicStruct<'a> for Multirefs {
    type Item = MultirefsRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => MultirefsRef::R(super::n::R::from_bytes_slice(&data).expect("Corrupted data")),
            _ => panic!("invalid type index {} for variadic type MultirefsRef", index),
        }
    }

    type ItemMut = MultirefsBuilder<'a>;

    #[inline]
    fn create_mut(data: &'a mut Vec<u8>) -> Self::ItemMut
    {
        Self::ItemMut { data }
    }
}

#[derive(Clone)]
pub struct A {
    _storage: ::std::rc::Rc<dyn flatdata::ResourceStorage>,
    list1 : &'static [super::n::S],
    list2 : &'static [super::n::S],
    multilist : flatdata::MultiArrayView<'static, Multilist>,
    refs : &'static [super::n::R],
    multirefs : flatdata::MultiArrayView<'static, Multirefs>,
}

impl A {
    fn signature_name(archive_name: &str) -> String {
        format!("{}.archive", archive_name)
    }

    #[inline]
    pub fn list1(&self) -> &[super::n::S] {
        self.list1
    }

    #[inline]
    pub fn list2(&self) -> &[super::n::S] {
        self.list2
    }

    #[inline]
    pub fn multilist(&self) -> &flatdata::MultiArrayView<Multilist> {
        &self.multilist
    }

    #[inline]
    pub fn refs(&self) -> &[super::n::R] {
        self.refs
    }

    #[inline]
    pub fn multirefs(&self) -> &flatdata::MultiArrayView<Multirefs> {
        &self.multirefs
    }

}

impl ::std::fmt::Debug for A {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct("A")
            .field("list1", &self.list1())
            .field("list2", &self.list2())
            .field("multilist", &self.multilist())
            .field("refs", &self.refs())
            .field("multirefs", &self.multirefs())
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

        let resource = extend(storage.read("list1", schema::a::resources::LIST1));
        let list1 = resource.map(|x| <&[super::n::S]>::from_bytes(x))??;
        let resource = extend(storage.read("list2", schema::a::resources::LIST2));
        let list2 = resource.map(|x| <&[super::n::S]>::from_bytes(x))??;
        let multilist = {
            let index_schema = &format!("index({})", schema::a::resources::MULTILIST);
            let index = extend(storage.read("multilist_index", &index_schema));
            let data = extend(storage.read("multilist", schema::a::resources::MULTILIST));
            let result = match (index, data) {
                (Ok(index), Ok(data)) => {
                    Ok(flatdata::MultiArrayView::new(
                        <&[super::_builtin::multivector::IndexType32]>::from_bytes(index)?,
                        data
                    ))
                }
                (Ok(_), Err(x)) | (Err(x), Ok(_)) => {return Err(x);}
                (Err(x), Err(_)) => Err(x),
            };
            result?
        };
        let resource = extend(storage.read("refs", schema::a::resources::REFS));
        let refs = resource.map(|x| <&[super::n::R]>::from_bytes(x))??;
        let multirefs = {
            let index_schema = &format!("index({})", schema::a::resources::MULTIREFS);
            let index = extend(storage.read("multirefs_index", &index_schema));
            let data = extend(storage.read("multirefs", schema::a::resources::MULTIREFS));
            let result = match (index, data) {
                (Ok(index), Ok(data)) => {
                    Ok(flatdata::MultiArrayView::new(
                        <&[super::_builtin::multivector::IndexType32]>::from_bytes(index)?,
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
            list1,
            list2,
            multilist,
            refs,
            multirefs,
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
    /// Stores [`list1`] in the archive.
    ///
    /// [`list1`]: struct.A.html#method.list1
    pub fn set_list1(&self, vector: &[super::n::S]) -> ::std::io::Result<()> {
        use flatdata::SliceExt;
        self.storage.write("list1", schema::a::resources::LIST1, vector.as_bytes())
    }

    /// Opens [`list1`] in the archive for buffered writing.
    ///
    /// Elements can be added to the vector until the [`ExternalVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`list1`]: struct.A.html#method.list1
    /// [`ExternalVector::close`]: flatdata/struct.ExternalVector.html#method.close
    #[inline]
    pub fn start_list1(&self) -> ::std::io::Result<flatdata::ExternalVector<super::n::S>> {
        flatdata::create_external_vector(&*self.storage, "list1", schema::a::resources::LIST1)
    }

    #[inline]
    /// Stores [`list2`] in the archive.
    ///
    /// [`list2`]: struct.A.html#method.list2
    pub fn set_list2(&self, vector: &[super::n::S]) -> ::std::io::Result<()> {
        use flatdata::SliceExt;
        self.storage.write("list2", schema::a::resources::LIST2, vector.as_bytes())
    }

    /// Opens [`list2`] in the archive for buffered writing.
    ///
    /// Elements can be added to the vector until the [`ExternalVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`list2`]: struct.A.html#method.list2
    /// [`ExternalVector::close`]: flatdata/struct.ExternalVector.html#method.close
    #[inline]
    pub fn start_list2(&self) -> ::std::io::Result<flatdata::ExternalVector<super::n::S>> {
        flatdata::create_external_vector(&*self.storage, "list2", schema::a::resources::LIST2)
    }

    /// Opens [`multilist`] in the archive for buffered writing.
    ///
    /// Elements can be added to the multivector until the [`MultiVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`multilist`]: struct.A.html#method.multilist
    /// [`MultiVector::close`]: flatdata/struct.MultiVector.html#method.close
    #[inline]
    pub fn start_multilist(&self) -> ::std::io::Result<flatdata::MultiVector<Multilist>> {
        flatdata::create_multi_vector(&*self.storage, "multilist", schema::a::resources::MULTILIST)
    }

    #[inline]
    /// Stores [`refs`] in the archive.
    ///
    /// [`refs`]: struct.A.html#method.refs
    pub fn set_refs(&self, vector: &[super::n::R]) -> ::std::io::Result<()> {
        use flatdata::SliceExt;
        self.storage.write("refs", schema::a::resources::REFS, vector.as_bytes())
    }

    /// Opens [`refs`] in the archive for buffered writing.
    ///
    /// Elements can be added to the vector until the [`ExternalVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`refs`]: struct.A.html#method.refs
    /// [`ExternalVector::close`]: flatdata/struct.ExternalVector.html#method.close
    #[inline]
    pub fn start_refs(&self) -> ::std::io::Result<flatdata::ExternalVector<super::n::R>> {
        flatdata::create_external_vector(&*self.storage, "refs", schema::a::resources::REFS)
    }

    /// Opens [`multirefs`] in the archive for buffered writing.
    ///
    /// Elements can be added to the multivector until the [`MultiVector::close`] method
    /// is called. To flush the data fully into the archive, this method must be called
    /// in the end.
    ///
    /// [`multirefs`]: struct.A.html#method.multirefs
    /// [`MultiVector::close`]: flatdata/struct.MultiVector.html#method.close
    #[inline]
    pub fn start_multirefs(&self) -> ::std::io::Result<flatdata::MultiVector<Multirefs>> {
        flatdata::create_multi_vector(&*self.storage, "multirefs", schema::a::resources::MULTIREFS)
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
pub const INDEX_TYPE32: &str = r#""#;
}

}
/// Builtin type to for MultiVector index
#[repr(transparent)]
pub struct IndexType32 {
    data: [u8; 4],
}

impl IndexType32 {
    /// Unsafe since the struct might not be self-contained
    pub unsafe fn new_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }
}

impl flatdata::Struct for IndexType32 {
    unsafe fn create_unchecked( ) -> Self {
        Self{data : [0; 4]}
    }

    const SCHEMA: &'static str = schema::structs::INDEX_TYPE32;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;
}

impl flatdata::Overlap for IndexType32 {}

impl IndexType32 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 32);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data.as_ptr(), 0, 32);
        let end = flatdata_read_bytes!(u64, self.data.as_ptr(), 0 + 4 * 8, 32);
        start..end
    }

}

impl std::fmt::Debug for IndexType32 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType32")
            .field("value", &self.value())
            .finish()
    }
}

impl std::cmp::PartialEq for IndexType32 {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl IndexType32 {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType32Ref.html#method.range
    #[inline]
    #[allow(missing_docs)]
    pub fn set_value(&mut self, value: u64) {
        flatdata_write_bytes!(u64; value, self.data, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType32) {
        self.set_value(other.value());
    }
}

impl flatdata::IndexStruct for IndexType32 {
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