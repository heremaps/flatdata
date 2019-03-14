#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/enums/namespaces.rs"));

#[test]
fn values_in_namespaces() {
    assert_eq!(a::Bar::Value as i32, 0);
    assert_eq!(b::Bar::Value as i32, 0);

    let a_buf = flatdata::StructBuf::<n::Foo>::new();
    assert_eq!(a_buf.get().f(), a::Bar::Value);

    let b_buf = flatdata::StructBuf::<m::Foo>::new();
    assert_eq!(b_buf.get().f(), b::Bar::Value);
}
