#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/enums/structs.rs"));

#[test]
fn enums_in_structs() {
    let mut buf = flatdata::StructBuf::<n::StructEnumI8>::new();
    buf.get_mut().set_f(n::EnumI8::Value);
    assert_eq!(buf.get().f(), n::EnumI8::Value);

    let mut buf = flatdata::StructBuf::<n::StructEnumU8>::new();
    buf.get_mut().set_f(n::EnumU8::Value);
    assert_eq!(buf.get().f(), n::EnumU8::Value);

    let mut buf = flatdata::StructBuf::<n::StructEnumI16>::new();
    buf.get_mut().set_f(n::EnumI16::Value);
    assert_eq!(buf.get().f(), n::EnumI16::Value);

    let mut buf = flatdata::StructBuf::<n::StructEnumU16>::new();
    buf.get_mut().set_f(n::EnumU16::Value);
    assert_eq!(buf.get().f(), n::EnumU16::Value);

    let mut buf = flatdata::StructBuf::<n::StructEnumI32>::new();
    buf.get_mut().set_f(n::EnumI32::Value);
    assert_eq!(buf.get().f(), n::EnumI32::Value);

    let mut buf = flatdata::StructBuf::<n::StructEnumU32>::new();
    buf.get_mut().set_f(n::EnumU32::Value);
    assert_eq!(buf.get().f(), n::EnumU32::Value);

    let mut buf = flatdata::StructBuf::<n::StructEnumI64>::new();
    buf.get_mut().set_f(n::EnumI64::Value);
    assert_eq!(buf.get().f(), n::EnumI64::Value);

    let mut buf = flatdata::StructBuf::<n::StructEnumU64>::new();
    buf.get_mut().set_f(n::EnumU64::Value);
    assert_eq!(buf.get().f(), n::EnumU64::Value);
}
