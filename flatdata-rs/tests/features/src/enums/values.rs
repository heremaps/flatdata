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
    assert_eq!(n::EnumI8::UnknownValueMinus1 as i8, -1);
    assert_eq!(n::EnumI8::UnknownValue2 as i8, 2);

    assert_eq!(n::EnumU8::FooU8Pos as u8, 255);
    assert_eq!(n::EnumU8::FooU8Zero as u8, 0);
    assert_eq!(n::EnumU8::FooU8PosHex as u8, 0xfe);
    assert_eq!(n::EnumU8::FooU8OneHex as u8, 0x1);
    assert_eq!(n::EnumU8::UnknownValue2 as i8, 2);
}
