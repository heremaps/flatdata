#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/enums/values.rs"));

#[test]
fn values() {
    assert_eq!(n::EnumI8::FooI8Neg as i8, -128);
    assert_eq!(n::EnumI8::FooI8Pos as i8, 127);
    assert_eq!(n::EnumI8::FooI8Zero as i8, 0);
    assert_eq!(n::EnumI8::FooI8NegHex as i8, -0x7f);
    assert_eq!(n::EnumI8::FooI8PosHex as i8, 0x7e);
    assert_eq!(n::EnumI8::FooI8OneHex as i8, 0x1);

    assert_eq!(n::EnumU8::FooU8Pos as u8, 255);
    assert_eq!(n::EnumU8::FooU8Zero as u8, 0);
    assert_eq!(n::EnumU8::FooU8PosHex as u8, 0xfe);
    assert_eq!(n::EnumU8::FooU8OneHex as u8, 0x1);

    assert_eq!(n::EnumI16::FooI16Neg as i16, -32768);
    assert_eq!(n::EnumI16::FooI16Pos as i16, 32767);
    assert_eq!(n::EnumI16::FooI16Zero as i16, 0);
    assert_eq!(n::EnumI16::FooI16NegHex as i16, -0x7fff);
    assert_eq!(n::EnumI16::FooI16PosHex as i16, 0x7ffe);
    assert_eq!(n::EnumI16::FooI16OneHex as i16, 0x1);

    assert_eq!(n::EnumU16::FooU16Pos as u16, 65535);
    assert_eq!(n::EnumU16::FooU16Zero as u16, 0);
    assert_eq!(n::EnumU16::FooU16PosHex as u16, 0xfffe);
    assert_eq!(n::EnumU16::FooU16OneHex as u16, 0x1);

    assert_eq!(n::EnumI32::FooI32Neg as i32, -2147483648);
    assert_eq!(n::EnumI32::FooI32Pos as i32, 2147483647);
    assert_eq!(n::EnumI32::FooI32Zero as i32, 0);
    assert_eq!(n::EnumI32::FooI32NegHex as i32, -0x7fffffff);
    assert_eq!(n::EnumI32::FooI32PosHex as i32, 0x7ffffffe);
    assert_eq!(n::EnumI32::FooI32OneHex as i32, 0x1);

    assert_eq!(n::EnumU32::FooU32Pos as u32, 4294967295);
    assert_eq!(n::EnumU32::FooU32Zero as u32, 0);
    assert_eq!(n::EnumU32::FooU32PosHex as u32, 0xfffffffe);
    assert_eq!(n::EnumU32::FooU32OneHex as u32, 0x1);

    assert_eq!(n::EnumI64::FooI64Neg as i64, -9223372036854775808);
    assert_eq!(n::EnumI64::FooI64Pos as i64, 9223372036854775807);
    assert_eq!(n::EnumI64::FooI64Zero as i64, 0);
    assert_eq!(n::EnumI64::FooI64NegHex as i64, -0x7fffffffffffffff);
    assert_eq!(n::EnumI64::FooI64PosHex as i64, 0x7ffffffffffffffe);
    assert_eq!(n::EnumI64::FooI64OneHex as i64, 0x1);

    assert_eq!(n::EnumU64::FooU64Pos as u64, 18446744073709551615);
    assert_eq!(n::EnumU64::FooU64Zero as u64, 0);
    assert_eq!(n::EnumU64::FooU64PosHex as u64, 0xfffffffffffffffe);
    assert_eq!(n::EnumU64::FooU64OneHex as u64, 0x1);
}
