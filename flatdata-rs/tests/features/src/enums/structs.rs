#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/enums/structs.rs"));

#[test]
fn enums_in_structs() {
    let mut value = n::StructEnumI8::new();
    value.set_f(n::EnumI8::Value);
    assert_eq!(value.f(), n::EnumI8::Value);

    let mut value = n::StructEnumU8::new();
    value.set_f(n::EnumU8::Value);
    assert_eq!(value.f(), n::EnumU8::Value);

    let mut value = n::StructEnumI16::new();
    value.set_f(n::EnumI16::Value);
    assert_eq!(value.f(), n::EnumI16::Value);

    let mut value = n::StructEnumU16::new();
    value.set_f(n::EnumU16::Value);
    assert_eq!(value.f(), n::EnumU16::Value);

    let mut value = n::StructEnumI32::new();
    value.set_f(n::EnumI32::Value);
    assert_eq!(value.f(), n::EnumI32::Value);

    let mut value = n::StructEnumU32::new();
    value.set_f(n::EnumU32::Value);
    assert_eq!(value.f(), n::EnumU32::Value);

    let mut value = n::StructEnumI64::new();
    value.set_f(n::EnumI64::Value);
    assert_eq!(value.f(), n::EnumI64::Value);

    let mut value = n::StructEnumU64::new();
    value.set_f(n::EnumU64::Value);
    assert_eq!(value.f(), n::EnumU64::Value);
}
