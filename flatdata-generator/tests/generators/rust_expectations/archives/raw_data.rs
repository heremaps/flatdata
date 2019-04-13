define_archive!(A, ABuilder, schema::a::A;
    raw_data(data, false, schema::a::resources::DATA, set_data),
    raw_data(optional_data, true, schema::a::resources::OPTIONAL_DATA, set_optional_data),
);
