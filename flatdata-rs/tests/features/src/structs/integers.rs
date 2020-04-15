#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/structs/integers.rs"));

#[test]
fn test_sizes() {
    use flatdata::Struct;

    assert_eq!(n::U8::SIZE_IN_BYTES, 1);
    assert_eq!(n::I8::SIZE_IN_BYTES, 1);
    assert_eq!(n::U16::SIZE_IN_BYTES, 2);
    assert_eq!(n::I16::SIZE_IN_BYTES, 2);
    assert_eq!(n::U32::SIZE_IN_BYTES, 4);
    assert_eq!(n::I32::SIZE_IN_BYTES, 4);
    assert_eq!(n::U64::SIZE_IN_BYTES, 8);
    assert_eq!(n::I64::SIZE_IN_BYTES, 8);
}

#[test]
fn test_min_values() {
    let mut value = n::U8::new();
    value.set_f(std::u8::MIN);
    assert_eq!(value.f(), std::u8::MIN);

    let mut value = n::I8::new();
    value.set_f(std::i8::MIN);
    assert_eq!(value.f(), std::i8::MIN);

    let mut value = n::U16::new();
    value.set_f(std::u16::MIN);
    assert_eq!(value.f(), std::u16::MIN);

    let mut value = n::I16::new();
    value.set_f(std::i16::MIN);
    assert_eq!(value.f(), std::i16::MIN);

    let mut value = n::U32::new();
    value.set_f(std::u32::MIN);
    assert_eq!(value.f(), std::u32::MIN);

    let mut value = n::I32::new();
    value.set_f(std::i32::MIN);
    assert_eq!(value.f(), std::i32::MIN);

    let mut value = n::U64::new();
    value.set_f(std::u64::MIN);
    assert_eq!(value.f(), std::u64::MIN);

    let mut value = n::I64::new();
    value.set_f(std::i64::MIN);
    assert_eq!(value.f(), std::i64::MIN);
}

#[test]
fn test_max_values() {
    let mut value = n::U8::new();
    value.set_f(std::u8::MAX);
    assert_eq!(value.f(), std::u8::MAX);

    let mut value = n::I8::new();
    value.set_f(std::i8::MAX);
    assert_eq!(value.f(), std::i8::MAX);

    let mut value = n::U16::new();
    value.set_f(std::u16::MAX);
    assert_eq!(value.f(), std::u16::MAX);

    let mut value = n::I16::new();
    value.set_f(std::i16::MAX);
    assert_eq!(value.f(), std::i16::MAX);

    let mut value = n::U32::new();
    value.set_f(std::u32::MAX);
    assert_eq!(value.f(), std::u32::MAX);

    let mut value = n::I32::new();
    value.set_f(std::i32::MAX);
    assert_eq!(value.f(), std::i32::MAX);

    let mut value = n::U64::new();
    value.set_f(std::u64::MAX);
    assert_eq!(value.f(), std::u64::MAX);

    let mut value = n::I64::new();
    value.set_f(std::i64::MAX);
    assert_eq!(value.f(), std::i64::MAX);
}
