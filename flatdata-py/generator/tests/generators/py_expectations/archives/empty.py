class n_A(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
archive A
{
}
}

"""
    _NAME = "A"
    _RESOURCES = {
        "A.archive" : flatdata.archive.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)