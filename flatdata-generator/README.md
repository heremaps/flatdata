# flatdata-generator

[![Build Status](https://api.travis-ci.com/heremaps/flatdata.svg?branch=master)](https://travis-ci.com/heremaps/flatdata/)

Generates code from a [flatdata](https://github.com/heremaps/flatdata) schema file.

For more information on `flatdata` and its implementations, please [refer to flatdata's homepage](https://github.com/heremaps/flatdata).

## Using `flatdata-generator`

```sh
# installation
pip3 install flatdata-generator

# example: generate a header-only C++ library
flatdata-generator -s locations.flatdata -g cpp -O locations.hpp
```

### Multi-file Schemas

When a schema uses `import` statements, each file should be generated
separately. Imported types are referenced via include/import directives rather
than being re-emitted:

```sh
# Generate shared types
flatdata-generator -s schema/types.flatdata -g cpp -O schema/types.h

# Generate main schema (will #include "types.h")
flatdata-generator -s schema/main.flatdata -g cpp -O schema/main.h
```

For Rust, the same approach applies — each imported file becomes its own module
with `pub use` re-exports connecting the namespaces.

Python and Dot generators emit all types monolithically (no separate generation
needed for the root file — all imported definitions are included in the output).

Currently supported target languages:

* C++
* Rust
* Python
* Dot (graph of the schema)
* Flatdata (normalized stable schema)

## Architecture

### Stages

The `flatdata` generator works in several stages which are clearly separated from one another and can be extended/tested in isolation:

1. **Resolve imports** starting from the root schema file. The importer
   (`importer.py`) performs a depth-first traversal of import statements,
   deduplicating files and handling cyclic imports. The result is an ordered
   list of resolved files with their parsed content.

2. **Parse the source schema** file using `pyparsing` library. Grammar
   for the schema is defined in `grammar.py`
3. **Construct a node tree** out of `pyparsing.ParseResults`. The node tree
   contains entities for every construct of flatdata grammar, organized
   in hierarchical order, allowing non-tree references between nodes:

   -  `Namespace` - Nesting namespaces in the tree is allowed.
   -  `Structure` - Structures are grouping together a set of fields.
   -  `Archive` - Archives are grouping together resources and are
      referencing structures or other archives (see `Reference`)
   -  `ResourceBase` - All resources derive from `ResourceBase`
   -  `Reference` - All references between flatdata entities are
      modeled with `Reference` nodes. All references participate in
      name resolution. There are two type of references:
      -  `RuntimeReference` - model explicit references and bound
         resources that show themselves at runtime.
      -  `TypeReference` - model type dependencies, which are used during
         topological sorting at a later stage and for schema resolution.

4. **Augment the tree** with structures and references that are not
   directly corresponding to `pyparsing.ParseResults` or needed to
   implement advanced features. Among these:

   -  **Add builtin structures** if any of the resources require them. For
      example, `multivector< N, ... >` requires
      `_builtin.multivector.IndexTypeN` to be available in the parent namespace.
   -  **Add constant references** to all archives so that constants are
      available for schema resolution.

5. **Resolve references** iterates through all references and tries to
   find a node they refer to, either in:

   -  Parent scopes until (inclusive) innermost parent namespace.
   -  Root node if path is fully qualified.

6. **Perform topological sorting** to detect cycles in between entities
   and to determine the order of serialization for targets that depend
   on one.

7. **Generate the source code** using nodes in topological order *and/or*
   the tree (depending on the generator architecture - recursive descent
   or iterative).

### Node Tree

Every node of the tree consists of its name, properties (metadata) and
holds references to its children. Every node is reachable via certain
path which is a dot-joint concatenation of the names of its parents.
Node tree enforces several properties of the flatdata schema:

-  *No conflicting declarations*: No two nodes with the same path are
   allowed.
-  *All references are correct*: All reference nodes are resolvable.
-  *No cyclic dependencies among resources*: All `TypeReference`
   participate in topological sorting of the DAG formed by the tree
   edges and edges between source and target of a `TypeReference`

When building a tree from multiple files, each node is tagged with its
`source_file` (the file it was defined in) and an `is_local` flag
(whether it belongs to the root file being generated). This allows
generators to filter nodes for separate compilation.

### References

Reference names are mangled so they are not ambiguous with other paths
components. For example reference to type `T` would have name `@T`,
and similarly reference to `.foo.bar.T` would change to
`@@foo@bar@T`.
