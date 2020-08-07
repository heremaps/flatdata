#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/empty.rs"));

#[cfg(test)]
fn is_sync<T: std::marker::Sync>(_t: T) {}

#[test]
fn test() {
    let storage = flatdata::MemoryResourceStorage::new("/my_test");

    let builder = n::ABuilder::new(storage.clone()).expect("Failed to create builder");
    std::mem::drop(builder);

    let archive = n::A::open(storage).expect("Failed to open archive");

    is_sync(archive);
}
