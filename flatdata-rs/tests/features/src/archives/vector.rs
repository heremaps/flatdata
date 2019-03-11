#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/vector.rs"));

#[test]
fn test() {
    use flatdata::Archive;
    use flatdata::ArchiveBuilder;

    for &set_optional in &[false, true] {
        let storage = flatdata::MemoryResourceStorage::new("/my_test");

        let mut data = flatdata::Vector::<n::S>::new();
        for x in 10..600 {
            data.grow().set_x(x);
        }

        let builder = n::ABuilder::new(storage.clone()).expect("Failed to create builder");
        builder
            .set_data(&data.as_view())
            .expect("Failed to set data");

        if set_optional {
            let mut optional_data = builder
                .start_optional_data()
                .expect("Failed to start optional data");
            for x in 10..600 {
                optional_data
                    .grow()
                    .expect("Failed to grow optional data")
                    .set_x(x);
            }
            optional_data
                .close()
                .expect("Failed to close optional data");
        }

        let archive = n::A::open(storage).expect("Failed to open archive");
        let data = archive.data();
        for x in (10..600).enumerate() {
            assert_eq!(data.at(x.0).x(), x.1);
        }
        if set_optional {
            let data = archive.optional_data().expect("Optional data not found");
            for x in (10..600).enumerate() {
                assert_eq!(data.at(x.0).x(), x.1);
            }
        }
    }
}
