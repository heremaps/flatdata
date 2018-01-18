/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <cstddef>
#include <cstring>

namespace flatdata
{
/**
 * Utility function to copy the underlying data of a flatdata structure.
 *
 * Note: flatdata structures are handles to memory, in particular their copy operator just copies
 * the handle, and not the data. In the situation when the underlying data should be copied, this
 * function can be used.
 */
template < typename T >
void
copy_struct( T destination, typename T::AccessorType source )
{
    std::memcpy( destination.data( ), source.data( ), T::size_in_bytes( ) );
}
}  // namespace flatdata
