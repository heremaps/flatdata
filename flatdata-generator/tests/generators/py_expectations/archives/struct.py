class n_S(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

"""
    _NAME = "n_S"
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
    data : .n.S;
    @optional
    optional_data : .n.S;
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
    data : .n.S;
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
    optional_data : .n.S;
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
        "data": flatdata.archive.ResourceSignature(container=flatdata.resources.Instance,
            initializer=n_S,
            schema=_DATA_SCHEMA,
            is_optional=False,
            doc=_DATA_DOC),
        "optional_data": flatdata.archive.ResourceSignature(container=flatdata.resources.Instance,
            initializer=n_S,
            schema=_OPTIONAL_DATA_SCHEMA,
            is_optional=True,
            doc=_OPTIONAL_DATA_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)

class n_ABuilder(flatdata.archive_builder.ArchiveBuilder):
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : .n.S;
    @optional
    optional_data : .n.S;
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
    data : .n.S;
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
    optional_data : .n.S;
}
}

"""
    _OPTIONAL_DATA_DOC = """"""
    _NAME = "A"
    _RESOURCES = {
        "A.archive" : flatdata.archive_builder.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "data": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.Instance,
            initializer=n_S,
            schema=_DATA_SCHEMA,
            is_optional=False,
            doc=_DATA_DOC),
        "optional_data": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.Instance,
            initializer=n_S,
            schema=_OPTIONAL_DATA_SCHEMA,
            is_optional=True,
            doc=_OPTIONAL_DATA_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive_builder.ArchiveBuilder.__init__(self, resource_storage)
