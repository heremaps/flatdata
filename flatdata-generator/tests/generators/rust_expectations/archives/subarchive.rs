define_archive!(X, XBuilder, schema::x::X;
    raw_data(payload, false, schema::x::resources::PAYLOAD, set_payload),
);

define_archive!(A, ABuilder, schema::a::A;
    archive(data, false, schema::a::resources::DATA, super::n::X, super::n::XBuilder),
    archive(optional_data, true, schema::a::resources::OPTIONAL_DATA, super::n::X, super::n::XBuilder),
);
