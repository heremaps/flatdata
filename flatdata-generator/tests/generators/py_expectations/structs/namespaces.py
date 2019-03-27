class n_Foo(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct Foo
{
    f : u32 : 32;
}
}

"""
    _SIZE_IN_BITS = 32
    _SIZE_IN_BYTES = 4
    _FIELDS = {
        "f": flatdata.structure.FieldSignature(offset=0, width=32, is_signed=False, dtype="u4"),
    }
    _FIELD_KEYS = {
        "f",
    }

class m_Foo(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace m {
struct Foo
{
    f : u32 : 32;
}
}

"""
    _SIZE_IN_BITS = 32
    _SIZE_IN_BYTES = 4
    _FIELDS = {
        "f": flatdata.structure.FieldSignature(offset=0, width=32, is_signed=False, dtype="u4"),
    }
    _FIELD_KEYS = {
        "f",
    }
