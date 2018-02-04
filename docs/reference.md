# Schema Language

## Basic Types

Flatdata supports following basic types:

   * `bool` - boolean data type
   * `i8` - signed 8-bit wide type
   * `u8` - unsigned 8-bit wide type
   * `i16` - signed 16-bit wide type
   * `u16` - unsigned 16-bit wide type
   * `i32` - signed 32-bit wide type
   * `u32` - unsigned 32-bit wide type
   * `i64` - signed 64-bit wide type
   * `u64` - unsigned 64-bit wide type

## Structures

Flatdata structure definition syntax resembles known alternatives, albeit with notable differences:

   * *Backward compatibility:* flatdata format does not have backward compatibility support built
       in. It is not meant to be used directly in communication protocols, there are libraries which
       are well-known and well-suited for that purpose. Flatdata is a create-once read-extensively
       storage library.
   * *Bit fields:* Unlike with some other formats, bitfields are supported natively in a platform-
       independent fashion.

Each structure is defined as follows:

    struct <Name> {
        <field> : <type> : <width>;
        ...
    }

Example:

    struct Structure {
        field1 : u32 : 29;
        field2 : u8 : 2;
    }

Every structure field specifies target language type to represent the field and its target
size in bits. Flatdata takes care of packing and aligning the structures correctly as well
as accessing them efficiently.

## Archives

Flatdata archive is the entry point to the data. Archives are the smallest data units which
can be opened or written to disk. An archive's schema is saved along with the data and is checked
when the archive is opened. If schema does not match expectations, archive can not be opened.

Archives are defined as follows:

    archive <name> {
        <resource> : <type>;
        ...
    }

For example:

    archive ExampleArchive {
        single_structure : StructureType;
        vector_of_stuff : vector< StructureType >;
        what_an_archive_without_a_map : multivector< 40, StructureA, StructureB, StructureC >;
        strings_forever : raw_data;
        lets_get_some_structure : archive OtherArchive;
    }

## Resources

Archive resources can be one of following types:

   * `T` - a single structure of given type
   * `vector< T >` - a vector of structures of a given type.
   * `multivector< IndexSize, T1, T2, ... >` - a heterogenuous associative container for storing
      multiple properties for a single entity. Allows efficient storage of the data whose
      properties are sparsily assigned to each item. Think of it as a multimap of variants.
      `IndexSize` is the number of bits that is used for indexing the entities. An index is
      addressing the start of the offset of a variant.
   * `raw_data` - Uninterpreted raw data. Useful for storing arrays of non-numeric data like
      strings referenced from structures.
   * `archive ArchiveName` - Archive resource. Archive resources allow to structure large archives
      better, while also acting as a namespace and grouping optionality semantics. Referenced
      archive type has to be defined.

## Comments

Flatdata schema supports C++-style comments. Comments located before structures/archives or their
members will be available in generated code. Example:

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

## Decorations

Decorations declare additional properties of entities they are applied to. Decorations supported at
the moment are described below. Note that not all target languages provide full support for all
decorations. For example, `dot` generator uses decorations to group archive resources and create
reference edges, while other generators mostly support only `@optional`.

Nonetheless, decorations are first-class citizens of schema and thus are validated as well during
archive opening.

### Optional

`@optional` can be applied to resources. If resource is optional and missing, archive can still be
opened successfully. Resource of any type can be optional.
Example:

    archive Archive {
        @optional
        resource: vector< SomeStructure >;
    }

### Explicit Reference

`@explicit_reference` declares an explicit reference of one resource's property to another resource.
This is a very common type of referencing in flatdata and can be seen as a "Foreign Key", with the
exception that consistency of the key is not enforced.

It is possible to define explicit reference with its target in a different archive, as long as it is
defined.

Example:

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

### Bound Implicitly

Sometimes it is useful to split structures' fields into multiple resources (for example, to promote
data locality in case binary search is done extensively on a particular field). `@bound_implicitly`
declares that such resources are grouped implicitly and therefore represent a single entity. The
decoration also gives entity a name

    @bound_implicitly( transactions: keys, transaction_data )
    archive Archive {
        keys: vector< Key >
        transaction_data : vector< Transaction >
    }

## Entity Referencing

Resources and decorations can reference other entities declared in the schema. Types can be specified
either with fully-qualified path or with local path, for example:

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

Local paths must be available in the current namespace. If not, error will be reported.

# Building archives

Archives are designed as efficient write-once read-many data storage. Thus, they do not look or feel
like conventional databases, instead, when building, they provide efficient serializers which write data
to the underlying storage:

   * Once structure's data is set, it cannot be reset (serialization is optimized for write-once scenario).
   * Once resource is created, it cannot be altered anymore. Attempt to do so will result in error.
   * Once archive exists, all its resources with the exception of archive resources are final,
       even if missing. Only missing archive resources can be created within existing archive.


# Generator Architecture

## Stages

Flatdata generator works in several stages which are clearly separated from one another and can be
extended/tested in isolation. These are:

   1. *Parse the source schema* file using `pyparsing` library. Grammar for the schema is defined in
       `grammar.py`
   1. *Construct a node tree* out of `pyparsing.ParseResults`. Node tree contains entities for every
       construct of flatdata grammar organized in hierarchical order allowing non-tree references between
       nodes. Most prominent node types are:
       - `Namespace` - Nesting namespaces in the tree is allowed.
       - `Structure` - Structures are grouping together a set of fields.
       - `Archive` - Archives are grouping together resources and are referencing structures or other
           archives (see `Reference`)
       - `ResourceBase` - All resources derive from `ResourceBase`
       - `Reference` - All references between flatdata entities are modelled with `Reference` nodes.
           All references participate in name resolution. There are two type of references:
           - `RuntimeReference` - model explicit references and bound resources. Model relations of the
               schema that show themselves at runtime.
           - `TypeReference` - model type dependencies. Are used during topological sorting at later stage.
               Are used as well at for schema resolution.
   1. *Augment the tree* with structures and references that are not directly corresponding to
       `pyparsing.ParseResults` or needed to implement advanced features. Among these:
       - *Add builtin structures* if any of the resources require them. For example, `multivector< N, ... >` requires
           `_builtin.multivector.IndexTypeN` to be available.
       - *Add constant references* to all archives so that constants are available for schema resolution.
   1. *Resolve references* iterates through all references and tries to find a node they refer to, either in:
       - Parent scopes until (inclusive) innermost parent namespace.
       - Root node if path is fully qualified.
   1. *Perform topological sorting* to detect cycles in between entities and to determine the order of
       serialization for targets that depend on one.
   1. *Generate the source code* using nodes in topological order *and/or* the tree (depending on the generator
       architecture - recursive descent or iterative).

## Node Tree

Every node of the tree consists of its name, properties (metadata) and holds references to its children.
Eveny node is reachable via certain path which is a dot-joint concatenation of the names of its parents.
Node tree enforces several properties of the flatdata schema:

   - *No conflicting declarations*: No two nodes with the same path are allowed.
   - *All references are correct*: All reference nodes are resolvable.
   - *No cyclyc dependencies among resources*: All `TypeReference` participate in topological sorting of the DAG
       formed by the tree edges and edges between source and target of a `TypeReference`

### References

Reference names are mangled so they are not ambiguous with other paths components. For example reference
to type `T` would have name `@T`, and similarly reference to `.foo.bar.T` would change to `@@foo@bar@T`.
