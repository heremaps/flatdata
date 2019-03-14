#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/namespaces.rs"));

#[test]
fn compilation() {
    // just check that symbols exists and everything compiles
    type A = a::A;
}
