/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "BitHelpers.h"
#include "functional/Tagged.h"

#include <boost/endian/conversion.hpp>
#include <boost/optional.hpp>
#include <boost/optional/optional_io.hpp>

#include <cstring>
#include <utility>

#include <flatdata/MemoryDescriptor.h>

#include <mutex>
#include <map>
#include <atomic>
#include <vector>
#include <memory>
#include <iostream>
#include <bitset>
#include <cassert>

namespace flatdata
{
struct DebugDataAccessStatistics
{
    static void
    register_mapping( const char* name, MemoryDescriptor data )
    {
        auto& self = instance( );
        std::vector< std::atomic< uint8_t > > accessed( ( data.size( ) + 7 ) / 8 );
        for ( auto& x : accessed )
        {
            // We have to initialize the vector this way since atomic is not copyable/moveable
            x = 0;
        }
        auto mapping = std::make_shared< Mapping >( name, data, std::move( accessed ) );
        std::unique_lock lock( self.m_mutex );
        // it's ok if we fail to insert here, we just take the data the another thread inserted)
        if ( auto [ iter, was_inserted ]
             = self.m_active_mappings.try_emplace( data.data( ), mapping );
             !was_inserted )
        {
            iter->second->references++;
        }
    }

    static void
    deregister_mapping( MemoryDescriptor data )
    {
        auto& self = instance( );
        std::unique_lock lock( self.m_mutex );
        if ( auto iter = self.m_active_mappings.find( data.data( ) );
             iter != self.m_active_mappings.end( ) )
        {
            if ( --iter->second->references == 0 )
            {
                self.m_inactive_mappings.emplace_back(
                    self.m_active_mappings.extract( iter ).mapped( ) );
            }
        }
        else
        {
            std::cerr << "Error: Trying to unregister mapping in DebugDataAccessStatistics that "
                         "was not registered"
                      << std::endl;
        }
    }

    static void
    read( MemoryDescriptor data )
    {
        auto& self = instance( );
        std::shared_lock lock( self.m_mutex );
        auto previous_mapping = self.m_active_mappings.upper_bound( data.data( ) );
        bool found = false;
        if ( !self.m_active_mappings.empty( )
             && previous_mapping != self.m_active_mappings.begin( ) )
        {
            previous_mapping--;
            if ( previous_mapping->second->data.data( ) > data.data( ) )
            {
                std::cerr << "ARAGHS" << std::endl;
            }
            if ( previous_mapping->second->data.data( ) + previous_mapping->second->data.size( )
                 >= data.data( ) + data.size( ) )
            {
                found = true;
            }
        }
        if ( found )
        {
            for ( size_t i = 0; i < data.size( ); i++ )
            {
                size_t pos = data.data( ) - previous_mapping->second->data.data( ) + i;
                size_t byte_pos = pos / 8;
                size_t bit_pos = pos % 8;
                uint8_t BIT_MASK = 1 << bit_pos;
                previous_mapping->second->accessed[ byte_pos ] |= BIT_MASK;
            }
        }
        else
        {
            self.m_unregistered_bytes_read += data.size( );
        }
    }

    ~DebugDataAccessStatistics( )
    {
        std::cerr << "===== flatdata debug statistics =====" << std::endl;
        std::cerr << "Usage statistics:" << std::endl;
        for ( auto& x : m_inactive_mappings )
        {
            size_t num_bytes_accessed = 0;
            for ( auto& bits : x->accessed )
            {
                num_bytes_accessed += std::bitset< 8 >( bits ).count( );
            }
            // We ignore access to the first 8 bytes (usually the size of the resource)
            if ( num_bytes_accessed <= 8 || x->data.size( ) == 0 )
            {
                continue;
            }
            std::cerr << "    " << x->name << ": " << num_bytes_accessed << " out of "
                      << x->data.size( ) << " (" << ( num_bytes_accessed * 100.0 / x->data.size( ) )
                      << "%)" << std::endl;
        }
        if ( !m_active_mappings.empty( ) )
        {
            std::cerr << "Resources not properly closed:" << std::endl;
            for ( auto& x : m_active_mappings )
            {
                std::cerr << "    " << x.second->name << std::endl;
            }
        }
        if ( m_unregistered_bytes_read != 0 )
        {
            std::cerr << "Unregistered data reads:" << std::endl;
            std::cerr << "    " << m_unregistered_bytes_read << std::endl;
        }
    }

private:
    static DebugDataAccessStatistics&
    instance( )
    {
        static DebugDataAccessStatistics self;
        return self;
    }

    struct Mapping
    {
        Mapping( std::string name,
                 MemoryDescriptor data,
                 std::vector< std::atomic< uint8_t > > accessed )
            : references( 1 )
            , name( std::move( name ) )
            , data( data )
            , accessed( std::move( accessed ) )
        {
        }

        std::atomic< size_t > references;
        std::string name;
        MemoryDescriptor data;
        std::vector< std::atomic< uint8_t > > accessed;
    };

    mutable std::shared_mutex m_mutex;
    mutable std::map< const uint8_t*, std::shared_ptr< Mapping > > m_active_mappings;
    mutable std::vector< std::shared_ptr< Mapping > > m_inactive_mappings;
    mutable std::atomic< size_t > m_unregistered_bytes_read;
};

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

    enum
    {
        bit_width = num_bits
    };
    enum : typename UnderlyingType< T >::type
    {
        max = bit_width == 0 ? 0
                             : ( bit_width - std::is_signed< T >::value
                                         == sizeof( typename BitsToUnsigned< num_bits >::type ) * 8
                                     ? typename BitsToUnsigned< num_bits >::type( -1 )
                                     : ( typename BitsToUnsigned< num_bits >::type( 1 )
                                         << ( bit_width - std::is_signed< T >::value ) )
                                           - 1 ),
        min = bit_width == 0 || !std::is_signed< T >::value
                  ? static_cast< typename UnderlyingType< T >::type >( 0 )
                  : static_cast< typename UnderlyingType< T >::type >( -max - 1 )
    };

    StreamType data;

    operator T( ) const
    {
        using UnsignedType =
            typename BitsToUnsigned< int_choice< num_bits, num_bits + offset % 8,
                                                 num_bits + offset % 8 <= 64 >::value >::type;

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

        DebugDataAccessStatistics::read(
            { data + BYTE_OFFSET,
              sizeof( UnsignedType )
                  + ( sizeof( UnsignedType ) * 8 - BIT_OFFSET < num_bits ? 1 : 0 ) } );

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
        Reader< T, offset, num_bits > start{ data };
        Reader< T, offset, num_bits > end{ data + struct_size_bytes };
        return std::pair< T, T >( start, end );
    }
};

template < typename T, T INVALID_VALUE, int offset, int num_bits, int struct_size_bytes >
struct Reader< Tagged< T, INVALID_VALUE >, offset, num_bits, struct_size_bytes >
{
    using StreamType = const unsigned char*;

    StreamType data;

    explicit operator bool( ) const
    {
        return Reader< T, offset, num_bits >{ data } != INVALID_VALUE;
    }

    T
    operator*( ) const
    {
        return Reader< T, offset, num_bits >{ data };
    }

    template < typename U >
    operator boost::optional< U >( ) const
    {
        if ( Reader< T, offset, num_bits >{ data } == INVALID_VALUE )
        {
            return { };
        }
        else
        {
            return boost::optional< U >( Reader< T, offset, num_bits >{ data } );
        }
    }
};

}  // namespace flatdata
