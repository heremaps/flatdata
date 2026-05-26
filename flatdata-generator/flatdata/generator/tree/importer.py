'''
 Copyright (c) 2025 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from __future__ import annotations

import logging
import os
from dataclasses import dataclass, field
from typing import Any

from pyparsing import ParseBaseException

from ..grammar import flatdata_grammar
from .errors import ImportFileNotFoundError, ImportParsingError

logger = logging.getLogger(__name__)


@dataclass
class ImportInfo:
    """Metadata about an import directive."""
    path: str           # original import path as written in the schema
    abs_path: str       # canonical absolute path of the imported file


@dataclass
class ResolvedFile:
    """A schema file with its imports resolved."""
    abs_path: str       # canonical absolute path
    content: str        # raw file content
    imports: list[ImportInfo]  # direct imports from this file
    parsed: Any = field(repr=False)  # cached pyparsing result for builder reuse


def resolve_imports(root_path: str) -> tuple[list[ResolvedFile], list[ImportInfo]]:
    """
    Recursively resolve all imports starting from root_path.

    Each file is parsed exactly once with the flatdata grammar. The parse
    result is cached in ``ResolvedFile.parsed`` so that downstream consumers
    (e.g. the AST builder) do not need to re-parse.

    Returns a tuple of:
    - list of ResolvedFile in dependency-first order (each file appears exactly once)
    - list of ImportInfo for direct imports from the root file

    Handles:
    - Diamond imports: same file imported from multiple paths (deduplication via canonical paths)
    - Cyclic imports: A imports B, B imports A (visited set prevents infinite recursion)
    - Symlinks: resolved via os.path.realpath() to canonical target

    :raises ImportFileNotFoundError: if an imported file does not exist
    """
    visited: set[str] = set()
    result: list[ResolvedFile] = []
    root_imports: list[ImportInfo] = []

    def _resolve(file_path: str, referenced_from: str | None) -> None:
        canonical = os.path.realpath(file_path)

        if canonical in visited:
            return
        visited.add(canonical)

        if not os.path.isfile(canonical):
            raise ImportFileNotFoundError(
                path=file_path,
                referenced_from=referenced_from or file_path
            )

        with open(canonical, 'r') as f:
            content = f.read()

        try:
            parsed = flatdata_grammar.parseString(content, parseAll=True)[0]
        except ParseBaseException as e:
            raise ImportParsingError(
                file_path=canonical,
                pyparsing_error=e,
                referenced_from=referenced_from
            )

        # Extract import paths from the cached parse result
        import_paths = (
            [imp["path"] for imp in parsed["imports"]]
            if "imports" in parsed else []
        )
        base_dir = os.path.dirname(canonical)

        imports: list[ImportInfo] = []
        for imp_path in import_paths:
            full_imp_path = os.path.join(base_dir, imp_path)
            imp_canonical = os.path.realpath(full_imp_path)

            if not os.path.isfile(imp_canonical):
                raise ImportFileNotFoundError(
                    path=imp_path,
                    referenced_from=canonical
                )

            imports.append(ImportInfo(path=imp_path, abs_path=imp_canonical))

            if imp_canonical in visited:
                logger.debug("Skipping already-visited import: %s (from %s)",
                           imp_path, canonical)
            else:
                _resolve(full_imp_path, referenced_from=canonical)

        result.append(ResolvedFile(
            abs_path=canonical,
            content=content,
            imports=imports,
            parsed=parsed,
        ))

    root_canonical = os.path.realpath(root_path)
    _resolve(root_path, referenced_from=None)

    # Extract root file's direct imports
    for resolved in result:
        if resolved.abs_path == root_canonical:
            root_imports = resolved.imports
            break

    return result, root_imports
