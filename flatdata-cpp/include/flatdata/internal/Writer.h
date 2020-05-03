/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "BitHelpers.h"
#include "Reader.h"

#include <boost/endian/conversion.hpp>

#include <cassert>
#include <type_traits>

namespace flatdata
{
/**
 * This class allows reading/writing integers/booleans/enumeration to a bitstream
 * Its data member is shared with other instances within the same structure by being part of the
 *     same union.
 * Data is encoded in little-endian
 * Data is written to [data + offset, data + offset + num_bits) bit positions
 *
 * @note The class expects data streams with 8 byte padding in the end when reading/writing
 */
template < typename T, int offset = 0, int num_bits = sizeof( T ) * 8, int struct_size_bytes = 0 >
struct Writer
{
    using StreamType = unsigned char*;
    using UnsignedType = typename BitsToUnsigned<
        int_choice< num_bits, num_bits + offset % 8, num_bits + offset % 8 <= 64 >::value >::type;
    enum
    {
        bit_width = num_bits
    };

    StreamType data;

    Writer&
    operator=( T t )
    {
        /* Does the following:
         * - takes the smallest data type available that can write the necessary amount of bytes
         *   (including offset in the beginning and empty space in the end)
         * - uses that data type to read/write the main part of the data
         * - In case of 64 bit numbers one byte could be missing in the data (e.g. unaligned 64 bit
         *   integer), and one more byte is read/written
         * - Previous data has to be erased with a mask
         * - Surrounding data of non-aligned integers is preserved
         */
        auto destination = data + offset / 8;
        static const int BIT_OFFSET = offset % 8;

        auto value_to_store = static_cast< UnsignedType >( t );
        if ( std::is_signed< T >::value )
        {
            // negative numbers are stored as 2-complement, and we need to remove upper bits
            value_to_store = masked< num_bits >( value_to_store );
        }
        UnsignedType value_mask = masked< num_bits >( static_cast< UnsignedType >( -1 ) );
        UnsignedType value;
        // read previous data
        std::memcpy( &value, destination, sizeof( UnsignedType ) );
        boost::endian::little_to_native_inplace( value );
        // remove previous value, but keep surrounding data
        value &= ~( value_mask << BIT_OFFSET );
        // add new value, and keep surrounding data
        value |= value_to_store << BIT_OFFSET;
        // write back new data
        boost::endian::native_to_little_inplace( value );
        std::memcpy( destination, &value, sizeof( UnsignedType ) );
        static const int BATCHED_BITS_WRITTEN = sizeof( UnsignedType ) * 8 - BIT_OFFSET;
        // one byte might be missing
        if ( BATCHED_BITS_WRITTEN < num_bits )
        {
            destination += sizeof( UnsignedType );
            value_to_store >>= BATCHED_BITS_WRITTEN % ( sizeof( UnsignedType ) * 8 );
            value_mask >>= BATCHED_BITS_WRITTEN % ( sizeof( UnsignedType ) * 8 );
            // remove previous value, but keep surrounding data
            unsigned char byte_value = static_cast< unsigned char >( *destination & ~value_mask );
            // add new value, and keep surrounding data
            *destination = byte_value | static_cast< unsigned char >( value_to_store );
        }
        return *this;
    }

    operator T( ) const
    {
        return Reader< T, offset, num_bits >{data};
    }
};

template < typename T, int offset, int num_bits, int struct_size_bytes >
struct Writer< std::pair< T, T >, offset, num_bits, struct_size_bytes >
{
    using StreamType = unsigned char*;

    StreamType data;

    // We do not support writing to a range, just reading it after the fact
};

template < typename T, T INVALID_VALUE, int offset, int num_bits, int struct_size_bytes >
struct Writer< Tagged< T, INVALID_VALUE >, offset, num_bits, struct_size_bytes >
{
    using StreamType = unsigned char*;

    StreamType data;

    explicit operator bool( ) const
    {
        return static_cast< bool >( Reader< Tagged< T, INVALID_VALUE >, offset, num_bits >{data} );
    }

    T operator*( ) const
    {
        return *Reader< Tagged< T, INVALID_VALUE >, offset, num_bits >{data};
    }

    operator boost::optional< T >( ) const
    {
        return Reader< Tagged< T, INVALID_VALUE >, offset, num_bits >{data};
    }

    Writer&
    operator=( T t )
    {
        Writer< T, offset, num_bits >{data} = t;
        return *this;
    }

    Writer&
    operator=( boost::optional< T > t )
    {
        if ( t )
        {
            Writer< T, offset, num_bits >{data} = t;
        }
        else
        {
            Writer< T, offset, num_bits >{data} = INVALID_VALUE;
        }
    }
};

}  // namespace flatdata
