#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/archives/references.rs"));

#[test]
fn test() {
    use flatdata::{Archive, ArchiveBuilder, Vector};

    let create_archive = |sizes: &[usize]| {
        let storage = flatdata::MemoryResourceStorage::new("/my_test");
        let builder = n::ABuilder::new(storage.clone()).unwrap();
        builder.set_list1(&Vector::with_len(sizes[0])).unwrap();
        builder.set_list2(&Vector::with_len(sizes[1])).unwrap();
        builder.set_raw1(&vec![0; sizes[2]]).unwrap();
        builder.set_raw2(&vec![0; sizes[3]]).unwrap();
        let mut ml = builder
            .start_multilist1()
            .expect("Failed to create MultiVector");
        for _ in 0..sizes[4] {
            ml.grow().unwrap();
        }
        ml.close().unwrap();
        let mut ml = builder
            .start_multilist2()
            .expect("Failed to create MultiVector");
        for _ in 0..sizes[5] {
            ml.grow().unwrap();
        }
        ml.close().unwrap();
        builder
            .start_refs()
            .expect("Failed to create MultiVector")
            .close()
            .unwrap();
        builder
            .start_multirefs()
            .expect("Failed to create MultiVector")
            .close()
            .unwrap();
        storage
    };

    let storage = create_archive(&[16, 16, 16, 16, 16, 16]);
    n::A::open(storage).expect("Failed to open archive even though sizes fit");
    for pos in 0..6 {
        let mut sizes = vec![16; 6];
        sizes[pos] = 17;
        let storage = create_archive(&sizes);
        assert!(n::A::open(storage).is_err(), "Failed for resource {}", pos);
    }
}
