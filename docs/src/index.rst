Flatdata
========

**Flatdata** is a library providing data structures for convenient creation, storage and access of
packed memory-mappable structures with minimal overhead possible.
Structures may contain bitfields which will be serialized in a platform-independent manner.
Library consists of:

- A code generator for different target languages
- Target language libraries
- Tools

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

   library-layout
   schema-language
   building-archives

Availble Implementations
------------------------

.. toctree::
   :maxdepth: 1

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

