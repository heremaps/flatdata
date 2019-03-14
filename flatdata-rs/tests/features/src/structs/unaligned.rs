#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/structs/unaligned.rs"));

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
    let mut buf = flatdata::StructBuf::<n::U8>::new();
    buf.get_mut().set_f(0);
    assert_eq!(buf.get().f(), 0);

    let mut buf = flatdata::StructBuf::<n::I8>::new();
    buf.get_mut().set_f(-(1 << (8 - 3 - 1)));
    assert_eq!(buf.get().f(), -(1 << (8 - 3 - 1)));

    let mut buf = flatdata::StructBuf::<n::U16>::new();
    buf.get_mut().set_f(0);
    assert_eq!(buf.get().f(), 0);

    let mut buf = flatdata::StructBuf::<n::I16>::new();
    buf.get_mut().set_f(-(1 << (16 - 3 - 1)));
    assert_eq!(buf.get().f(), -(1 << (16 - 3 - 1)));

    let mut buf = flatdata::StructBuf::<n::U32>::new();
    buf.get_mut().set_f(0);
    assert_eq!(buf.get().f(), 0);

    let mut buf = flatdata::StructBuf::<n::I32>::new();
    buf.get_mut().set_f(-(1 << (32 - 3 - 1)));
    assert_eq!(buf.get().f(), -(1 << (32 - 3 - 1)));

    let mut buf = flatdata::StructBuf::<n::U64>::new();
    buf.get_mut().set_f(0);
    assert_eq!(buf.get().f(), 0);

    let mut buf = flatdata::StructBuf::<n::I64>::new();
    buf.get_mut().set_f(-(1 << (64 - 3 - 1)));
    assert_eq!(buf.get().f(), -(1 << (64 - 3 - 1)));
}

#[test]
fn test_max_values() {
    let mut buf = flatdata::StructBuf::<n::U8>::new();
    buf.get_mut().set_f((1 << (8 - 3)) - 1);
    assert_eq!(buf.get().f(), (1 << (8 - 3)) - 1);

    let mut buf = flatdata::StructBuf::<n::I8>::new();
    buf.get_mut().set_f((1 << (8 - 3) - 1) - 1);
    assert_eq!(buf.get().f(), (1 << (8 - 3 - 1)) - 1);

    let mut buf = flatdata::StructBuf::<n::U16>::new();
    buf.get_mut().set_f((1 << (16 - 3)) - 1);
    assert_eq!(buf.get().f(), (1 << (16 - 3)) - 1);

    let mut buf = flatdata::StructBuf::<n::I16>::new();
    buf.get_mut().set_f((1 << (16 - 3 - 1)) - 1);
    assert_eq!(buf.get().f(), (1 << (16 - 3 - 1)) - 1);

    let mut buf = flatdata::StructBuf::<n::U32>::new();
    buf.get_mut().set_f((1 << (32 - 3)) - 1);
    assert_eq!(buf.get().f(), (1 << (32 - 3)) - 1);

    let mut buf = flatdata::StructBuf::<n::I32>::new();
    buf.get_mut().set_f((1 << (32 - 3 - 1)) - 1);
    assert_eq!(buf.get().f(), (1 << (32 - 3 - 1)) - 1);

    let mut buf = flatdata::StructBuf::<n::U64>::new();
    buf.get_mut().set_f((1 << (64 - 3)) - 1);
    assert_eq!(buf.get().f(), (1 << (64 - 3)) - 1);

    let mut buf = flatdata::StructBuf::<n::I64>::new();
    buf.get_mut().set_f((1 << (64 - 3 - 1)) - 1);
    assert_eq!(buf.get().f(), (1 << (64 - 3 - 1)) - 1);
}
