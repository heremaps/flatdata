#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/subarchive.rs"));

#[test]
fn test() {
    use flatdata::{Archive, ArchiveBuilder};

    for &set_optional in &[false, true] {
        let storage = flatdata::MemoryResourceStorage::new("/my_test");

        let builder = n::ABuilder::new(storage.clone()).expect("Failed to create builder");
        let sub_builder = builder.data().expect("Failed to open sub-builder");
        sub_builder
            .set_payload(b"My Data")
            .expect("Failed to set data");

        if set_optional {
            builder
                .optional_data()
                .expect("Failed to open optional sub-builder")
                .set_payload(b"My Other Data")
                .expect("Failed to set optional data");
        }

        let archive = n::A::open(storage).expect("Failed to open archive");
        assert_eq!(archive.data().payload(), b"My Data");
        if set_optional {
            assert_eq!(
                archive
                    .optional_data()
                    .expect("Optional data missing")
                    .payload(),
                b"My Other Data"
            );
        }
    }
}
