/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#ifdef DEBUG_DATA_ACCESS_STATISTICS

#include <flatdata/MemoryDescriptor.h>

#include <atomic>
#include <bitset>
#include <iomanip>
#include <iostream>
#include <map>
#include <memory>
#include <mutex>
#include <optional>
#include <shared_mutex>
#include <utility>
#include <vector>

namespace flatdata
{
/*
 * This class provides the ability to analyse data access patterns by logging all data access
 * to flatdata resources via `Reader`s (usually most data except raw_data).
 * This requires a compile time flag (FLATDATA_ENABLE_DEBUG_DATA_ACCESS_ANALYSIS)
 * Note: This requires a lot of RAM: 1 byte for every 8 byte each resource consumes
 *       E.g. to analyse access to a 16GB archive you need an additional 2GB RAM.
 */
struct DebugDataAccessStatistics
{
    /*
     * Registers a resource mapping with this class. Afterwards calls to `read` will record access
     * to this resource.
     */
    static void
    register_mapping( const char* name, MemoryDescriptor data )
    {
        auto& self = instance( );
        auto mapping = std::make_shared< Mapping >( name, data );
        std::unique_lock lock( self.m_mutex );
        // it's ok if we fail to insert here, we just take the data the another thread inserted)
        if ( auto [ iter, was_inserted ]
             = self.m_active_mappings.try_emplace( data.data( ), mapping );
             !was_inserted )
        {
            iter->second->references++;
        }
    }

    /*
     * Deregisters a previously registered resource.
     */
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
            std::cerr << "Error: Trying to deregister mapping in DebugDataAccessStatistics that "
                         "was not registered"
                      << std::endl;
        }
    }

    /*
     * Registers a data read from a specific location. If the read comes from a registered resource
     * it will mark the appropriate bytes in the resource as read.
     */
    static void
    read( MemoryDescriptor data, size_t byte_pos, size_t size_of_struct )
    {
        if ( data.size( ) == 0 )
        {
            return;
        }
        auto& self = instance( );
        std::shared_lock lock( self.m_mutex );

        // Look for the mapping that contains this memory region completely
        auto previous_mapping = self.m_active_mappings.upper_bound( data.data( ) );
        bool found = false;
        if ( !self.m_active_mappings.empty( )
             && previous_mapping != self.m_active_mappings.begin( ) )
        {
            previous_mapping--;
            if ( previous_mapping->second->data.data( ) + previous_mapping->second->data.size( )
                 >= data.data( ) + data.size( ) )
            {
                found = true;
            }
        }
        if ( !found )
        {
            self.m_unregistered_bytes_read += data.size( );
            return;
        }

        // mark all bytes `data` points to as read
        // process reads up to 64 bytes at once. Most reads access at most 2 chunks (all
        // structure members are at most 64 bit)
        size_t offset = data.data( ) - previous_mapping->second->data.data( );
        BitsetType chunk = 0;
        size_t chunk_pos = offset / BITS_PER_ITEM;
        for ( size_t i = 0; i < data.size( ); i++ )
        {
            size_t pos = offset + i;
            size_t item_pos = pos / BITS_PER_ITEM;
            if ( item_pos != chunk_pos )
            {
                previous_mapping->second->accessed[ chunk_pos ] |= chunk;
                chunk = 0;
                chunk_pos = item_pos;
            }
            size_t bit_pos = pos % BITS_PER_ITEM;
            BitsetType bit_mask = static_cast< BitsetType >( 1 ) << bit_pos;
            chunk |= bit_mask;
        }
        previous_mapping->second->accessed[ chunk_pos ] |= chunk;

        // Mark the struct size that was used for later analysis
        if ( size_of_struct > 0 && size_of_struct < MAX_STRUCT_SIZE
             && data.data( ) >= previous_mapping->second->data.data( ) + 8 )
        {
            // keep track of struct sizes encountered, unless its at the start (where the
            // resource length is stored and read from)
            previous_mapping->second->struct_sizes[ size_of_struct ]++;
            if ( ( data.data( ) - byte_pos - previous_mapping->second->data.data( ) - 8 /*length*/ )
                     % size_of_struct
                 != 0 )
            {
                // we want to keep track of whether struct sizes where aligned or not, otherwise
                // we cannot do analysis on it in the end
                previous_mapping->second->struct_sizes_misalignments++;
            }
        }
    }

    /*
     * Resets statistics. Does not change which resources are registers with this class.
     * This can be useful if you want to isolate some warmup / startup period from the rest of the
     * task.
     */
    static void
    clear( )
    {
        auto& self = instance( );
        std::unique_lock lock( self.m_mutex );
        self.m_inactive_mappings.clear( );
        for ( auto& mapping : self.m_active_mappings )
        {
            for ( auto& x : mapping.second->accessed )
            {
                x = 0;
            }
            for ( auto& x : mapping.second->struct_sizes )
            {
                x = 0;
            }
            mapping.second->struct_sizes_misalignments = 0;
        }
    }

    /*
     * Statistics are printed on shutdown. Only statistics of resources that have been deregistered
     * are reported.
     */
    ~DebugDataAccessStatistics( )
    {
        // compute number of pages accessed for each mapping and sort by that
        size_t sum_pages = 0;
        for ( auto& x : m_inactive_mappings )
        {
            size_t last_page = std::numeric_limits< size_t >::max( );
            for ( size_t i = 0; i < x->accessed.size( ); i++ )
            {
                size_t num_bits_set = std::bitset< BITS_PER_ITEM >( x->accessed[ i ] ).count( );
                x->num_bytes_accessed += num_bits_set;
                size_t page = i / ( PAGE_SIZE / BITS_PER_ITEM );
                if ( num_bits_set != 0 && last_page != page )
                {
                    last_page = page;
                    x->pages.emplace_back( page );
                    sum_pages++;
                }
            }
        }
        std::stable_sort(
            m_inactive_mappings.begin( ), m_inactive_mappings.end( ),
            []( auto& left, auto& right ) { return left->pages.size( ) > right->pages.size( ); } );

        std::cerr << "===== flatdata debug statistics =====" << std::endl;
        std::cerr << "Total pages: " << sum_pages << std::endl;
        std::cerr << "Usage statistics:" << std::endl;

        for ( auto& x : m_inactive_mappings )
        {
            // We ignore access to the first 8 bytes (usually the size of the resource)
            if ( x->num_bytes_accessed <= 8 || x->data.size( ) == 0 )
            {
                continue;
            }
            std::cerr << "    " << x->name << "(" << x->pages.size( ) * 100.0 / sum_pages
                      << "% of pages): " << std::endl;
            std::cerr << "        bytes [count]: " << x->num_bytes_accessed << " out of "
                      << x->data.size( ) << std::endl;
            std::cerr << "        bytes [accessed]: "
                      << ( x->num_bytes_accessed * 100.0 / x->data.size( ) ) << "%" << std::endl;
            std::cerr << "        pages [count]: " << x->pages.size( ) << " out of "
                      << x->data.size( ) / PAGE_SIZE << std::endl;
            std::cerr << "        pages [data access]: "
                      << x->num_bytes_accessed * 100.0 / x->pages.size( ) / PAGE_SIZE << "%"
                      << std::endl;

            // check padding / useless data in structs
            std::optional< size_t > most_used_size = 0;
            size_t sizes_sum = 0;
            for ( size_t i = 0; i < MAX_STRUCT_SIZE; i++ )
            {
                sizes_sum += x->struct_sizes[ i ];
                if ( !most_used_size || x->struct_sizes[ *most_used_size ] < x->struct_sizes[ i ] )
                {
                    most_used_size = i;
                }
            }

            if ( !most_used_size || sizes_sum <= 1
                 || x->struct_sizes_misalignments > sizes_sum / 100
                 || x->struct_sizes[ *most_used_size ] * 1.01 < sizes_sum || x->pages.empty( ) )
            {
                std::cerr << "        mixed data -> no padding analysis" << std::endl;
                continue;
            }

            // Analys which struct bytes were not read and might be redundant (or better decoupled
            // from the others)
            std::vector< size_t > offset_counts( *most_used_size, 0 );
            size_t num_structs = 0;
            size_t current_page = 0;
            for ( size_t pos = 8; pos + *most_used_size <= x->data.size( ); pos += *most_used_size )
            {
                // performance: we use the page data to skip structs that were never read
                size_t start_page = pos / PAGE_SIZE;
                if ( start_page + 1 < x->pages[ current_page ] )
                {
                    continue;  // no data to be found here
                }
                if ( start_page > x->pages[ current_page ] )
                {
                    if ( ++current_page >= x->pages.size( ) )
                    {
                        break;
                    }
                }

                // Check if any byte was accessed, and if so compute access stats for this
                // instance
                auto compute_stats = [ & ] {
                    num_structs++;
                    for ( size_t struct_pos = pos; struct_pos < pos + *most_used_size;
                          struct_pos++ )
                    {
                        size_t item_pos = struct_pos / BITS_PER_ITEM;
                        size_t bit_pos = struct_pos % BITS_PER_ITEM;
                        BitsetType bit_mask = static_cast< BitsetType >( 1 ) << bit_pos;
                        offset_counts[ struct_pos - pos ]
                            += ( x->accessed[ item_pos ] & bit_mask ) != 0 ? 0 : 1;
                    }
                };
                for ( size_t struct_pos = pos; struct_pos < pos + *most_used_size; struct_pos++ )
                {
                    size_t item_pos = struct_pos / BITS_PER_ITEM;
                    size_t bit_pos = struct_pos % BITS_PER_ITEM;
                    BitsetType bit_mask = static_cast< BitsetType >( 1 ) << bit_pos;
                    if ( ( x->accessed[ item_pos ] & bit_mask ) != 0 )
                    {
                        compute_stats( );
                        break;
                    }
                }
            }

            std::cerr << "        unused struct members:" << std::endl;
            for ( size_t byte = 0; byte < *most_used_size; byte++ )
            {
                // Only output stats in case the byte was unused for at least 25% of the time
                if ( offset_counts[ byte ] * 100.0 / num_structs >= 25 )
                {
                    std::cerr << "            byte " << std::setw( 3 ) << byte
                              << " redundant: " << offset_counts[ byte ] * 100.0 / num_structs
                              << "%" << std::endl;
                }
            }
        }

        // warn about resources not properly closed
        if ( !m_active_mappings.empty( ) )
        {
            std::cerr << "Resources not properly closed:" << std::endl;
            for ( auto& x : m_active_mappings )
            {
                std::cerr << "    " << x.second->name << std::endl;
            }
        }

        // stats about unregistered reads (not necessarily an error/warning)
        if ( m_unregistered_bytes_read != 0 )
        {
            std::cerr << "Unregistered data reads:" << std::endl;
            std::cerr << "    " << m_unregistered_bytes_read << std::endl;
        }
    }

private:
    static constexpr size_t MAX_STRUCT_SIZE = 256;
    static constexpr size_t PAGE_SIZE = 1 << 12;
    using BitsetType = uint32_t;
    static constexpr size_t BITS_PER_ITEM = sizeof( BitsetType ) * 8;

    // Provide access to the global instance of this class
    static DebugDataAccessStatistics&
    instance( )
    {
        static DebugDataAccessStatistics self;
        return self;
    }

    struct Mapping
    {
        Mapping( std::string name, MemoryDescriptor data )
            : references( 1 )
            , name( std::move( name ) )
            , data( data )
            , accessed( ( data.size( ) + BITS_PER_ITEM - 1 ) / BITS_PER_ITEM )
            , struct_sizes( MAX_STRUCT_SIZE )
        {
            // We have to initialize atomic members
            for ( auto& x : accessed )
            {
                x = 0;
            }
            for ( auto& x : struct_sizes )
            {
                x = 0;
            }
            struct_sizes_misalignments = 0;
        }

        // Reference counting in case a resource if registered multiple times
        std::atomic< size_t > references;
        // Name of the resource, e.g. the local path, or file system path
        std::string name;
        // Data region covered by the resource
        MemoryDescriptor data;
        // Bitset marking which bytes have been accessed, concurrent writes possible
        std::vector< std::atomic< BitsetType > > accessed;
        // All struct sizes encountered while reading, is be used to analyse padding/redundant
        // data
        std::vector< std::atomic< size_t > > struct_sizes;
        // How many reads where done to none-aligned structs
        std::atomic< size_t > struct_sizes_misalignments;
        // The pages that were accessed. Only filled while printing statistics.
        std::vector< size_t > pages;
        // How many bytes were accessed. Only filled while printing statistics.
        size_t num_bytes_accessed = 0;
    };

    // We protect modifications to the list of Mappings with the read/write mutex
    mutable std::shared_mutex m_mutex;
    // registered mappings
    mutable std::map< const uint8_t*, std::shared_ptr< Mapping > > m_active_mappings;
    // previously registered mappings
    mutable std::vector< std::shared_ptr< Mapping > > m_inactive_mappings;
    // reads to unregistered locations (e.g. data on the stack)
    mutable std::atomic< size_t > m_unregistered_bytes_read;
};

}  // namespace flatdata

#endif