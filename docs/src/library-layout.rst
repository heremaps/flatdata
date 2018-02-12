Flatdata Library Layout
=======================

Flatdata Code is organized as follows:

-  ``generator`` includes sources of the flatdata code generator.

   -  ``generator/app.py`` executable script. Use it to generate code in
      target language.
   -  ``generator/tree/**/*.py`` AST-like data structure used as
      internal representation of the schema. Base functionality of
      flatdata/generators is implemented here.
   -  ``generator/generators/**/*.py`` target language generators.
      Provide common framework for generating source code for different
      targets.
   -  ``generator/templates/**/*.jinja2`` target language templates.
      Language-specific functionality of flatdata is mostly implemented
      here.

-  ``flatdata-cpp`` includes C++ library sources. Client application
   needs to include and link against this library.
-  ``flatdata-py`` includes python library sources. Client application
   needs to have this folder in PYTHON\_PATH
-  ``tools`` contains tools to work with flatdata archives.

   -  ``tools/inspect_flatdata.py`` provides interactive python
      interpreter loaded with a specified archive.
