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
#include <shared_mutex>
#include <iomanip>
#include <optional>

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
    read( MemoryDescriptor data, size_t byte_pos, size_t size_of_struct )
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
            if ( size_of_struct > 0 && size_of_struct < 256
                 && data.data( ) >= previous_mapping->second->data.data( ) + 8 )
            {
                // keep track of struct sizes encountered, unless its at the start (where the
                // resource length is stored and read from)
                previous_mapping->second->struct_sizes[ size_of_struct ]++;
                if ( ( data.data( ) - byte_pos - previous_mapping->second->data.data( )
                       - 8 /*length*/ )
                         % size_of_struct
                     != 0 )
                {
                    // we want to keep track of whether struct sizes where aligned or not, otherwise
                    // we cannot do analysis on it in the end
                    previous_mapping->second->struct_sizes_misalignments++;
                }
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
            size_t last_page = std::numeric_limits< size_t >::max( );
            size_t num_pages = 0;
            // ignore the first 8 bytes, they are containing the size of the resource
            for ( size_t i = 1; i < x->accessed.size( ); i++ )
            {
                size_t num_bits_set = std::bitset< 8 >( x->accessed[ i ] ).count( );
                num_bytes_accessed += num_bits_set;
                size_t page = i / ( ( 1 << 12 ) / 8 );  // 8 bits per byte, 4kb per page
                if ( num_bits_set != 0 && last_page != page )
                {
                    last_page = page;
                    num_pages++;
                }
            }
            // We ignore access to the first 8 bytes (usually the size of the resource)
            if ( num_bytes_accessed <= 8 || x->data.size( ) == 0 )
            {
                continue;
            }
            std::cerr << "    " << x->name << ": " << std::endl;
            std::cerr << "        bytes [count]: " << num_bytes_accessed << " out of "
                      << x->data.size( ) << std::endl;
            std::cerr << "        bytes [accessed]: "
                      << ( num_bytes_accessed * 100.0 / x->data.size( ) ) << "%" << std::endl;
            std::cerr << "        pages [count]: " << num_pages << " out of "
                      << x->data.size( ) / ( 1 << 12 ) << std::endl;
            std::cerr << "        pages [data access]: "
                      << num_bytes_accessed * 100.0 / num_pages / ( 1 << 12 ) << "%" << std::endl;

            // check padding / useless data in structs
            std::optional< size_t > most_used_size = 0;
            size_t sizes_sum = 0;
            for ( size_t i = 0; i < 256; i++ )
            {
                sizes_sum += x->struct_sizes[ i ];
                if ( !most_used_size || x->struct_sizes[ *most_used_size ] < x->struct_sizes[ i ] )
                {
                    most_used_size = i;
                }
            }

            if ( !most_used_size || sizes_sum <= 1
                 || x->struct_sizes_misalignments > sizes_sum / 100
                 || x->struct_sizes[ *most_used_size ] * 1.01 < sizes_sum )
            {
                std::cerr << "        mixed data -> no padding analysis" << std::endl;
            }
            else
            {
                std::vector< size_t > offset_counts( *most_used_size, 0 );
                size_t num_structs = 0;
                for ( size_t pos = 8; pos + *most_used_size <= x->data.size( );
                      pos += *most_used_size )
                {
                    bool struct_accessed = false;
                    for ( size_t struct_pos = pos; struct_pos < pos + *most_used_size;
                          struct_pos++ )
                    {
                        size_t byte_pos = struct_pos / 8;
                        size_t bit_pos = struct_pos % 8;
                        uint8_t BIT_MASK = 1 << bit_pos;
                        struct_accessed |= ( x->accessed[ byte_pos ] & BIT_MASK ) != 0;
                    }
                    if ( struct_accessed )
                    {
                        num_structs++;
                        for ( size_t struct_pos = pos; struct_pos < pos + *most_used_size;
                              struct_pos++ )
                        {
                            size_t byte_pos = struct_pos / 8;
                            size_t bit_pos = struct_pos % 8;
                            uint8_t BIT_MASK = 1 << bit_pos;
                            offset_counts[ struct_pos - pos ]
                                += ( x->accessed[ byte_pos ] & BIT_MASK ) != 0 ? 0 : 1;
                        }
                    }
                }

                std::cerr << "        unused struct members:" << std::endl;
                for ( size_t byte = 0; byte < *most_used_size; byte++ )
                {
                    if ( offset_counts[ byte ] * 100.0 / num_structs > 10 )
                    {
                        std::cerr << "            byte " << std::setw( 3 ) << byte
                                  << " redundant: " << offset_counts[ byte ] * 100.0 / num_structs
                                  << "%" << std::endl;
                    }
                }
            }
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
            , struct_sizes( 256 )
        {
            for ( auto& x : struct_sizes )
            {
                // std::atomtic initialization requires this
                x = 0;
            }
            struct_sizes_misalignments = 0;
        }

        std::atomic< size_t > references;
        std::string name;
        MemoryDescriptor data;
        std::vector< std::atomic< uint8_t > > accessed;
        std::vector< std::atomic< size_t > > struct_sizes;
        std::atomic< size_t > struct_sizes_misalignments;
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
                  + ( sizeof( UnsignedType ) * 8 - BIT_OFFSET < num_bits ? 1 : 0 ) },
            BYTE_OFFSET, struct_size_bytes );

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
        Reader< T, offset, num_bits, struct_size_bytes > start{ data };
        Reader< T, offset, num_bits, struct_size_bytes > end{ data + struct_size_bytes };
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
            return boost::optional< U >( Reader< T, offset, num_bits, struct_size_bytes >{ data } );
        }
    }
};

}  // namespace flatdata
