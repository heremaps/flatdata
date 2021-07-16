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
    _NAME = "n_S"
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
class n_A(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
    @range( y_range )
    first_y : u32 : 14;
}
}

namespace n {
archive A
{
    data : vector< .n.S >;
}
}

"""
    _DATA_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
    @range( y_range )
    first_y : u32 : 14;
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
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)

class n_ABuilder(flatdata.archive_builder.ArchiveBuilder):
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
    @range( y_range )
    first_y : u32 : 14;
}
}

namespace n {
archive A
{
    data : vector< .n.S >;
}
}

"""
    _DATA_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
    @range( y_range )
    first_y : u32 : 14;
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
    _NAME = "A"
    _RESOURCES = {
        "A.archive" : flatdata.archive_builder.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "data": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.Vector,
            initializer=n_S,
            schema=_DATA_SCHEMA,
            is_optional=False,
            doc=_DATA_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive_builder.ArchiveBuilder.__init__(self, resource_storage)
