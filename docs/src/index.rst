Flatdata
========

``Flatdata`` is a library providing data structures for convenient creation,
storage and access of packed memory-mappable structures with minimal overhead. Library consists
of schema language, code generator for C++ and Python, and target language libraries.

Why ``flatdata``?
-----------------

Flatdata helps creating efficient datasets:

* Zero overhead random access
* Support for bit and byte packing
* Structuring data using a schema definition
* Optimized for large read-only datasets
* Portable with support for multiple languages

Flatdata doesn't provide:

* Backwards compatible schema evolution
* Support for mutable datasets
* Portable floating point serialization

For more details read :doc:`why-flatdata`.

Quick Start
-----------

For quick start documentation please look into :doc:`quick-start`.

Supported Languages
-------------------

At the moment following languages are supported:

C++
  Main development target and first-class citizen. Used
  extensively, tested excessively, normally receives features first.
  Use ``cpp`` generator.
  Libraries located in ``flatdata-cpp`` folder. See documentation in :doc:`flatdata-cpp/index`.

*Python*
  Used mostly for prototyping, debugging and testing purposes.
  Normally, receives features a bit later.
  Use ``py`` generator.
  Libraries located in ``flatdata-py``. See documentation in :doc:`flatdata-py/index`.

*Dot*
  Used to generate diagrams of the schema.
  Normally, it is up to date. Use ``dot`` generator.


User Guide
----------

.. toctree::
   :maxdepth: 2

   quick-start
   why-flatdata
   library-layout
   schema-language
   building-archives

Availble Implementations
------------------------

.. toctree::
   :maxdepth: 2

   flatdata-cpp/index.rst
   flatdata-py/index.rst


Developer Guide
---------------

.. toctree::
   :maxdepth: 2

   developer-documentation/generator-architecture


Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`

