# `flatdata` Schema Language

## Basic Types

Flatdata supports the following primitive types:

-  ``bool`` - boolean data type
-  ``i8`` - signed 8-bit wide type
-  ``u8`` - unsigned 8-bit wide type
-  ``i16`` - signed 16-bit wide type
-  ``u16`` - unsigned 16-bit wide type
-  ``i32`` - signed 32-bit wide type
-  ``u32`` - unsigned 32-bit wide type
-  ``i64`` - signed 64-bit wide type
-  ``u64`` - unsigned 64-bit wide type

## Enumerations

Flatdata supports adding enumeration over basic types. Each enumeration
value can either automatically be assigned a value (previous value +1,
starting with 0), or manually.

Each enumeration is defined as follows:

```cpp
enum <Name> : <type> {
    <value name> [= value],
    ...
}
```

The following restrictions for values are checked:
-   No duplicate values
-   Values must fit into the underlying type

## Structures

Flatdata structure definition syntax resembles known alternatives,
albeit with notable differences:

-  *Backward compatibility:* flatdata format does not have backward
   compatibility support built in. It is not meant to be used directly
   in communication protocols, there are libraries which are well-known
   and well-suited for that purpose. Flatdata is a create-once
   read-extensively storage library.
-  *Bit fields:* Unlike with some other formats, bitfields are supported
   natively in a platform- independent fashion.

Each structure is defined as follows:

```cpp
struct <Name> {
    <field> : <type> : <width>;
    ...
}
```

``<type>`` can either be a basic type, or an enumeration.

Example:

```cpp
struct Structure {
    field1 : u32 : 29;
    field2 : u8 : 2;
}
```

Every structure field specifies target language type to represent the
field and its target size in bits. Flatdata takes care of packing and
aligning the structures correctly as well as accessing them efficiently.

## Archives

Flatdata archive is the entry point to the data. Archives are the
smallest data units which can be opened or written to disk. An archive's
schema is saved along with the data and is checked when the archive is
opened. If schema does not match expectations, archive can not be
opened.

Archives are defined as follows:

```cpp
archive <name> {
    <resource> : <type>;
    ...
}
```

For example:

```cpp
archive ExampleArchive {
    single_structure : StructureType;
    vector_of_stuff : vector< StructureType >;
    what_an_archive_without_a_map : multivector< 40, StructureA, StructureB, StructureC >;
    strings_forever : raw_data;
    lets_get_some_structure : archive OtherArchive;
}
```

## Resources

Archive resources can be one of following types:

-  ``T`` - a single structure of given type
-  ``vector< T >`` - a vector of structures of a given type.
-  ``multivector< IndexSize, T1, T2, ... >`` - a heterogenuous
   associative container for storing multiple properties for a single
   entity. Allows efficient storage of the data whose properties are
   sparsily assigned to each item. Think of it as a multimap of
   variants. ``IndexSize`` is the number of bits used for indexing the
   entities. An index is addressing the start of the offset of a
   variant in the data.
-  ``raw_data`` - Uninterpreted raw data. Useful for storing arrays of
   non-numeric data like strings referenced from structures.
-  ``archive ArchiveName`` - Archive resource. Archive resources allow
   to structure large archives better, while also acting as a namespace
   and grouping optionality semantics. Referenced archive type has to be
   defined.

## Comments

Flatdata schema supports C++-style comments. Comments located before
structures/archives or their members will be available in generated
code. Example:

```cpp
/// A single secret. Might be important
struct Secret { importance : u64 : 64; }

/**
    * Very important archive
    */
archive TheBookOfSecrets {
    // More important secret
    secret1 : Secret;
    // Less important secret
    secret2 : Secret;
}
```

## Decorations

Decorations declare additional properties of entities they are applied
to. Decorations supported at the moment are described below. Note that
not all target languages provide full support for all decorations. For
example, ``dot`` generator uses decorations to group archive resources
and create reference edges, while other generators mostly support only
``@optional``.

Nonetheless, decorations are first-class citizens of schema and thus are
validated as well during archive opening.

## Optional

``@optional`` can be applied to resources. If resource is optional and
missing, archive can still be opened successfully. Resource of any type
can be optional. Example:

```cpp
archive Archive {
    @optional
    resource: vector< SomeStructure >;
}
```

## Explicit Reference

``@explicit_reference`` declares an explicit reference of one resource's
property to another resource. This is a very common type of referencing
in flatdata and can be seen as a "Foreign Key", with the exception that
consistency of the key is not enforced.

It is possible to define explicit reference with its target in a
different archive, as long as it is defined.

Example:

```cpp
struct Person {
    name : u64 : 64;
    first_child : u64 : 64;
}

archive Archive {
    @explicit_reference( Person.name, names )
    @explicit_reference( Person.first_child, children )
    people: vector< Person >

    children: vector< Child >

    names: raw_data
}
```

## Bound Implicitly

Sometimes it is useful to split structures' fields into multiple
resources (for example, to promote data locality in case binary search
is done extensively on a particular field). ``@bound_implicitly``
declares that such resources are grouped implicitly and therefore
represent a single entity. The decoration also gives entity a name

```cpp
@bound_implicitly( transactions: keys, transaction_data )
archive Archive {
    keys: vector< Key >
    transaction_data : vector< Transaction >
}
```

## Entity Referencing

Resources and decorations can reference other entities declared in the
schema. Types can be specified either with fully-qualified path or with
local path, for example:

```cpp
namespace N {
    struct T {
        ...
    }

    archive Archive {
        // Local path
        resource: vector< T >
        // Fully-qualified path
        another_resource: vector< .N.T >
    }
}
```

Local paths must be available in the current namespace.

## Index Ranges

When flattening a data model into a flatdata schema one often encounters
a pattern of storing index ranges as members of consecutive vector items:

```cpp
struct Node {
    ...
    first_edge_ref : u32;
}

struct Edge {
    ...
}

archive Archive {
    // contains sentinel
    @explicit_reference( Nodes.first_edge_ref, edges )
    nodes : vector< Nodes >;
    edges : vector< Edges >;
}
```

In this case the edges of a node `i` are then retrieved as
```cpp
edges.slice(nodes[i].first_edge_ref..nodes[i + 1].first_edge_ref)
```
Additionally the last element of the `nodes` vector is usually a sentinel (only used to retrieve `first_edge_index`).
To simplify this flatdata offers the `@range(name_of_range_attribute)` annotation:

```cpp
struct Node {
    ...
    @range(edges_range)
    first_edge_ref : u32;
}
```

This will have two effects:
* Adding `edges_range` attribute exposing range `(nodes[i].first_edge_ref, nodes[i + 1].first_edge_ref)`
* Hiding the sentinel in views (it still needs to be populated first, though)

Retrieving all edges is now as easy as this:
```cpp
edges.slice(nodes[i].edges_range)
```
