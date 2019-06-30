#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/raw_data.rs"));

#[test]
fn test() {
    use flatdata::{Archive, ArchiveBuilder};

    for &set_optional in &[false, true] {
        let storage = flatdata::MemoryResourceStorage::new("/my_test");

        let builder = n::ABuilder::new(storage.clone()).expect("Failed to create builder");
        builder.set_data(b"My Data").expect("Failed to set data");

        if set_optional {
            builder
                .set_optional_data(b"My Other Data")
                .expect("Failed to set optional data");
        }

        let archive = n::A::open(storage).expect("Failed to open archive");
        assert_eq!(archive.data().as_bytes(), b"My Data");
        if set_optional {
            assert_eq!(
                archive
                    .optional_data()
                    .expect("Optional data missing")
                    .as_bytes(),
                b"My Other Data"
            );
        }
    }
}
