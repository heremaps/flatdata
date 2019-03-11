define_struct!(
    S,
    RefS,
    RefMutS,
    schema::structs::S,
    8,
    (x, set_x, u64, u64, 0, 64));

define_struct!(
    T,
    RefT,
    RefMutT,
    schema::structs::T,
    8,
    (x, set_x, u64, u64, 0, 64));

/// Builtin union type of .n.S, .n.T.
define_variadic_struct!(Data, RefData, BuilderData,
    IndexType8,
    0 => ( S, super::n::S, add_s),
    1 => ( T, super::n::T, add_t));

/// Builtin union type of .n.S, .n.T.
define_variadic_struct!(OptionalData, RefOptionalData, BuilderOptionalData,
    IndexType16,
    0 => ( S, super::n::S, add_s),
    1 => ( T, super::n::T, add_t));

/// Builtin union type of .n.S, .n.T.
define_variadic_struct!(DataU64Index, RefDataU64Index, BuilderDataU64Index,
    IndexType64,
    0 => ( S, super::n::S, add_s),
    1 => ( T, super::n::T, add_t));

define_archive!(A, ABuilder,
    schema::a::A;
    // struct resources
;
    // vector resources
;
    // multivector resources
    (data, start_data,
        Data,
        schema::a::resources::DATA,
        data_index, super::_builtin::multivector::IndexType8, false),
    (optional_data, start_optional_data,
        OptionalData,
        schema::a::resources::OPTIONAL_DATA,
        optional_data_index, super::_builtin::multivector::IndexType16, true),
    (data_u64_index, start_data_u64_index,
        DataU64Index,
        schema::a::resources::DATA_U64_INDEX,
        data_u64_index_index, super::_builtin::multivector::IndexType64, false);
    // raw data resources
;
    // subarchives
);

}

pub mod _builtin {

pub mod multivector {

pub mod schema {
pub mod structs {
pub const INDEX_TYPE8: &str = r#""#;
pub const INDEX_TYPE16: &str = r#""#;
pub const INDEX_TYPE64: &str = r#""#;}}
/// Builtin type to for MultiVector index
define_index!(
    IndexType8,
    RefIndexType8,
    RefMutIndexType8,
    schema::structs::INDEX_TYPE8,
    1,
    8
);


/// Builtin type to for MultiVector index
define_index!(
    IndexType16,
    RefIndexType16,
    RefMutIndexType16,
    schema::structs::INDEX_TYPE16,
    2,
    16
);


/// Builtin type to for MultiVector index
define_index!(
    IndexType64,
    RefIndexType64,
    RefMutIndexType64,
    schema::structs::INDEX_TYPE64,
    8,
    64
);