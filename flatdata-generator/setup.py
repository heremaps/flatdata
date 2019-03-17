#!/usr/bin/env python3
import setuptools
from setuptools import find_packages


setuptools.setup(
    name="flatdata-generator",
    version="0.2.1",
    author="Flatdata Developers",
    description="Generate source code for C++, Rust, Go or Python from a Flatdata schema file",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/heremaps/flatdata",
    packages=find_packages(),
    entry_points={
        'console_scripts': [
            'flatdata-generator=flatdata.generator.app:main'
        ],
    },
    install_requires=open("requirements.txt").readlines(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: Apache License",
        "Operating System :: OS Independent",
    ],
)
