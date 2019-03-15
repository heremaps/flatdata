class n_S(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

"""
    _SIZE_IN_BITS = 64
    _SIZE_IN_BYTES = 8
    _FIELDS = {
        "x": flatdata.structure.FieldSignature(offset=0, width=64, is_signed=False, dtype="u8"),
    }
    _FIELD_KEYS = {
        "x",
    }
class n_A(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : vector< .n.S >;
    @optional
    optional_data : vector< .n.S >;
}
}

"""
    _DATA_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : vector< .n.S >;
}
}

"""
    _DATA_DOC = """"""
    _OPTIONAL_DATA_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    @optional
    optional_data : vector< .n.S >;
}
}

"""
    _OPTIONAL_DATA_DOC = """"""
    _NAME = "A"
    _RESOURCES = {
        "A.archive" : flatdata.archive.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "data": flatdata.archive.ResourceSignature(container=flatdata.resources.Vector,
            initializer=n_S,
            schema=_DATA_SCHEMA,
            is_optional=False,
            doc=_DATA_DOC),
        "optional_data": flatdata.archive.ResourceSignature(container=flatdata.resources.Vector,
            initializer=n_S,
            schema=_OPTIONAL_DATA_SCHEMA,
            is_optional=True,
            doc=_OPTIONAL_DATA_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)