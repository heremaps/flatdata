define_struct!(
    S,
    RefS,
    RefMutS,
    schema::structs::S,
    8,
    (x, set_x, u64, u64, 0, 64));

define_archive!(A, ABuilder, schema::a::A;
    struct(data, false, schema::a::resources::DATA, set_data, super::n::S),
    struct(optional_data, true, schema::a::resources::OPTIONAL_DATA, set_optional_data, super::n::S),
);
