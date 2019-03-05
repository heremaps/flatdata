define_struct!(
    S,
    RefS,
    RefMutS,
    schema::structs::S,
    8,
    (x, set_x, u64, u64, 0, 64));

define_archive!(A, ABuilder,
    schema::a::A;
    // struct resources
    (data, set_data,
        super::n::S,
        schema::a::resources::DATA, false),
    (optional_data, set_optional_data,
        super::n::S,
        schema::a::resources::OPTIONAL_DATA, true);
    // vector resources
;
    // multivector resources
;
    // raw data resources
;
    // subarchives
);