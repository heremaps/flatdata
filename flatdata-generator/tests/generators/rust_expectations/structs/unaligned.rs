define_struct!(
    U8,
    RefU8,
    RefMutU8,
    schema::structs::U8,
    1,
    (padding, set_padding, u64, u64, 0, 3),
    (f, set_f, u8, u8, 3, 5));

define_struct!(
    I8,
    RefI8,
    RefMutI8,
    schema::structs::I8,
    1,
    (padding, set_padding, u64, u64, 0, 3),
    (f, set_f, i8, i8, 3, 5));

define_struct!(
    U16,
    RefU16,
    RefMutU16,
    schema::structs::U16,
    2,
    (padding, set_padding, u64, u64, 0, 3),
    (f, set_f, u16, u16, 3, 13));

define_struct!(
    I16,
    RefI16,
    RefMutI16,
    schema::structs::I16,
    2,
    (padding, set_padding, u64, u64, 0, 3),
    (f, set_f, i16, i16, 3, 13));

define_struct!(
    U32,
    RefU32,
    RefMutU32,
    schema::structs::U32,
    4,
    (padding, set_padding, u64, u64, 0, 3),
    (f, set_f, u32, u32, 3, 29));

define_struct!(
    I32,
    RefI32,
    RefMutI32,
    schema::structs::I32,
    4,
    (padding, set_padding, u64, u64, 0, 3),
    (f, set_f, i32, i32, 3, 29));

define_struct!(
    U64,
    RefU64,
    RefMutU64,
    schema::structs::U64,
    8,
    (padding, set_padding, u64, u64, 0, 3),
    (f, set_f, u64, u64, 3, 61));

define_struct!(
    I64,
    RefI64,
    RefMutI64,
    schema::structs::I64,
    8,
    (padding, set_padding, u64, u64, 0, 3),
    (f, set_f, i64, i64, 3, 61));