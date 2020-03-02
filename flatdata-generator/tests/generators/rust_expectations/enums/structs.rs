///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`StructEnumI8Ref`] for the read-only access, and
/// * [`StructEnumI8Mut`] for the mutable access
///
/// to the `StructEnumI8` data.
///
/// [`StructEnumI8Ref`]: struct.StructEnumI8Ref.html
/// [`StructEnumI8Mut`]: struct.StructEnumI8Mut.html
#[derive(Clone, Debug)]
pub struct StructEnumI8 {}

/// Read-only access to [`StructEnumI8`].
///
/// [`StructEnumI8`]: struct.StructEnumI8.html
#[derive(Clone, Copy)]
pub struct StructEnumI8Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for StructEnumI8
{
    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMI8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = StructEnumI8Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = StructEnumI8Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for StructEnumI8 {}

impl<'a> StructEnumI8Ref<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumI8 {
        let value = flatdata_read_bytes!(i8, self.data, 0, 1);
        unsafe { std::mem::transmute::<i8, super::n::EnumI8>(value) }
    }

}

impl<'a> std::fmt::Debug for StructEnumI8Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumI8")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for StructEnumI8Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for StructEnumI8Ref<'a> {}

/// Mutable access to [`StructEnumI8`].
///
/// [`StructEnumI8`]: struct.StructEnumI8.html
pub struct StructEnumI8Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> StructEnumI8Mut<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumI8 {
        let value = flatdata_read_bytes!(i8, self.data, 0, 1);
        unsafe { std::mem::transmute::<i8, super::n::EnumI8>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: super::n::EnumI8) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(i8; value, buffer, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumI8Ref) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for StructEnumI8Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        StructEnumI8Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for StructEnumI8Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`StructEnumU8Ref`] for the read-only access, and
/// * [`StructEnumU8Mut`] for the mutable access
///
/// to the `StructEnumU8` data.
///
/// [`StructEnumU8Ref`]: struct.StructEnumU8Ref.html
/// [`StructEnumU8Mut`]: struct.StructEnumU8Mut.html
#[derive(Clone, Debug)]
pub struct StructEnumU8 {}

/// Read-only access to [`StructEnumU8`].
///
/// [`StructEnumU8`]: struct.StructEnumU8.html
#[derive(Clone, Copy)]
pub struct StructEnumU8Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for StructEnumU8
{
    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMU8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = StructEnumU8Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = StructEnumU8Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for StructEnumU8 {}

impl<'a> StructEnumU8Ref<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumU8 {
        let value = flatdata_read_bytes!(u8, self.data, 0, 1);
        unsafe { std::mem::transmute::<u8, super::n::EnumU8>(value) }
    }

}

impl<'a> std::fmt::Debug for StructEnumU8Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumU8")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for StructEnumU8Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for StructEnumU8Ref<'a> {}

/// Mutable access to [`StructEnumU8`].
///
/// [`StructEnumU8`]: struct.StructEnumU8.html
pub struct StructEnumU8Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> StructEnumU8Mut<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumU8 {
        let value = flatdata_read_bytes!(u8, self.data, 0, 1);
        unsafe { std::mem::transmute::<u8, super::n::EnumU8>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: super::n::EnumU8) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(u8; value, buffer, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumU8Ref) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for StructEnumU8Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        StructEnumU8Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for StructEnumU8Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`StructEnumI16Ref`] for the read-only access, and
/// * [`StructEnumI16Mut`] for the mutable access
///
/// to the `StructEnumI16` data.
///
/// [`StructEnumI16Ref`]: struct.StructEnumI16Ref.html
/// [`StructEnumI16Mut`]: struct.StructEnumI16Mut.html
#[derive(Clone, Debug)]
pub struct StructEnumI16 {}

/// Read-only access to [`StructEnumI16`].
///
/// [`StructEnumI16`]: struct.StructEnumI16.html
#[derive(Clone, Copy)]
pub struct StructEnumI16Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for StructEnumI16
{
    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMI16;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = StructEnumI16Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = StructEnumI16Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for StructEnumI16 {}

impl<'a> StructEnumI16Ref<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumI16 {
        let value = flatdata_read_bytes!(i16, self.data, 0, 1);
        unsafe { std::mem::transmute::<i16, super::n::EnumI16>(value) }
    }

}

impl<'a> std::fmt::Debug for StructEnumI16Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumI16")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for StructEnumI16Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for StructEnumI16Ref<'a> {}

/// Mutable access to [`StructEnumI16`].
///
/// [`StructEnumI16`]: struct.StructEnumI16.html
pub struct StructEnumI16Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> StructEnumI16Mut<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumI16 {
        let value = flatdata_read_bytes!(i16, self.data, 0, 1);
        unsafe { std::mem::transmute::<i16, super::n::EnumI16>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: super::n::EnumI16) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(i16; value, buffer, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumI16Ref) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for StructEnumI16Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        StructEnumI16Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for StructEnumI16Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`StructEnumU16Ref`] for the read-only access, and
/// * [`StructEnumU16Mut`] for the mutable access
///
/// to the `StructEnumU16` data.
///
/// [`StructEnumU16Ref`]: struct.StructEnumU16Ref.html
/// [`StructEnumU16Mut`]: struct.StructEnumU16Mut.html
#[derive(Clone, Debug)]
pub struct StructEnumU16 {}

/// Read-only access to [`StructEnumU16`].
///
/// [`StructEnumU16`]: struct.StructEnumU16.html
#[derive(Clone, Copy)]
pub struct StructEnumU16Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for StructEnumU16
{
    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMU16;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = StructEnumU16Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = StructEnumU16Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for StructEnumU16 {}

impl<'a> StructEnumU16Ref<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumU16 {
        let value = flatdata_read_bytes!(u16, self.data, 0, 1);
        unsafe { std::mem::transmute::<u16, super::n::EnumU16>(value) }
    }

}

impl<'a> std::fmt::Debug for StructEnumU16Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumU16")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for StructEnumU16Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for StructEnumU16Ref<'a> {}

/// Mutable access to [`StructEnumU16`].
///
/// [`StructEnumU16`]: struct.StructEnumU16.html
pub struct StructEnumU16Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> StructEnumU16Mut<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumU16 {
        let value = flatdata_read_bytes!(u16, self.data, 0, 1);
        unsafe { std::mem::transmute::<u16, super::n::EnumU16>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: super::n::EnumU16) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(u16; value, buffer, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumU16Ref) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for StructEnumU16Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        StructEnumU16Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for StructEnumU16Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`StructEnumI32Ref`] for the read-only access, and
/// * [`StructEnumI32Mut`] for the mutable access
///
/// to the `StructEnumI32` data.
///
/// [`StructEnumI32Ref`]: struct.StructEnumI32Ref.html
/// [`StructEnumI32Mut`]: struct.StructEnumI32Mut.html
#[derive(Clone, Debug)]
pub struct StructEnumI32 {}

/// Read-only access to [`StructEnumI32`].
///
/// [`StructEnumI32`]: struct.StructEnumI32.html
#[derive(Clone, Copy)]
pub struct StructEnumI32Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for StructEnumI32
{
    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMI32;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = StructEnumI32Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = StructEnumI32Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for StructEnumI32 {}

impl<'a> StructEnumI32Ref<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumI32 {
        let value = flatdata_read_bytes!(i32, self.data, 0, 1);
        unsafe { std::mem::transmute::<i32, super::n::EnumI32>(value) }
    }

}

impl<'a> std::fmt::Debug for StructEnumI32Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumI32")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for StructEnumI32Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for StructEnumI32Ref<'a> {}

/// Mutable access to [`StructEnumI32`].
///
/// [`StructEnumI32`]: struct.StructEnumI32.html
pub struct StructEnumI32Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> StructEnumI32Mut<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumI32 {
        let value = flatdata_read_bytes!(i32, self.data, 0, 1);
        unsafe { std::mem::transmute::<i32, super::n::EnumI32>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: super::n::EnumI32) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(i32; value, buffer, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumI32Ref) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for StructEnumI32Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        StructEnumI32Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for StructEnumI32Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`StructEnumU32Ref`] for the read-only access, and
/// * [`StructEnumU32Mut`] for the mutable access
///
/// to the `StructEnumU32` data.
///
/// [`StructEnumU32Ref`]: struct.StructEnumU32Ref.html
/// [`StructEnumU32Mut`]: struct.StructEnumU32Mut.html
#[derive(Clone, Debug)]
pub struct StructEnumU32 {}

/// Read-only access to [`StructEnumU32`].
///
/// [`StructEnumU32`]: struct.StructEnumU32.html
#[derive(Clone, Copy)]
pub struct StructEnumU32Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for StructEnumU32
{
    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMU32;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = StructEnumU32Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = StructEnumU32Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for StructEnumU32 {}

impl<'a> StructEnumU32Ref<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumU32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 1);
        unsafe { std::mem::transmute::<u32, super::n::EnumU32>(value) }
    }

}

impl<'a> std::fmt::Debug for StructEnumU32Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumU32")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for StructEnumU32Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for StructEnumU32Ref<'a> {}

/// Mutable access to [`StructEnumU32`].
///
/// [`StructEnumU32`]: struct.StructEnumU32.html
pub struct StructEnumU32Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> StructEnumU32Mut<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumU32 {
        let value = flatdata_read_bytes!(u32, self.data, 0, 1);
        unsafe { std::mem::transmute::<u32, super::n::EnumU32>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: super::n::EnumU32) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(u32; value, buffer, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumU32Ref) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for StructEnumU32Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        StructEnumU32Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for StructEnumU32Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`StructEnumI64Ref`] for the read-only access, and
/// * [`StructEnumI64Mut`] for the mutable access
///
/// to the `StructEnumI64` data.
///
/// [`StructEnumI64Ref`]: struct.StructEnumI64Ref.html
/// [`StructEnumI64Mut`]: struct.StructEnumI64Mut.html
#[derive(Clone, Debug)]
pub struct StructEnumI64 {}

/// Read-only access to [`StructEnumI64`].
///
/// [`StructEnumI64`]: struct.StructEnumI64.html
#[derive(Clone, Copy)]
pub struct StructEnumI64Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for StructEnumI64
{
    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMI64;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = StructEnumI64Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = StructEnumI64Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for StructEnumI64 {}

impl<'a> StructEnumI64Ref<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumI64 {
        let value = flatdata_read_bytes!(i64, self.data, 0, 1);
        unsafe { std::mem::transmute::<i64, super::n::EnumI64>(value) }
    }

}

impl<'a> std::fmt::Debug for StructEnumI64Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumI64")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for StructEnumI64Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for StructEnumI64Ref<'a> {}

/// Mutable access to [`StructEnumI64`].
///
/// [`StructEnumI64`]: struct.StructEnumI64.html
pub struct StructEnumI64Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> StructEnumI64Mut<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumI64 {
        let value = flatdata_read_bytes!(i64, self.data, 0, 1);
        unsafe { std::mem::transmute::<i64, super::n::EnumI64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: super::n::EnumI64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(i64; value, buffer, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumI64Ref) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for StructEnumI64Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        StructEnumI64Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for StructEnumI64Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`StructEnumU64Ref`] for the read-only access, and
/// * [`StructEnumU64Mut`] for the mutable access
///
/// to the `StructEnumU64` data.
///
/// [`StructEnumU64Ref`]: struct.StructEnumU64Ref.html
/// [`StructEnumU64Mut`]: struct.StructEnumU64Mut.html
#[derive(Clone, Debug)]
pub struct StructEnumU64 {}

/// Read-only access to [`StructEnumU64`].
///
/// [`StructEnumU64`]: struct.StructEnumU64.html
#[derive(Clone, Copy)]
pub struct StructEnumU64Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for StructEnumU64
{
    const SCHEMA: &'static str = schema::structs::STRUCT_ENUMU64;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = StructEnumU64Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = StructEnumU64Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for StructEnumU64 {}

impl<'a> StructEnumU64Ref<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumU64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 1);
        unsafe { std::mem::transmute::<u64, super::n::EnumU64>(value) }
    }

}

impl<'a> std::fmt::Debug for StructEnumU64Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("StructEnumU64")
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for StructEnumU64Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for StructEnumU64Ref<'a> {}

/// Mutable access to [`StructEnumU64`].
///
/// [`StructEnumU64`]: struct.StructEnumU64.html
pub struct StructEnumU64Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> StructEnumU64Mut<'a> {
    #[inline]
    pub fn f(&self) -> super::n::EnumU64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 1);
        unsafe { std::mem::transmute::<u64, super::n::EnumU64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: super::n::EnumU64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 1)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &StructEnumU64Ref) {
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for StructEnumU64Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        StructEnumU64Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for StructEnumU64Mut<'a> {}
#[derive(Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum EnumI8 {
    Value = 0,
}

impl flatdata::helper::Int for EnumI8 {
    const IS_SIGNED: bool = true;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnumU8 {
    Value = 0,
}

impl flatdata::helper::Int for EnumU8 {
    const IS_SIGNED: bool = false;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum EnumI16 {
    Value = 0,
}

impl flatdata::helper::Int for EnumI16 {
    const IS_SIGNED: bool = true;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EnumU16 {
    Value = 0,
}

impl flatdata::helper::Int for EnumU16 {
    const IS_SIGNED: bool = false;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum EnumI32 {
    Value = 0,
}

impl flatdata::helper::Int for EnumI32 {
    const IS_SIGNED: bool = true;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EnumU32 {
    Value = 0,
}

impl flatdata::helper::Int for EnumU32 {
    const IS_SIGNED: bool = false;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum EnumI64 {
    Value = 0,
}

impl flatdata::helper::Int for EnumI64 {
    const IS_SIGNED: bool = true;
}
#[derive(Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum EnumU64 {
    Value = 0,
}

impl flatdata::helper::Int for EnumU64 {
    const IS_SIGNED: bool = false;
}