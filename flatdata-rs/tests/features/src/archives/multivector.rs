#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/multivector.rs"));

#[test]
fn test() {
    use flatdata::{Archive, ArchiveBuilder};

    for &set_optional in &[false, true] {
        let storage = flatdata::MemoryResourceStorage::new("/my_test");

        let builder = n::ABuilder::new(storage.clone()).expect("Failed to create builder");
        let mut data = builder.start_data().expect("Failed to start data");
        for x in 0..12 {
            let mut list = data.grow().expect("Failed to add list to data");
            list.add_s().set_x(x);
            list.add_t().set_x(x);
        }
        data.close().expect("Failed to close data");

        if set_optional {
            let mut optional_data = builder
                .start_optional_data()
                .expect("Failed to start optional data");
            for _ in 0..256 {
                optional_data
                    .grow()
                    .expect("Failed to add list to optional data");
            }
            optional_data
                .close()
                .expect("Failed to close optional data");
        }

        let mut data_u64_index = builder
            .start_data_u64_index()
            .expect("Failed to start data_u64_index");
        for x in 0..65536 {
            let mut list = data_u64_index
                .grow()
                .expect("Failed to add list to data_u64_index");
            list.add_s().set_x(x);
        }
        data_u64_index.close().expect("Failed to close data");

        let archive = n::A::open(storage).expect("Failed to open archive");
        let data = archive.data();

        for x in 0..12 {
            let mut iter = data.at(x);
            match iter.next().expect("Missing item") {
                n::DataRef::S(s) => assert_eq!(s.x(), x as u64),
                _ => assert!(false, "Found wrong item"),
            }
            match iter.next().expect("Missing item") {
                n::DataRef::T(s) => assert_eq!(s.x(), x as u64),
                _ => assert!(false, "Found wrong item"),
            }
            assert!(iter.next().is_none(), "Too many items");
        }

        if set_optional {
            let optional_data = archive.optional_data().expect("Optional data not found");
            for x in 0..256 {
                let mut iter = optional_data.at(x);
                assert!(iter.next().is_none(), "Too many items");
            }
        }

        let data_u64_index = archive.data_u64_index();
        for x in 0..65536 {
            let mut iter = data_u64_index.at(x);
            match iter.next().expect("Missing item") {
                n::DataU64IndexRef::S(s) => assert_eq!(s.x(), x as u64),
                _ => assert!(false, "Found wrong item"),
            }
            assert!(iter.next().is_none(), "Too many items");
        }
    }
}
