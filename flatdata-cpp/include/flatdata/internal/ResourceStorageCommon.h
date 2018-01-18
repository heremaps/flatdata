/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "Writer.h"
#include "Constants.h"

#include <cstdint>
#include <ostream>

namespace flatdata
{
namespace resource_storage
{
using size_type = uint64_t;

template < typename T >
void
write_to_stream( std::ostream& stream, T value )
{
    unsigned char data[ sizeof( T ) ] = {0};
    Writer< T > writer{data};
    writer = value;
    stream.write( reinterpret_cast< const char* >( data ), sizeof( T ) );
}

inline void
write_padding( std::ostream& stream )
{
    char zero[ PADDING_SIZE ] = {0};
    stream.write( zero, PADDING_SIZE );
}

}  // namespace resource_storage
}  // namespace flatdata
