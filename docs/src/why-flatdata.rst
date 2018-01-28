Why ``flatdata``?
=================

There are plenty libraries and services for handling data. How is
``flatdata`` different?

Access Patterns
---------------

What happens if you need to traverse hundreds of
thousands of entities per incoming request? What if for each entity you
need to do extra lookups in the data? What if the entities are
relatively small (few dozen bytes), but exhibit poor locality?

``flatdata`` provides tools to implement efficient data storage that
enables such use-cases. While structuring the data is still done by
developer, ``flatdata`` provides implementations for common patterns
that make the job simpler.

Efficient Storage
-----------------

What if your data consists of millions or
billions of entities? What if the data is as rich as hundreds of
potentially assigned attributes for each entity? Storing things
efficiently makes for a smaller footprint, saves bandwidth and allows to
better utilize system caches, directly affecting performance.

``flatdata`` allows to operate on bit level without losing benefits of
structured data access. If the attribute is boolean, why use a byte for
it? If the value range is [0, 211) why not use spare five bits at the
end of the field?

Structure
---------

Finally, if your data is rich, you might want to give it
some structure. ``flatdata`` allows to define data structure using a
schema language, which can also be used for documenting the data format,
as well as to avoid writing boilerplate code.

When is That Useful?
--------------------

We designed the library that way to support following patterns:

-  Your data updates infrequently (say, every few hours. But as long as
   update rate is *much* smaller than serialization time, you should be
   set).
-  You can afford to recreate the full flatdata archive every time you
   need to do structure update.
-  Your data can be efficiently serialized into a mid-sized archive
   (~50GB? ~200GB? As long as data size is *comparable* to the amount of
   RAM on the machine, you should be fine.)
-  Your data is going to be accessed *many* times (substantially more
   often than it is updated).
-  You want to optimize your data to be cache-friendly: sort things as
   often as possible, minimize size of indices, group data that is used
   together, store everything else separately.

What Can I Store in Flatdata?
-----------------------------

``flatdata`` does not impose any particular way of structuring the data.
So, mostly anything. That said, following patterns work best:

-  Structures with categorical and numerical data.
-  Implicit or explicit references between data structures. One to one,
   one to many, many to many. Pretty much as in any DBMS, up to
   developer to implement via associative vectors.
-  Low-frequency attributes, which can be assigned to only a small
   subset of a large number of entities.
-  Arbitrary metadata that can be attached to entities.

What is behind ``flatdata``?
----------------------------

-  ``flatdata`` is based on files. ``flatdata`` uses custom
   platform-independent structure alignment (one byte) and stores data
   in little-endian order.
-  Reading and writing data is implemented in efficient C++. Templates,
   inlining and compiler optimizations yield only few instructions per
   data access.
-  When accessing ``flatdata``, no data is accessed or copied, until the
   point you use it. *When* you use it, you read only the bytes you
   need. Memory-mapped files and page cache takes care of everything
   else.
-  When writing to ``flatdata``, one can create full collection of
   structures in memory and dump it to disk or efficiently build up
   large collection while keeping fixed memory footprint.
