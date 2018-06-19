/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <cstddef>

namespace flatdata
{
namespace internal
{
/// Returns whether two schema strings are equal while whitespace and comments
/// are ignored.
bool schema_equal( const char* schema1, size_t size1, const char* schema2, size_t size2 );
}  // namespace internal
}  // namespace flatdata
