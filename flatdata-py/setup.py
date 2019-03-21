#!/usr/bin/env python3
from setuptools import find_namespace_packages
import setuptools


setuptools.setup(
    name="flatdata-py",
    version="0.2.1",
    author="Flatdata Developers",
    description="Python 3 implementation of Flatdata",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/heremaps/flatdata",
    packages=find_namespace_packages(),
    entry_points={
        'console_scripts': [
            'flatdata-inspector=flatdata.inspector:main'
        ],
    },
    install_requires=open("requirements.txt").readlines(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: Apache License",
        "Operating System :: OS Independent",
    ],
)
