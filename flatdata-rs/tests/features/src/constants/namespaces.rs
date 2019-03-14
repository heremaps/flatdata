#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/constants/namespaces.rs"));

#[test]
fn values_in_namespace() {
    assert_eq!(n::FOO, 0);
    assert_eq!(m::FOO, 1);
}
