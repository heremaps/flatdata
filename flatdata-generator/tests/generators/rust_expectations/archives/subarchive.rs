define_archive!(X, XBuilder,
    schema::x::X;
    // struct resources
;
    // vector resources
;
    // multivector resources
;
    // raw data resources
    (payload, set_payload,
        schema::x::resources::PAYLOAD, false);
    // subarchives
);

define_archive!(A, ABuilder,
    schema::a::A;
    // struct resources
;
    // vector resources
;
    // multivector resources
;
    // raw data resources
;
    // subarchives
    (data,
        super::n::X, super::n::XBuilder,
        schema::a::resources::DATA, false),
    (optional_data,
        super::n::X, super::n::XBuilder,
        schema::a::resources::OPTIONAL_DATA, true));