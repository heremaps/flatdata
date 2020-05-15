#![allow(dead_code)]

include!(concat!(env!("OUT_DIR"), "/constants/invalid_value.rs"));

#[test]
fn values() {
    let mut data = n::Bar::new();

    data.set_invalid_zero(10);
    assert_eq!(data.invalid_zero(), Some(10));

    data.set_invalid_zero(0);
    assert_eq!(data.invalid_zero(), None);

    data.invset_alid_min_int(10);
    assert_eq!(data.invalid_min_int(), Some(10));

    data.invset_alid_min_int(-128);
    assert_eq!(data.invalid_min_int(), None);

    data.invset_alid_max_int(10);
    assert_eq!(data.invalid_max_int(), Some(10));

    data.invset_alid_max_int(127);
    assert_eq!(data.invalid_max_int(), None);
}
