#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/imports/cross_namespace/main.rs"));

#[test]
fn test_cross_namespace_imported_enum() {
    let storage = flatdata::MemoryResourceStorage::new("/imports_cross_ns");

    let mut data = flatdata::Vector::<app::Entry>::new();
    let item = data.grow();
    item.set_id(7);
    item.set_kind(defs::Kind::B);

    let builder =
        app::MainBuilder::new(storage.clone()).expect("Failed to create builder");
    builder
        .set_entries(&data.as_view())
        .expect("Failed to set entries");

    let archive = app::Main::open(storage).expect("Failed to open archive");
    let entries = archive.entries();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].id(), 7);
    assert_eq!(entries[0].kind(), defs::Kind::B);
}
