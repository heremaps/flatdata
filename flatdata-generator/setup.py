#!/usr/bin/env python3
from setuptools import find_packages, setup

setup(
    name="flatdata-generator",
    version="0.3.0-2",
    author="Flatdata Developers",
    description="Generate source code for C++, Rust, Go or Python from a Flatdata schema file",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/heremaps/flatdata",
    # we can't use find_namespace_packages as it is only a very recent addition to setuptools
    packages=['flatdata.' + p for p in find_packages('flatdata')],
    package_data={
        '': ['*.jinja2'],
    },
    entry_points={
        'console_scripts': ['flatdata-generator=flatdata.generator.app:main'],
    },
    install_requires=[
        "pyparsing>=2.0",
        "jinja2>=2.2",
    ],
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
