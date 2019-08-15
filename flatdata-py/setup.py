#!/usr/bin/env python3
from setuptools import find_packages, setup


setup(
    name="flatdata-py",
    version="0.3.0",
    author="Flatdata Developers",
    description="Python 3 implementation of Flatdata",
    long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/heremaps/flatdata",
    # we can't use find_namespace_packages as it is only a very recent addition to setuptools
    packages=['flatdata.' + p for p in find_packages('flatdata')],
    extras_require={
        'inspector': ['IPython']
    },
    entry_points={
        'console_scripts': [
            'flatdata-inspector = flatdata.lib.inspector:main [inspector]'
        ],
    },
    install_requires=open("requirements.txt").readlines(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
    ],
)
