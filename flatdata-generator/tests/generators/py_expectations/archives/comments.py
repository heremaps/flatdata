class n_Foo(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
archive Foo
{
    bar : raw_data;
}
}

"""
    _BAR_SCHEMA = """namespace n {
archive Foo
{
    bar : raw_data;
}
}

"""
    _BAR_DOC = """// this is a comment about foo.bar"""
    _NAME = "Foo"
    _RESOURCES = {
        "Foo.archive" : flatdata.archive.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "bar": flatdata.archive.ResourceSignature(container=flatdata.resources.RawData,
            initializer=None,
            schema=_BAR_SCHEMA,
            is_optional=False,
            doc=_BAR_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)

class n_Bar(flatdata.archive.Archive):
    _SCHEMA = """namespace n {
archive Bar
{
    foo : raw_data;
}
}

"""
    _FOO_SCHEMA = """namespace n {
archive Bar
{
    foo : raw_data;
}
}

"""
    _FOO_DOC = """/*
         * this is a comment about bar.foo
         */"""
    _NAME = "Bar"
    _RESOURCES = {
        "Bar.archive" : flatdata.archive.ResourceSignature(
            container=flatdata.resources.RawData,
            initializer=None,
            schema=_SCHEMA,
            is_optional=False,
            doc="Archive signature"),
        "foo": flatdata.archive.ResourceSignature(container=flatdata.resources.RawData,
            initializer=None,
            schema=_FOO_SCHEMA,
            is_optional=False,
            doc=_FOO_DOC),
    }

    def __init__(self, resource_storage):
        flatdata.archive.Archive.__init__(self, resource_storage)

