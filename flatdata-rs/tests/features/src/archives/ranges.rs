#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/ranges.rs"));

#[test]
fn test() {
    use flatdata::Archive;
    use flatdata::ArchiveBuilder;

    let storage = flatdata::MemoryResourceStorage::new("/my_test");

    let mut data = flatdata::Vector::<n::S>::new();
    for x in 10..600 {
        let next = data.grow();
        next.set_x(x);
        next.set_first_y(x as u32 * 10);
    }
    assert_eq!(600 - 10, data.len());

    let builder = n::ABuilder::new(storage.clone()).expect("Failed to create builder");
    builder
        .set_data(&data.as_view())
        .expect("Failed to set data");

    let archive = n::A::open(storage).expect("Failed to open archive");
    let data = archive.data();
    assert_eq!(599 - 10, data.len());
    for x in (10..599).enumerate() {
        assert_eq!(data[x.0].x(), x.1);
        assert_eq!(
            data[x.0].y_range(),
            (x.1 as u32 * 10..(x.1 as u32 + 1) * 10)
        );
    }
}
