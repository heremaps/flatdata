/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <boost/functional/hash.hpp>
#include <cstddef>

namespace flatdata
{
/**
 * Hash function for a flatdata structure.
 *
 * Example:
 *
 *    std::unordered_set< flatdata::SomeStruct, flatdata::Hash > set;
 *    std::unordered_map< flatdata::SomeStruct, uint32_t, flatdata::Hash > map;
 *
 */
struct Hash
{
    template < typename T >
    size_t operator( )( const T& value ) const
    {
        std::size_t hash = 0;
        auto data = value.data( );
        auto size = value.size_in_bytes( );
        for ( size_t i = 0; i < size; i++ )
        {
            boost::hash_combine( hash, data[ i ] );
        }

        return hash;
    }
};
}  // namespace flatdata
