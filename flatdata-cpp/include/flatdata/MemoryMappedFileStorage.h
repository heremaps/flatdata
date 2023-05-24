/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#define BOOST_DATE_TIME_NO_LIB

#include "MemoryDescriptor.h"
#include "DebugDataAccessStatistics.h"

#include <boost/interprocess/file_mapping.hpp>
#include <boost/interprocess/mapped_region.hpp>
#include <boost/optional.hpp>

#include <cstdio>
#include <map>
#include <memory>
#include <tuple>

namespace flatdata
{
class MemoryMappedFileStorage
{
public:
    MemoryDescriptor read( const char* path );
    ~MemoryMappedFileStorage( );

private:
    std::map< std::string, boost::interprocess::mapped_region > m_maps;
};

inline MemoryDescriptor
MemoryMappedFileStorage::read( const char* path )
{
    auto found = m_maps.find( path );
    if ( found != m_maps.end( ) )
    {
        const boost::interprocess::mapped_region& region = found->second;
        return MemoryDescriptor( static_cast< const unsigned char* >( region.get_address( ) ),
                                 region.get_size( ) );
    }

    try
    {
        boost::interprocess::file_mapping file( path, boost::interprocess::read_only );
        boost::interprocess::mapped_region region( file, boost::interprocess::read_only );
        if ( region.get_size( ) == 0 )
        {
            return MemoryDescriptor( );
        }
        MemoryDescriptor result( static_cast< const unsigned char* >( region.get_address( ) ),
                                 region.get_size( ) );
        m_maps.emplace( path, std::move( region ) );
        DebugDataAccessStatistics::register_mapping( path, result );

        return result;
    }
    catch ( boost::interprocess::interprocess_exception& )
    {
        return MemoryDescriptor( );
    }
}

inline MemoryMappedFileStorage::~MemoryMappedFileStorage( )
{
    for ( const auto& [ key, value ] : m_maps )
    {
        MemoryDescriptor mapping( static_cast< const unsigned char* >( value.get_address( ) ),
                                  value.get_size( ) );
        DebugDataAccessStatistics::deregister_mapping( mapping );
    }
}

}  // namespace flatdata
