#[derive(Clone, Debug)]
pub struct S {}

/// Read-only access to [`S`].
///
/// [`S`]: struct.S.html
#[derive(Clone, Copy)]
pub struct SRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for S
{
    const SCHEMA: &'static str = schema::structs::S;
    const SIZE_IN_BYTES: usize = 4;
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
    pub fn x(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
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
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> SMut<'a> {
    #[inline]
    pub fn x(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_x(&mut self, value: u32) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(u32; value, buffer, 0, 32)
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
/// * [`RRef`] for the read-only access, and
/// * [`RMut`] for the mutable access
///
/// to the `R` data.
///
/// [`RRef`]: struct.RRef.html
/// [`RMut`]: struct.RMut.html
#[derive(Clone, Debug)]
pub struct R {}

/// Read-only access to [`R`].
///
/// [`R`]: struct.R.html
#[derive(Clone, Copy)]
pub struct RRef<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for R
{
    const SCHEMA: &'static str = schema::structs::R;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = RRef<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = RMut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for R {}

impl<'a> RRef<'a> {
    #[inline]
    pub fn ref_(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

}

impl<'a> std::fmt::Debug for RRef<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("R")
            .field("ref_", &self.ref_())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for RRef<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.ref_() == other.ref_()     }
}

impl<'a> flatdata::Ref for RRef<'a> {}

/// Mutable access to [`R`].
///
/// [`R`]: struct.R.html
pub struct RMut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> RMut<'a> {
    #[inline]
    pub fn ref_(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 32);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_ref_(&mut self, value: u32) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(u32; value, buffer, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &RRef) {
        self.set_ref_(other.ref_());
    }
}

impl<'a> std::fmt::Debug for RMut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        RRef { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for RMut<'a> {}


/// Enum for read-only heterogeneous access to elements in a
/// bucket of the [`multilist`] resource.
///
/// [`multilist`]: struct.Archive{.n.A}.html#method.multilist
#[derive(Clone, PartialEq)]
pub enum MultilistRef<'a> {
    #[allow(missing_docs)]
    S(<super::n::S as flatdata::Struct<'a>>::Item),}

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
            MultilistRef::S(_) => <super::n::S as flatdata::Struct<'a>>::SIZE_IN_BYTES,
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
    pub fn add_s<'b>(&'b mut self) -> <super::n::S as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::S as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 0;
        <super::n::S as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
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

impl<'a> flatdata::VariadicStruct<'a> for Multilist {
    type Index = super::_builtin::multivector::IndexType32;

    type Item = MultilistRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => MultilistRef::S(<super::n::S as flatdata::Struct<'a>>::create(data)),
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
    R(<super::n::R as flatdata::Struct<'a>>::Item),}

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
            MultirefsRef::R(_) => <super::n::R as flatdata::Struct<'a>>::SIZE_IN_BYTES,
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
    pub fn add_r<'b>(&'b mut self) -> <super::n::R as flatdata::Struct<'b>>::ItemMut {
        let old_len = self.data.len();
        let increment = 1 + <super::n::R as flatdata::Struct<'b>>::SIZE_IN_BYTES;
        self.data.resize(old_len + increment, 0);
        self.data[old_len - flatdata::PADDING_SIZE] = 0;
        <super::n::R as flatdata::Struct<'b>>::create_mut(
            &mut self.data[1 + old_len - flatdata::PADDING_SIZE..]
        )
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

impl<'a> flatdata::VariadicStruct<'a> for Multirefs {
    type Index = super::_builtin::multivector::IndexType32;

    type Item = MultirefsRef<'a>;

    #[inline]
    fn create(index: flatdata::TypeIndex, data: &'a [u8]) -> Self::Item
    {
        match index {
                0 => MultirefsRef::R(<super::n::R as flatdata::Struct<'a>>::create(data)),
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
    list1: flatdata::MemoryDescriptor,
    list2: flatdata::MemoryDescriptor,
    multilist: (flatdata::MemoryDescriptor, flatdata::MemoryDescriptor),
    refs: flatdata::MemoryDescriptor,
    multirefs: (flatdata::MemoryDescriptor, flatdata::MemoryDescriptor),
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
    pub fn list1(&self) -> flatdata::ArrayView<super::n::S>
    {
        flatdata::ArrayView::new(&unsafe {self.list1.as_bytes()})
    }

    #[inline]
    pub fn list2(&self) -> flatdata::ArrayView<super::n::S>
    {
        flatdata::ArrayView::new(&unsafe {self.list2.as_bytes()})
    }

    #[inline]
    pub fn multilist(&self) -> flatdata::MultiArrayView<Multilist>
    {
        flatdata::MultiArrayView::new(
            flatdata::ArrayView::new(&unsafe {self.multilist.0.as_bytes()}),
            &unsafe {self.multilist.1.as_bytes()},
        )
    }

    #[inline]
    pub fn refs(&self) -> flatdata::ArrayView<super::n::R>
    {
        flatdata::ArrayView::new(&unsafe {self.refs.as_bytes()})
    }

    #[inline]
    pub fn multirefs(&self) -> flatdata::MultiArrayView<Multirefs>
    {
        flatdata::MultiArrayView::new(
            flatdata::ArrayView::new(&unsafe {self.multirefs.0.as_bytes()}),
            &unsafe {self.multirefs.1.as_bytes()},
        )
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
        storage.read(&Self::signature_name(Self::NAME), Self::SCHEMA)?;

        let list1 = Self::read_resource(&*storage, "list1", schema::a::resources::LIST1)?;
        let list2 = Self::read_resource(&*storage, "list2", schema::a::resources::LIST2)?;
        let multilist = {
            let index_schema = &format!("index({})", schema::a::resources::MULTILIST);
            let index = Self::read_resource(&*storage, "multilist_index", &index_schema)?;
            let data = Self::read_resource(&*storage, "multilist", schema::a::resources::MULTILIST)?;            (index, data)
        };
        let refs = Self::read_resource(&*storage, "refs", schema::a::resources::REFS)?;
        let multirefs = {
            let index_schema = &format!("index({})", schema::a::resources::MULTIREFS);
            let index = Self::read_resource(&*storage, "multirefs_index", &index_schema)?;
            let data = Self::read_resource(&*storage, "multirefs", schema::a::resources::MULTIREFS)?;            (index, data)
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
    pub fn set_list1(&self, vector: &flatdata::ArrayView<super::n::S>) -> ::std::io::Result<()> {
        self.storage.write("list1", schema::a::resources::LIST1, vector.as_ref())
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
    pub fn set_list2(&self, vector: &flatdata::ArrayView<super::n::S>) -> ::std::io::Result<()> {
        self.storage.write("list2", schema::a::resources::LIST2, vector.as_ref())
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
    pub fn set_refs(&self, vector: &flatdata::ArrayView<super::n::R>) -> ::std::io::Result<()> {
        self.storage.write("refs", schema::a::resources::REFS, vector.as_ref())
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
///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`IndexType32Ref`] for the read-only access, and
/// * [`IndexType32Mut`] for the mutable access
///
/// to the `IndexType32` data.
///
/// [`IndexType32Ref`]: struct.IndexType32Ref.html
/// [`IndexType32Mut`]: struct.IndexType32Mut.html
#[derive(Clone, Debug)]
pub struct IndexType32 {}

/// Read-only access to [`IndexType32`].
///
/// [`IndexType32`]: struct.IndexType32.html
#[derive(Clone, Copy)]
pub struct IndexType32Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for IndexType32
{
    const SCHEMA: &'static str = schema::structs::INDEX_TYPE32;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = true;

    type Item = IndexType32Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = IndexType32Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}


impl<'a> IndexType32Ref<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: #method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 32);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn range(&self) -> std::ops::Range<u64> {
        let start = flatdata_read_bytes!(u64, self.data, 0, 32);
        let end = flatdata_read_bytes!(u64, self.data, 0 + 4 * 8, 32);
        start..end
    }

}

impl<'a> std::fmt::Debug for IndexType32Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("IndexType32")
            .field("value", &self.value())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for IndexType32Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()     }
}

impl<'a> flatdata::Ref for IndexType32Ref<'a> {}

/// Mutable access to [`IndexType32`].
///
/// [`IndexType32`]: struct.IndexType32.html
pub struct IndexType32Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> IndexType32Mut<'a> {
    /// First element of the range [`range`].
    ///
    /// [`range`]: struct.IndexType32Ref.html#method.range
    #[inline]
    pub fn value(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 32);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_value(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 32)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &IndexType32Ref) {
        self.set_value(other.value());
    }
}

impl<'a> std::fmt::Debug for IndexType32Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        IndexType32Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for IndexType32Mut<'a> {}

impl<'a> flatdata::IndexStruct<'a> for IndexType32 {
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