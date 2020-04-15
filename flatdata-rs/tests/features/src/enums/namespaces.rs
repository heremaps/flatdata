#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/enums/namespaces.rs"));

#[test]
fn values_in_namespaces() {
    assert_eq!(a::Bar::Value as i32, 0);
    assert_eq!(b::Bar::Value as i32, 0);

    let a = n::Foo::new();
    assert_eq!(a.f(), a::Bar::Value);

    let b = m::Foo::new();
    assert_eq!(b.f(), b::Bar::Value);
}
