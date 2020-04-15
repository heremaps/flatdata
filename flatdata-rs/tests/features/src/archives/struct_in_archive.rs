#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/struct.rs"));

#[test]
fn test() {
    use flatdata::{Archive, ArchiveBuilder};

    for &set_optional in &[false, true] {
        let storage = flatdata::MemoryResourceStorage::new("/my_test");

        let builder = n::ABuilder::new(storage.clone()).expect("Failed to create builder");
        let mut data = n::S::new();
        data.set_x(14);
        builder.set_data(&data).expect("Failed to set data");

        if set_optional {
            let mut optional_data = n::S::new();
            optional_data.set_x(16);
            builder
                .set_optional_data(&optional_data)
                .expect("Failed to set optional data");
        }

        let archive = n::A::open(storage).expect("Failed to open archive");
        assert_eq!(archive.data().x(), 14);
        if set_optional {
            assert_eq!(
                archive.optional_data().expect("Optional data missing").x(),
                16
            );
        }
    }
}
