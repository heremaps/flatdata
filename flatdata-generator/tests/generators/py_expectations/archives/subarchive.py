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

class n_A(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
archive X
{
    payload : raw_data;
}
}

namespace n {
archive A
{
    data : archive .n.X;
    @optional
    optional_data : archive .n.X;
}
}

"""
    _DATA_SCHEMA = """namespace n {
archive X
{
    payload : raw_data;
}
}

namespace n {
archive A
{
    data : archive .n.X;
}
}

"""
    _DATA_DOC = """"""
    _OPTIONAL_DATA_SCHEMA = """namespace n {
archive X
{
    payload : raw_data;
}
}

namespace n {
archive A
{
    @optional
    optional_data : archive .n.X;
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
        "data": flatdata.archive.ResourceSignature(container=flatdata.archive.Archive,
            initializer=n_X,
            schema=_DATA_SCHEMA,
            is_optional=False,
            doc=_DATA_DOC),
        "optional_data": flatdata.archive.ResourceSignature(container=flatdata.archive.Archive,
            initializer=n_X,
            schema=_OPTIONAL_DATA_SCHEMA,
            is_optional=True,
            doc=_OPTIONAL_DATA_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)