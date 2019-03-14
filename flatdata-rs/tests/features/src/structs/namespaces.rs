#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/structs/namespaces.rs"));

#[test]
fn proper_namespaces() {
    // just check that this compiles
    type X = n::Foo;
    type Y = m::Foo;
}
