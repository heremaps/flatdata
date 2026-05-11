#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/imports/simple/main.rs"));

#[test]
fn test_imported_types_in_archive() {
    let storage = flatdata::MemoryResourceStorage::new("/imports_simple");

    let mut data = flatdata::Vector::<import_types::S>::new();
    let item = data.grow();
    item.set_x(42);
    item.set_y(100);

    let builder = app::ABuilder::new(storage.clone()).expect("Failed to create builder");
    builder
        .set_data(&data.as_view())
        .expect("Failed to set data");

    let archive = app::A::open(storage).expect("Failed to open archive");
    let data = archive.data();
    assert_eq!(data.len(), 1);
    assert_eq!(data[0].x(), 42);
    assert_eq!(data[0].y(), 100);
}
