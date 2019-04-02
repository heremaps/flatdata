define_struct!(
    S,
    RefS,
    RefMutS,
    schema::structs::S,
    10,
    (x, set_x, u64, u64, 0, 64),
    (first_y, set_first_y, u32, u32, 64, 14),
    range(y_range, u32, 64, 14)
);

define_archive!(A, ABuilder,
    schema::a::A;
    // struct resources
;
    // vector resources
    (data, set_data, start_data,
        super::n::S,
        schema::a::resources::DATA, false);
    // multivector resources
;
    // raw data resources
;
    // subarchives
);