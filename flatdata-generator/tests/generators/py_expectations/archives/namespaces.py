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
class n_X(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
archive X
{
    payload : raw_data;
}
}

"""
    _PAYLOAD_SCHEMA = """namespace n {
archive X
{
    payload : raw_data;
}
}

"""
    _PAYLOAD_DOC = """"""
    _NAME = "X"
    _RESOURCES = {
        "X.archive" : flatdata.archive.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "payload": flatdata.archive.ResourceSignature(container=flatdata.resources.RawData,
            initializer=None,
            schema=_PAYLOAD_SCHEMA,
            is_optional=False,
            doc=_PAYLOAD_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)

class n_XBuilder(flatdata.archive_builder.ArchiveBuilder):
    _SCHEMA = """namespace n {
archive X
{
    payload : raw_data;
}
}

"""
    _PAYLOAD_SCHEMA = """namespace n {
archive X
{
    payload : raw_data;
}
}

"""
    _PAYLOAD_DOC = """"""
    _NAME = "X"
    _RESOURCES = {
        "X.archive" : flatdata.archive_builder.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "payload": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.RawData,
            initializer=None,
            schema=_PAYLOAD_SCHEMA,
            is_optional=False,
            doc=_PAYLOAD_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive_builder.ArchiveBuilder.__init__(self, resource_storage)


class m_S(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace m {
struct S
{
    x : u64 : 64;
}
}

"""
    _NAME = "m_S"
    _SIZE_IN_BITS = 64
    _SIZE_IN_BYTES = 8
    _FIELDS = {
        "x": flatdata.structure.FieldSignature(offset=0, width=64, is_signed=False, dtype="u8"),
    }
    _FIELD_KEYS = {
        "x",
    }
class m_X(flatdata.archive.Archive):
    _SCHEMA = """namespace m {
archive X
{
    payload : raw_data;
}
}

"""
    _PAYLOAD_SCHEMA = """namespace m {
archive X
{
    payload : raw_data;
}
}

"""
    _PAYLOAD_DOC = """"""
    _NAME = "X"
    _RESOURCES = {
        "X.archive" : flatdata.archive.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "payload": flatdata.archive.ResourceSignature(container=flatdata.resources.RawData,
            initializer=None,
            schema=_PAYLOAD_SCHEMA,
            is_optional=False,
            doc=_PAYLOAD_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)

class m_XBuilder(flatdata.archive_builder.ArchiveBuilder):
    _SCHEMA = """namespace m {
archive X
{
    payload : raw_data;
}
}

"""
    _PAYLOAD_SCHEMA = """namespace m {
archive X
{
    payload : raw_data;
}
}

"""
    _PAYLOAD_DOC = """"""
    _NAME = "X"
    _RESOURCES = {
        "X.archive" : flatdata.archive_builder.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "payload": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.RawData,
            initializer=None,
            schema=_PAYLOAD_SCHEMA,
            is_optional=False,
            doc=_PAYLOAD_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive_builder.ArchiveBuilder.__init__(self, resource_storage)

#  Builtin type to for MultiVector index 
class _builtin_multivector_IndexType32(flatdata.structure.Structure):
    """/** Builtin type to for MultiVector index */"""
    _SCHEMA = """"""
    _NAME = "_builtin_multivector_IndexType32"
    _SIZE_IN_BITS = 32
    _SIZE_IN_BYTES = 4
    _FIELDS = {
        "value": flatdata.structure.FieldSignature(offset=0, width=32, is_signed=False, dtype="u8"),
    }
    _FIELD_KEYS = {
        "value",
    }
class a_A(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace m {
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive X
{
    payload : raw_data;
}
}

namespace a {
archive A
{
    single : .n.S;
    list : vector< .m.S >;
    multi : multivector< 32, .n.S >;
    inner : archive .n.X;
}
}

"""
    _SINGLE_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    single : .n.S;
}
}

"""
    _SINGLE_DOC = """"""
    _LIST_SCHEMA = """namespace m {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    list : vector< .m.S >;
}
}

"""
    _LIST_DOC = """"""
    _MULTI_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    multi : multivector< 32, .n.S >;
}
}

"""
    _MULTI_DOC = """"""
    _INNER_SCHEMA = """namespace n {
archive X
{
    payload : raw_data;
}
}

namespace a {
archive A
{
    inner : archive .n.X;
}
}

"""
    _INNER_DOC = """"""
    _NAME = "A"
    _RESOURCES = {
        "A.archive" : flatdata.archive.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "single": flatdata.archive.ResourceSignature(container=flatdata.resources.Instance,
            initializer=n_S,
            schema=_SINGLE_SCHEMA,
            is_optional=False,
            doc=_SINGLE_DOC),
        "list": flatdata.archive.ResourceSignature(container=flatdata.resources.Vector,
            initializer=m_S,
            schema=_LIST_SCHEMA,
            is_optional=False,
            doc=_LIST_DOC),
        "multi": flatdata.archive.ResourceSignature(container=flatdata.resources.Multivector,
            initializer=[_builtin_multivector_IndexType32,n_S],
            schema=_MULTI_SCHEMA,
            is_optional=False,
            doc=_MULTI_DOC),
        "inner": flatdata.archive.ResourceSignature(container=flatdata.archive.Archive,
            initializer=n_X,
            schema=_INNER_SCHEMA,
            is_optional=False,
            doc=_INNER_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)

class a_ABuilder(flatdata.archive_builder.ArchiveBuilder):
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace m {
struct S
{
    x : u64 : 64;
}
}

namespace n {
archive X
{
    payload : raw_data;
}
}

namespace a {
archive A
{
    single : .n.S;
    list : vector< .m.S >;
    multi : multivector< 32, .n.S >;
    inner : archive .n.X;
}
}

"""
    _SINGLE_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    single : .n.S;
}
}

"""
    _SINGLE_DOC = """"""
    _LIST_SCHEMA = """namespace m {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    list : vector< .m.S >;
}
}

"""
    _LIST_DOC = """"""
    _MULTI_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace a {
archive A
{
    multi : multivector< 32, .n.S >;
}
}

"""
    _MULTI_DOC = """"""
    _INNER_SCHEMA = """namespace n {
archive X
{
    payload : raw_data;
}
}

namespace a {
archive A
{
    inner : archive .n.X;
}
}

"""
    _INNER_DOC = """"""
    _NAME = "A"
    _RESOURCES = {
        "A.archive" : flatdata.archive_builder.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "single": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.Instance,
            initializer=n_S,
            schema=_SINGLE_SCHEMA,
            is_optional=False,
            doc=_SINGLE_DOC),
        "list": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.Vector,
            initializer=m_S,
            schema=_LIST_SCHEMA,
            is_optional=False,
            doc=_LIST_DOC),
        "multi": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.Multivector,
            initializer=[_builtin_multivector_IndexType32,n_S],
            schema=_MULTI_SCHEMA,
            is_optional=False,
            doc=_MULTI_DOC),
        "inner": flatdata.archive_builder.ResourceSignature(container=flatdata.archive.Archive,
            initializer=n_X,
            schema=_INNER_SCHEMA,
            is_optional=False,
            doc=_INNER_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive_builder.ArchiveBuilder.__init__(self, resource_storage)

