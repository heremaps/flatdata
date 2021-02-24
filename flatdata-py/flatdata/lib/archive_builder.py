'''
 Copyright (c) 2021 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

class ArchiveBuilder:
    """
    Archive class. Entry point to Flatdata.
    Provides access to flatdata resources and verifies archive/resource schemas on opening.
    """

    def __init__(self, resource_storage):
        NotImplemented