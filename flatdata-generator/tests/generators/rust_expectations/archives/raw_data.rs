define_archive!(A, ABuilder,
    schema::a::A;
    // struct resources
;
    // vector resources
;
    // multivector resources
;
    // raw data resources
    (data, set_data,
        schema::a::resources::DATA, false),
    (optional_data, set_optional_data,
        schema::a::resources::OPTIONAL_DATA, true);
    // subarchives
);