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

class n_T(flatdata.structure.Structure):
    """"""
    _SCHEMA = """namespace n {
struct T
{
    x : u64 : 64;
}
}

"""
    _NAME = "n_T"
    _SIZE_IN_BITS = 64
    _SIZE_IN_BYTES = 8
    _FIELDS = {
        "x": flatdata.structure.FieldSignature(offset=0, width=64, is_signed=False, dtype="u8"),
    }
    _FIELD_KEYS = {
        "x",
    }
#  Builtin type to for MultiVector index 
class _builtin_multivector_IndexType8(flatdata.structure.Structure):
    """/** Builtin type to for MultiVector index */"""
    _SCHEMA = """"""
    _NAME = "_builtin_multivector_IndexType8"
    _SIZE_IN_BITS = 8
    _SIZE_IN_BYTES = 1
    _FIELDS = {
        "value": flatdata.structure.FieldSignature(offset=0, width=8, is_signed=False, dtype="u8"),
    }
    _FIELD_KEYS = {
        "value",
    }
#  Builtin type to for MultiVector index 
class _builtin_multivector_IndexType16(flatdata.structure.Structure):
    """/** Builtin type to for MultiVector index */"""
    _SCHEMA = """"""
    _NAME = "_builtin_multivector_IndexType16"
    _SIZE_IN_BITS = 16
    _SIZE_IN_BYTES = 2
    _FIELDS = {
        "value": flatdata.structure.FieldSignature(offset=0, width=16, is_signed=False, dtype="u8"),
    }
    _FIELD_KEYS = {
        "value",
    }
#  Builtin type to for MultiVector index 
class _builtin_multivector_IndexType64(flatdata.structure.Structure):
    """/** Builtin type to for MultiVector index */"""
    _SCHEMA = """"""
    _NAME = "_builtin_multivector_IndexType64"
    _SIZE_IN_BITS = 64
    _SIZE_IN_BYTES = 8
    _FIELDS = {
        "value": flatdata.structure.FieldSignature(offset=0, width=64, is_signed=False, dtype="u8"),
    }
    _FIELD_KEYS = {
        "value",
    }
class n_A(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : multivector< 8, .n.S, .n.T >;
    @optional
    optional_data : multivector< 16, .n.S, .n.T >;
    data_u64_index : multivector< 64, .n.S, .n.T >;
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
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : multivector< 8, .n.S, .n.T >;
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
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    @optional
    optional_data : multivector< 16, .n.S, .n.T >;
}
}

"""
    _OPTIONAL_DATA_DOC = """"""
    _DATA_U64_INDEX_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data_u64_index : multivector< 64, .n.S, .n.T >;
}
}

"""
    _DATA_U64_INDEX_DOC = """"""
    _NAME = "A"
    _RESOURCES = {
        "A.archive" : flatdata.archive.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "data": flatdata.archive.ResourceSignature(container=flatdata.resources.Multivector,
            initializer=[_builtin_multivector_IndexType8,n_S,n_T],
            schema=_DATA_SCHEMA,
            is_optional=False,
            doc=_DATA_DOC),
        "optional_data": flatdata.archive.ResourceSignature(container=flatdata.resources.Multivector,
            initializer=[_builtin_multivector_IndexType16,n_S,n_T],
            schema=_OPTIONAL_DATA_SCHEMA,
            is_optional=True,
            doc=_OPTIONAL_DATA_DOC),
        "data_u64_index": flatdata.archive.ResourceSignature(container=flatdata.resources.Multivector,
            initializer=[_builtin_multivector_IndexType64,n_S,n_T],
            schema=_DATA_U64_INDEX_SCHEMA,
            is_optional=False,
            doc=_DATA_U64_INDEX_DOC),
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
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : multivector< 8, .n.S, .n.T >;
    @optional
    optional_data : multivector< 16, .n.S, .n.T >;
    data_u64_index : multivector< 64, .n.S, .n.T >;
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
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data : multivector< 8, .n.S, .n.T >;
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
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    @optional
    optional_data : multivector< 16, .n.S, .n.T >;
}
}

"""
    _OPTIONAL_DATA_DOC = """"""
    _DATA_U64_INDEX_SCHEMA = """namespace n {
struct S
{
    x : u64 : 64;
}
}

namespace n {
struct T
{
    x : u64 : 64;
}
}

namespace n {
archive A
{
    data_u64_index : multivector< 64, .n.S, .n.T >;
}
}

"""
    _DATA_U64_INDEX_DOC = """"""
    _NAME = "A"
    _RESOURCES = {
        "A.archive" : flatdata.archive_builder.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "data": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.Multivector,
            initializer=[_builtin_multivector_IndexType8,n_S,n_T],
            schema=_DATA_SCHEMA,
            is_optional=False,
            doc=_DATA_DOC),
        "optional_data": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.Multivector,
            initializer=[_builtin_multivector_IndexType16,n_S,n_T],
            schema=_OPTIONAL_DATA_SCHEMA,
            is_optional=True,
            doc=_OPTIONAL_DATA_DOC),
        "data_u64_index": flatdata.archive_builder.ResourceSignature(container=flatdata.resources.Multivector,
            initializer=[_builtin_multivector_IndexType64,n_S,n_T],
            schema=_DATA_U64_INDEX_SCHEMA,
            is_optional=False,
            doc=_DATA_U64_INDEX_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive_builder.ArchiveBuilder.__init__(self, resource_storage)
