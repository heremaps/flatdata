#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/constants/values.rs"));

#[test]
fn values() {
    assert_eq!(n::FOO_I8_NEG, -128);
    assert_eq!(n::FOO_I8_POS, 127);
    assert_eq!(n::FOO_I8_ZERO, 0);
    assert_eq!(n::FOO_I8_NEG_HEX, -0x80);
    assert_eq!(n::FOO_I8_POS_HEX, 0x7f);
    assert_eq!(n::FOO_I8_ZERO_HEX, 0x0);
    assert_eq!(n::FOO_U8_POS, 255);
    assert_eq!(n::FOO_U8_ZERO, 0);
    assert_eq!(n::FOO_U8_POS_HEX, 0xff);
    assert_eq!(n::FOO_U8_ZERO_HEX, 0x0);

    assert_eq!(n::FOO_I16_NEG, -32768);
    assert_eq!(n::FOO_I16_POS, 32767);
    assert_eq!(n::FOO_I16_ZERO, 0);
    assert_eq!(n::FOO_I16_NEG_HEX, -0x8000);
    assert_eq!(n::FOO_I16_POS_HEX, 0x7fff);
    assert_eq!(n::FOO_I16_ZERO_HEX, 0x0);
    assert_eq!(n::FOO_U16_POS, 65535);
    assert_eq!(n::FOO_U16_ZERO, 0);
    assert_eq!(n::FOO_U16_POS_HEX, 0xffff);
    assert_eq!(n::FOO_U16_ZERO_HEX, 0x0);

    assert_eq!(n::FOO_I32_NEG, -2147483648);
    assert_eq!(n::FOO_I32_POS, 2147483647);
    assert_eq!(n::FOO_I32_ZERO, 0);
    assert_eq!(n::FOO_I32_NEG_HEX, -0x80000000);
    assert_eq!(n::FOO_I32_POS_HEX, 0x7fffffff);
    assert_eq!(n::FOO_I32_ZERO_HEX, 0x0);
    assert_eq!(n::FOO_U32_POS, 4294967295);
    assert_eq!(n::FOO_U32_ZERO, 0);
    assert_eq!(n::FOO_U32_POS_HEX, 0xffffffff);
    assert_eq!(n::FOO_U32_ZERO_HEX, 0x0);

    assert_eq!(n::FOO_I64_NEG, -9223372036854775808);
    assert_eq!(n::FOO_I64_POS, 9223372036854775807);
    assert_eq!(n::FOO_I64_ZERO, 0);
    assert_eq!(n::FOO_I64_NEG_HEX, -0x8000000000000000);
    assert_eq!(n::FOO_I64_POS_HEX, 0x7fffffffffffffff);
    assert_eq!(n::FOO_I64_ZERO_HEX, 0x0);
    assert_eq!(n::FOO_U64_POS, 18446744073709551615);
    assert_eq!(n::FOO_U64_ZERO, 0);
    assert_eq!(n::FOO_U64_POS_HEX, 0xffffffffffffffff);
    assert_eq!(n::FOO_U64_ZERO_HEX, 0x0);
}
