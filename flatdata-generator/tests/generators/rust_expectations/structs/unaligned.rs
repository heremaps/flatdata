///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`U8Ref`] for the read-only access, and
/// * [`U8Mut`] for the mutable access
///
/// to the `U8` data.
///
/// [`U8Ref`]: struct.U8Ref.html
/// [`U8Mut`]: struct.U8Mut.html
#[derive(Clone, Debug)]
pub struct U8 {}

/// Read-only access to [`U8`].
///
/// [`U8`]: struct.U8.html
#[derive(Clone, Copy)]
pub struct U8Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for U8
{
    const SCHEMA: &'static str = schema::structs::U8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = U8Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = U8Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for U8 {}

impl<'a> U8Ref<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn f(&self) -> u8 {
        let value = flatdata_read_bytes!(u8, self.data, 3, 5);
        unsafe { std::mem::transmute::<u8, u8>(value) }
    }

}

impl<'a> std::fmt::Debug for U8Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("U8")
            .field("padding", &self.padding())
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for U8Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.padding() == other.padding() &&        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for U8Ref<'a> {}

/// Mutable access to [`U8`].
///
/// [`U8`]: struct.U8.html
pub struct U8Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> U8Mut<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_padding(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 3)
    }

    #[inline]
    pub fn f(&self) -> u8 {
        let value = flatdata_read_bytes!(u8, self.data, 3, 5);
        unsafe { std::mem::transmute::<u8, u8>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: u8) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(u8; value, buffer, 3, 5)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &U8Ref) {
        self.set_padding(other.padding());
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for U8Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        U8Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for U8Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`I8Ref`] for the read-only access, and
/// * [`I8Mut`] for the mutable access
///
/// to the `I8` data.
///
/// [`I8Ref`]: struct.I8Ref.html
/// [`I8Mut`]: struct.I8Mut.html
#[derive(Clone, Debug)]
pub struct I8 {}

/// Read-only access to [`I8`].
///
/// [`I8`]: struct.I8.html
#[derive(Clone, Copy)]
pub struct I8Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for I8
{
    const SCHEMA: &'static str = schema::structs::I8;
    const SIZE_IN_BYTES: usize = 1;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = I8Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = I8Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for I8 {}

impl<'a> I8Ref<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn f(&self) -> i8 {
        let value = flatdata_read_bytes!(i8, self.data, 3, 5);
        unsafe { std::mem::transmute::<i8, i8>(value) }
    }

}

impl<'a> std::fmt::Debug for I8Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("I8")
            .field("padding", &self.padding())
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for I8Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.padding() == other.padding() &&        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for I8Ref<'a> {}

/// Mutable access to [`I8`].
///
/// [`I8`]: struct.I8.html
pub struct I8Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> I8Mut<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_padding(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 3)
    }

    #[inline]
    pub fn f(&self) -> i8 {
        let value = flatdata_read_bytes!(i8, self.data, 3, 5);
        unsafe { std::mem::transmute::<i8, i8>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: i8) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 1)
        };
        flatdata_write_bytes!(i8; value, buffer, 3, 5)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &I8Ref) {
        self.set_padding(other.padding());
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for I8Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        I8Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for I8Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`U16Ref`] for the read-only access, and
/// * [`U16Mut`] for the mutable access
///
/// to the `U16` data.
///
/// [`U16Ref`]: struct.U16Ref.html
/// [`U16Mut`]: struct.U16Mut.html
#[derive(Clone, Debug)]
pub struct U16 {}

/// Read-only access to [`U16`].
///
/// [`U16`]: struct.U16.html
#[derive(Clone, Copy)]
pub struct U16Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for U16
{
    const SCHEMA: &'static str = schema::structs::U16;
    const SIZE_IN_BYTES: usize = 2;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = U16Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = U16Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for U16 {}

impl<'a> U16Ref<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn f(&self) -> u16 {
        let value = flatdata_read_bytes!(u16, self.data, 3, 13);
        unsafe { std::mem::transmute::<u16, u16>(value) }
    }

}

impl<'a> std::fmt::Debug for U16Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("U16")
            .field("padding", &self.padding())
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for U16Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.padding() == other.padding() &&        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for U16Ref<'a> {}

/// Mutable access to [`U16`].
///
/// [`U16`]: struct.U16.html
pub struct U16Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> U16Mut<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_padding(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 2)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 3)
    }

    #[inline]
    pub fn f(&self) -> u16 {
        let value = flatdata_read_bytes!(u16, self.data, 3, 13);
        unsafe { std::mem::transmute::<u16, u16>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: u16) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 2)
        };
        flatdata_write_bytes!(u16; value, buffer, 3, 13)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &U16Ref) {
        self.set_padding(other.padding());
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for U16Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        U16Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for U16Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`I16Ref`] for the read-only access, and
/// * [`I16Mut`] for the mutable access
///
/// to the `I16` data.
///
/// [`I16Ref`]: struct.I16Ref.html
/// [`I16Mut`]: struct.I16Mut.html
#[derive(Clone, Debug)]
pub struct I16 {}

/// Read-only access to [`I16`].
///
/// [`I16`]: struct.I16.html
#[derive(Clone, Copy)]
pub struct I16Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for I16
{
    const SCHEMA: &'static str = schema::structs::I16;
    const SIZE_IN_BYTES: usize = 2;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = I16Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = I16Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for I16 {}

impl<'a> I16Ref<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn f(&self) -> i16 {
        let value = flatdata_read_bytes!(i16, self.data, 3, 13);
        unsafe { std::mem::transmute::<i16, i16>(value) }
    }

}

impl<'a> std::fmt::Debug for I16Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("I16")
            .field("padding", &self.padding())
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for I16Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.padding() == other.padding() &&        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for I16Ref<'a> {}

/// Mutable access to [`I16`].
///
/// [`I16`]: struct.I16.html
pub struct I16Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> I16Mut<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_padding(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 2)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 3)
    }

    #[inline]
    pub fn f(&self) -> i16 {
        let value = flatdata_read_bytes!(i16, self.data, 3, 13);
        unsafe { std::mem::transmute::<i16, i16>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: i16) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 2)
        };
        flatdata_write_bytes!(i16; value, buffer, 3, 13)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &I16Ref) {
        self.set_padding(other.padding());
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for I16Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        I16Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for I16Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`U32Ref`] for the read-only access, and
/// * [`U32Mut`] for the mutable access
///
/// to the `U32` data.
///
/// [`U32Ref`]: struct.U32Ref.html
/// [`U32Mut`]: struct.U32Mut.html
#[derive(Clone, Debug)]
pub struct U32 {}

/// Read-only access to [`U32`].
///
/// [`U32`]: struct.U32.html
#[derive(Clone, Copy)]
pub struct U32Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for U32
{
    const SCHEMA: &'static str = schema::structs::U32;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = U32Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = U32Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for U32 {}

impl<'a> U32Ref<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn f(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 3, 29);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

}

impl<'a> std::fmt::Debug for U32Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("U32")
            .field("padding", &self.padding())
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for U32Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.padding() == other.padding() &&        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for U32Ref<'a> {}

/// Mutable access to [`U32`].
///
/// [`U32`]: struct.U32.html
pub struct U32Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> U32Mut<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_padding(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 3)
    }

    #[inline]
    pub fn f(&self) -> u32 {
        let value = flatdata_read_bytes!(u32, self.data, 3, 29);
        unsafe { std::mem::transmute::<u32, u32>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: u32) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(u32; value, buffer, 3, 29)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &U32Ref) {
        self.set_padding(other.padding());
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for U32Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        U32Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for U32Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`I32Ref`] for the read-only access, and
/// * [`I32Mut`] for the mutable access
///
/// to the `I32` data.
///
/// [`I32Ref`]: struct.I32Ref.html
/// [`I32Mut`]: struct.I32Mut.html
#[derive(Clone, Debug)]
pub struct I32 {}

/// Read-only access to [`I32`].
///
/// [`I32`]: struct.I32.html
#[derive(Clone, Copy)]
pub struct I32Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for I32
{
    const SCHEMA: &'static str = schema::structs::I32;
    const SIZE_IN_BYTES: usize = 4;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = I32Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = I32Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for I32 {}

impl<'a> I32Ref<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn f(&self) -> i32 {
        let value = flatdata_read_bytes!(i32, self.data, 3, 29);
        unsafe { std::mem::transmute::<i32, i32>(value) }
    }

}

impl<'a> std::fmt::Debug for I32Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("I32")
            .field("padding", &self.padding())
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for I32Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.padding() == other.padding() &&        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for I32Ref<'a> {}

/// Mutable access to [`I32`].
///
/// [`I32`]: struct.I32.html
pub struct I32Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> I32Mut<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_padding(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 3)
    }

    #[inline]
    pub fn f(&self) -> i32 {
        let value = flatdata_read_bytes!(i32, self.data, 3, 29);
        unsafe { std::mem::transmute::<i32, i32>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: i32) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 4)
        };
        flatdata_write_bytes!(i32; value, buffer, 3, 29)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &I32Ref) {
        self.set_padding(other.padding());
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for I32Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        I32Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for I32Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`U64Ref`] for the read-only access, and
/// * [`U64Mut`] for the mutable access
///
/// to the `U64` data.
///
/// [`U64Ref`]: struct.U64Ref.html
/// [`U64Mut`]: struct.U64Mut.html
#[derive(Clone, Debug)]
pub struct U64 {}

/// Read-only access to [`U64`].
///
/// [`U64`]: struct.U64.html
#[derive(Clone, Copy)]
pub struct U64Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for U64
{
    const SCHEMA: &'static str = schema::structs::U64;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = U64Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = U64Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for U64 {}

impl<'a> U64Ref<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn f(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 3, 61);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

}

impl<'a> std::fmt::Debug for U64Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("U64")
            .field("padding", &self.padding())
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for U64Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.padding() == other.padding() &&        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for U64Ref<'a> {}

/// Mutable access to [`U64`].
///
/// [`U64`]: struct.U64.html
pub struct U64Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> U64Mut<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_padding(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 3)
    }

    #[inline]
    pub fn f(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 3, 61);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 3, 61)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &U64Ref) {
        self.set_padding(other.padding());
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for U64Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        U64Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for U64Mut<'a> {}

///
/// ## Access pattern
///
/// This structure is used as a template parameter in containers.
/// It does not contain any data, instead it references
///
/// * [`I64Ref`] for the read-only access, and
/// * [`I64Mut`] for the mutable access
///
/// to the `I64` data.
///
/// [`I64Ref`]: struct.I64Ref.html
/// [`I64Mut`]: struct.I64Mut.html
#[derive(Clone, Debug)]
pub struct I64 {}

/// Read-only access to [`I64`].
///
/// [`I64`]: struct.I64.html
#[derive(Clone, Copy)]
pub struct I64Ref<'a> {
    data: *const u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> flatdata::Struct<'a> for I64
{
    const SCHEMA: &'static str = schema::structs::I64;
    const SIZE_IN_BYTES: usize = 8;
    const IS_OVERLAPPING_WITH_NEXT : bool = false;

    type Item = I64Ref<'a>;

    #[inline]
    fn create(data : &'a[u8]) -> Self::Item
    {
        Self::Item { data : data.as_ptr(), _phantom : std::marker::PhantomData }
    }

    type ItemMut = I64Mut<'a>;

    #[inline]
    fn create_mut(data: &'a mut[u8]) -> Self::ItemMut
    {
        Self::ItemMut { data : data.as_mut_ptr(), _phantom : std::marker::PhantomData }
    }
}

impl flatdata::NoOverlap for I64 {}

impl<'a> I64Ref<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[inline]
    pub fn f(&self) -> i64 {
        let value = flatdata_read_bytes!(i64, self.data, 3, 61);
        unsafe { std::mem::transmute::<i64, i64>(value) }
    }

}

impl<'a> std::fmt::Debug for I64Ref<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("I64")
            .field("padding", &self.padding())
            .field("f", &self.f())
            .finish()
    }
}

impl<'a> std::cmp::PartialEq for I64Ref<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.padding() == other.padding() &&        self.f() == other.f()     }
}

impl<'a> flatdata::Ref for I64Ref<'a> {}

/// Mutable access to [`I64`].
///
/// [`I64`]: struct.I64.html
pub struct I64Mut<'a> {
    data: *mut u8,
    _phantom: std::marker::PhantomData<&'a u8>,
}

impl<'a> I64Mut<'a> {
    #[inline]
    pub fn padding(&self) -> u64 {
        let value = flatdata_read_bytes!(u64, self.data, 0, 3);
        unsafe { std::mem::transmute::<u64, u64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_padding(&mut self, value: u64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(u64; value, buffer, 0, 3)
    }

    #[inline]
    pub fn f(&self) -> i64 {
        let value = flatdata_read_bytes!(i64, self.data, 3, 61);
        unsafe { std::mem::transmute::<i64, i64>(value) }
    }

    #[allow(missing_docs)]
    #[inline]
    pub fn set_f(&mut self, value: i64) {
        let buffer = unsafe {
            std::slice::from_raw_parts_mut(self.data, 8)
        };
        flatdata_write_bytes!(i64; value, buffer, 3, 61)
    }


    /// Copies the data from `other` into this struct.
    #[inline]
    pub fn fill_from(&mut self, other: &I64Ref) {
        self.set_padding(other.padding());
        self.set_f(other.f());
    }
}

impl<'a> std::fmt::Debug for I64Mut<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        I64Ref { data : self.data, _phantom : std::marker::PhantomData }.fmt( f )
    }
}

impl<'a> flatdata::RefMut for I64Mut<'a> {}