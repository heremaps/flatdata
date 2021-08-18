#!/usr/bin/env python3
import os
from setuptools import find_packages, setup

SOURCE_FILEPATH = os.path.dirname(os.path.abspath(__file__))

setup(
    name="flatdata-py",
    version="0.4.4",
    author="Flatdata Developers",
    description="Python 3 implementation of Flatdata",
    long_description=open(os.path.join(SOURCE_FILEPATH, "README.md")).read(),
    long_description_content_type="text/markdown",
    url="https://github.com/heremaps/flatdata",
    # we can't use find_namespace_packages as it is only a very recent addition to setuptools
    packages=["flatdata." + p for p in find_packages("flatdata")],
    extras_require={
        "inspector": ["IPython"]
    },
    entry_points={
        "console_scripts": [
            "flatdata-inspector = flatdata.lib.inspector:main [inspector]",
            "flatdata-writer = flatdata.lib.writer:main [writer]"
        ],
    },
    setup_requires=["wheel"],
    install_requires=[
        "flatdata-generator==0.4.4",
        "numpy",
        "pandas"
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
