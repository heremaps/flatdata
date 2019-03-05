define_struct!(
    U8,
    RefU8,
    RefMutU8,
    schema::structs::U8,
    1,
    (f, set_f, u8, u8, 0, 8));

define_struct!(
    I8,
    RefI8,
    RefMutI8,
    schema::structs::I8,
    1,
    (f, set_f, i8, i8, 0, 8));

define_struct!(
    U16,
    RefU16,
    RefMutU16,
    schema::structs::U16,
    2,
    (f, set_f, u16, u16, 0, 16));

define_struct!(
    I16,
    RefI16,
    RefMutI16,
    schema::structs::I16,
    2,
    (f, set_f, i16, i16, 0, 16));

define_struct!(
    U32,
    RefU32,
    RefMutU32,
    schema::structs::U32,
    4,
    (f, set_f, u32, u32, 0, 32));

define_struct!(
    I32,
    RefI32,
    RefMutI32,
    schema::structs::I32,
    4,
    (f, set_f, i32, i32, 0, 32));

define_struct!(
    U64,
    RefU64,
    RefMutU64,
    schema::structs::U64,
    8,
    (f, set_f, u64, u64, 0, 64));

define_struct!(
    I64,
    RefI64,
    RefMutI64,
    schema::structs::I64,
    8,
    (f, set_f, i64, i64, 0, 64));
