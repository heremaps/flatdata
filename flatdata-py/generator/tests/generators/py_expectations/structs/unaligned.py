class n_U8(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct U8
{
    padding : u64 : 3;
    f : u8 : 5;
}
}

"""
    _SIZE_IN_BITS = 8
    _SIZE_IN_BYTES = 1
    _FIELDS = {
        "padding": flatdata.structure.FieldSignature(offset=0, width=3, is_signed=False, dtype="u8"),
        "f": flatdata.structure.FieldSignature(offset=3, width=5, is_signed=False, dtype="B"),
    }
    _FIELD_KEYS = {
        "padding",
        "f",
    }

class n_I8(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct I8
{
    padding : u64 : 3;
    f : i8 : 5;
}
}

"""
    _SIZE_IN_BITS = 8
    _SIZE_IN_BYTES = 1
    _FIELDS = {
        "padding": flatdata.structure.FieldSignature(offset=0, width=3, is_signed=False, dtype="u8"),
        "f": flatdata.structure.FieldSignature(offset=3, width=5, is_signed=True, dtype="b"),
    }
    _FIELD_KEYS = {
        "padding",
        "f",
    }

class n_U16(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct U16
{
    padding : u64 : 3;
    f : u16 : 13;
}
}

"""
    _SIZE_IN_BITS = 16
    _SIZE_IN_BYTES = 2
    _FIELDS = {
        "padding": flatdata.structure.FieldSignature(offset=0, width=3, is_signed=False, dtype="u8"),
        "f": flatdata.structure.FieldSignature(offset=3, width=13, is_signed=False, dtype="u2"),
    }
    _FIELD_KEYS = {
        "padding",
        "f",
    }

class n_I16(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct I16
{
    padding : u64 : 3;
    f : i16 : 13;
}
}

"""
    _SIZE_IN_BITS = 16
    _SIZE_IN_BYTES = 2
    _FIELDS = {
        "padding": flatdata.structure.FieldSignature(offset=0, width=3, is_signed=False, dtype="u8"),
        "f": flatdata.structure.FieldSignature(offset=3, width=13, is_signed=True, dtype="i2"),
    }
    _FIELD_KEYS = {
        "padding",
        "f",
    }

class n_U32(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct U32
{
    padding : u64 : 3;
    f : u32 : 29;
}
}

"""
    _SIZE_IN_BITS = 32
    _SIZE_IN_BYTES = 4
    _FIELDS = {
        "padding": flatdata.structure.FieldSignature(offset=0, width=3, is_signed=False, dtype="u8"),
        "f": flatdata.structure.FieldSignature(offset=3, width=29, is_signed=False, dtype="u4"),
    }
    _FIELD_KEYS = {
        "padding",
        "f",
    }

class n_I32(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct I32
{
    padding : u64 : 3;
    f : i32 : 29;
}
}

"""
    _SIZE_IN_BITS = 32
    _SIZE_IN_BYTES = 4
    _FIELDS = {
        "padding": flatdata.structure.FieldSignature(offset=0, width=3, is_signed=False, dtype="u8"),
        "f": flatdata.structure.FieldSignature(offset=3, width=29, is_signed=True, dtype="i4"),
    }
    _FIELD_KEYS = {
        "padding",
        "f",
    }

class n_U64(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct U64
{
    padding : u64 : 3;
    f : u64 : 61;
}
}

"""
    _SIZE_IN_BITS = 64
    _SIZE_IN_BYTES = 8
    _FIELDS = {
        "padding": flatdata.structure.FieldSignature(offset=0, width=3, is_signed=False, dtype="u8"),
        "f": flatdata.structure.FieldSignature(offset=3, width=61, is_signed=False, dtype="u8"),
    }
    _FIELD_KEYS = {
        "padding",
        "f",
    }

class n_I64(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct I64
{
    padding : u64 : 3;
    f : i64 : 61;
}
}

"""
    _SIZE_IN_BITS = 64
    _SIZE_IN_BYTES = 8
    _FIELDS = {
        "padding": flatdata.structure.FieldSignature(offset=0, width=3, is_signed=False, dtype="u8"),
        "f": flatdata.structure.FieldSignature(offset=3, width=61, is_signed=True, dtype="i8"),
    }
    _FIELD_KEYS = {
        "padding",
        "f",
    }
