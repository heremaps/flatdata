#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/subarchive.rs"));

use flatdata::ResourceStorage;

#[test]
fn test() {
    for &(set_optional, broken) in &[(false, false), (true, false), (true, true)] {
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

            if broken {
                let mut stream = storage
                    .create_output_stream("/my_test/optional_data/payload.schema")
                    .expect("Failed to overwrite schema");
                let _ = stream
                    .write(b"broken stuff")
                    .expect("Failed to overwrite schema");
                stream.flush().expect("Failed to overwrite schema");
            }
        }

        if broken {
            n::A::open(storage).expect_err("Should have failed to open archive");
            continue;
        }
        let archive = n::A::open(storage).expect("Failed to open archive");
        assert_eq!(archive.data().payload().as_bytes(), b"My Data");
        if set_optional {
            assert_eq!(
                archive
                    .optional_data()
                    .expect("Optional data missing")
                    .payload()
                    .as_bytes(),
                b"My Other Data"
            );
        }
    }
}
