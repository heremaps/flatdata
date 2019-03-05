#[derive(Debug, PartialEq, Eq)]
#[repr(i8)]
pub enum EnumI8 {
    FooI8Neg = -128,
    FooI8Pos = 127,
    FooI8Zero = 0,
    FooI8NegHex = -127,
    FooI8PosHex = 126,
    FooI8OneHex = 1,
}

impl Int for EnumI8 {
    const IS_SIGNED: bool = true;
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EnumU8 {
    FooU8Pos = 255,
    FooU8Zero = 0,
    FooU8PosHex = 254,
    FooU8OneHex = 1,
}

impl Int for EnumU8 {
    const IS_SIGNED: bool = false;
}

#[derive(Debug, PartialEq, Eq)]
#[repr(i16)]
pub enum EnumI16 {
    FooI16Neg = -32768,
    FooI16Pos = 32767,
    FooI16Zero = 0,
    FooI16NegHex = -32767,
    FooI16PosHex = 32766,
    FooI16OneHex = 1,
}

impl Int for EnumI16 {
    const IS_SIGNED: bool = true;
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum EnumU16 {
    FooU16Pos = 65535,
    FooU16Zero = 0,
    FooU16PosHex = 65534,
    FooU16OneHex = 1,
}

impl Int for EnumU16 {
    const IS_SIGNED: bool = false;
}

#[derive(Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum EnumI32 {
    FooI32Neg = -2147483648,
    FooI32Pos = 2147483647,
    FooI32Zero = 0,
    FooI32NegHex = -2147483647,
    FooI32PosHex = 2147483646,
    FooI32OneHex = 1,
}

impl Int for EnumI32 {
    const IS_SIGNED: bool = true;
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum EnumU32 {
    FooU32Pos = 4294967295,
    FooU32Zero = 0,
    FooU32PosHex = 4294967294,
    FooU32OneHex = 1,
}

impl Int for EnumU32 {
    const IS_SIGNED: bool = false;
}

#[derive(Debug, PartialEq, Eq)]
#[repr(i64)]
pub enum EnumI64 {
    FooI64Neg = -9223372036854775808,
    FooI64Pos = 9223372036854775807,
    FooI64Zero = 0,
    FooI64NegHex = -9223372036854775807,
    FooI64PosHex = 9223372036854775806,
    FooI64OneHex = 1,
}

impl Int for EnumI64 {
    const IS_SIGNED: bool = true;
}

#[derive(Debug, PartialEq, Eq)]
#[repr(u64)]
pub enum EnumU64 {
    FooU64Pos = 18446744073709551615,
    FooU64Zero = 0,
    FooU64PosHex = 18446744073709551614,
    FooU64OneHex = 1,
}

impl Int for EnumU64 {
    const IS_SIGNED: bool = false;
}