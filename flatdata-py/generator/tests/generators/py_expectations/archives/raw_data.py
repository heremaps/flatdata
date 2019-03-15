class n_A(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
archive A
{
    data : raw_data;
    @optional
    optional_data : raw_data;
}
}

"""
    _DATA_SCHEMA = """namespace n {
archive A
{
    data : raw_data;
}
}

"""
    _DATA_DOC = """"""
    _OPTIONAL_DATA_SCHEMA = """namespace n {
archive A
{
    @optional
    optional_data : raw_data;
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
        "data": flatdata.archive.ResourceSignature(container=flatdata.resources.RawData,
            initializer=None,
            schema=_DATA_SCHEMA,
            is_optional=False,
            doc=_DATA_DOC),
        "optional_data": flatdata.archive.ResourceSignature(container=flatdata.resources.RawData,
            initializer=None,
            schema=_OPTIONAL_DATA_SCHEMA,
            is_optional=True,
            doc=_OPTIONAL_DATA_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)

