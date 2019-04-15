class n_S(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
    @range( y_range )
    first_y : u32 : 14;
}
}

"""
    _SIZE_IN_BITS = 78
    _SIZE_IN_BYTES = 10
    _FIELDS = {
        "x": flatdata.structure.FieldSignature(offset=0, width=64, is_signed=False, dtype="u8"),
        "first_y": flatdata.structure.FieldSignature(offset=64, width=14, is_signed=False, dtype="u4"),
    }
    _FIELD_KEYS = {
        "x",
        "first_y",
    }
