Flatdata Usage Examples
=======================

This folder contains examples of flatdata usage. Those are meant to show how
to use serialization/deserialization APIs and different resource types available.

Binary Layout Example
---------------------

This example writes a simple not aligned data structure (23 bits long) to a flatdata vector.
The structure is filled with ones and zeroes to make it easier to see how different fields are
stored.

Two structures are written to the output to show the structure alignment and padding.

Coappearances
-------------

This examples converts a graph of coappearances from json to flatdata. A graph of coappearances is
an undirected graph where the vertices are characters from a book. An edge between two characters
represents their appearance in the same scene.

The idea of this example to show how to convert a nested data format to a flat representation. Further, the example introduces

* all available data structures in flatdata,
* a technique how to represent ranges with sentinels, and
* representation of strings as raw blocks.

The examples also contains a simple reader which dumps the flatdata archive to terminal.

The data [karenina.json](karenina.json) is based on characters coappearance in Leo Tolstoy's "_Anna
Karenina_", compiled by [Donald Knuth][1].

[1]: https://www-cs-faculty.stanford.edu/~knuth/sgb.html
