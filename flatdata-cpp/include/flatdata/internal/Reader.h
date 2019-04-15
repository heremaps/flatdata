/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "BitHelpers.h"

#include <boost/endian/conversion.hpp>

#include <cstring>

namespace flatdata
{
/**
 * This class allows reading integers/booleans/enumeration to a bitstream
 * Its data member is shared with other instances within the same structure by being part of the
 *     same union.
 * Data is encoded in little-endian
 * Data is read from [data + offset, data + offset + num_bits) bit positions
 *
 * @note The class expects data streams with 8 byte padding in the end when reading
 */
template < typename T, int offset = 0, int num_bits = sizeof( T ) * 8, int struct_size_bytes = 0 >
struct Reader
{
    using StreamType = const unsigned char*;
    using UnsignedType = typename BitsToUnsigned<
        int_choice< num_bits, num_bits + offset % 8, num_bits + offset % 8 <= 64 >::value >::type;
    enum
    {
        bit_width = num_bits
    };

    StreamType data;

    operator T( ) const
    {
        /* Does the following:
         * - takes the smallest data type available that can read the necessary amount of bytes
         *   (including offset in the beginning and empty space in the end)
         * - uses that data type to read the main part of the data
         * - In case of 64 bit numbers one byte could be missing in the data (e.g. unaligned 64 bit
         *   integer), and one more byte is read
         * - Previous data has to be erased with a mask
         * - Surrounding data of non-aligned integers is preserved
         */
        static const int BYTE_OFFSET = offset / 8;
        static const int BIT_OFFSET = offset % 8;

        if ( num_bits == 1 )
        {
            return static_cast< T >( ( data[ BYTE_OFFSET ] & ( 1 << BIT_OFFSET ) ) != 0 );
        }

        UnsignedType result;
        std::memcpy( &result, data + BYTE_OFFSET, sizeof( result ) );
        boost::endian::little_to_native_inplace( result );
        result >>= BIT_OFFSET;
        if ( sizeof( UnsignedType ) * 8 - BIT_OFFSET < num_bits )
        {
            UnsignedType temp = data[ BYTE_OFFSET + sizeof( UnsignedType ) ];
            result |= temp << ( ( sizeof( UnsignedType ) * 8 - BIT_OFFSET )
                                % ( sizeof( UnsignedType ) * 8 ) );
        }
        result = masked< num_bits >( result );
        return extend_sign< T, num_bits >( result );
    }

    template < typename U >
    U
    as( ) const
    {
        return static_cast< U >( this->operator T( ) );
    }
};

template < typename T, int offset, int num_bits, int struct_size_bytes >
struct Reader< std::pair< T, T >, offset, num_bits, struct_size_bytes >
{
    using StreamType = const unsigned char*;

    StreamType data;

    template < typename U >
    operator std::pair< U, U >( ) const
    {
        Reader< T, offset, num_bits > start{data};
        Reader< T, offset, num_bits > end{data + struct_size_bytes};
        return std::pair< T, T >( start, end );
    }
};

}  // namespace flatdata
