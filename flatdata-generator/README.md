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

Currently supported target languages:

* C++
* Rust
* Python
* Go
* Dot (graph of the schema)
* Flatdata (normalized stable schema)

## Architecture

### Stages

The `flatdata` generator works in several stages which are clearly separated from one another and can be extended/tested in isolation:

1. **Parse the source schema** file using `pyparsing` library. Grammar
   for the schema is defined in `grammar.py`
2. **Construct a node tree** out of `pyparsing.ParseResults`. The node tree
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

3. **Augment the tree** with structures and references that are not
   directly corresponding to `pyparsing.ParseResults` or needed to
   implement advanced features. Among these:

   -  **Add builtin structures** if any of the resources require them. For
      example, `multivector< N, ... >` requires
      `_builtin.multivector.IndexTypeN` to be available.
   -  **Add constant references** to all archives so that constants are
      available for schema resolution.

4. **Resolve references** iterates through all references and tries to
   find a node they refer to, either in:

   -  Parent scopes until (inclusive) innermost parent namespace.
   -  Root node if path is fully qualified.

5. **Perform topological sorting** to detect cycles in between entities
   and to determine the order of serialization for targets that depend
   on one.

6. **Generate the source code** using nodes in topological order *and/or*
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

### References

Reference names are mangled so they are not ambiguous with other paths
components. For example reference to type `T` would have name `@T`,
and similarly reference to `.foo.bar.T` would change to
`@@foo@bar@T`.
