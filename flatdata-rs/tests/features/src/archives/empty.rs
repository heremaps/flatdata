#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/empty.rs"));

#[test]
fn test() {
    use flatdata::Archive;
    use flatdata::ArchiveBuilder;

    let storage = flatdata::MemoryResourceStorage::new("/my_test");

    let _builder = n::ABuilder::new(storage.clone()).expect("Failed to create builder");

    let _archive = n::A::open(storage).expect("Failed to open archive");
}
