/**
 * Copyright (c) 2021 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "MemoryDescriptor.h"
#include "internal/TarReader.h"

#include <boost/interprocess/file_mapping.hpp>
#include <boost/interprocess/mapped_region.hpp>

#include <cstdio>
#include <map>
#include <memory>

namespace flatdata
{
class MemoryMappedTarFileStorage
{
public:
    explicit MemoryMappedTarFileStorage( const char* tar_path );

    MemoryDescriptor read( const char* path ) const;

private:
    boost::interprocess::mapped_region m_region;
    std::map< std::string, MemoryDescriptor > m_files;
};

inline MemoryMappedTarFileStorage::MemoryMappedTarFileStorage( const char* tar_path )
{
    try
    {
        boost::interprocess::file_mapping file( tar_path, boost::interprocess::read_only );
        boost::interprocess::mapped_region region( file, boost::interprocess::read_only );
        if ( region.get_size( ) == 0 )
        {
            return;
        }

        m_region = std::move( region );
    }
    catch ( boost::interprocess::interprocess_exception& )
    {
        return;
    }

    MemoryDescriptor tar_archive( static_cast< const unsigned char* >( m_region.get_address( ) ),
                                  m_region.get_size( ) );
    std::vector< internal::TarFileEntry > file_entries;
    try
    {
        file_entries = internal::read_tar_file_entries( tar_archive );
    }
    catch ( const std::runtime_error& e )
    {
        throw std::runtime_error( std::string( "Error reading TAR archive: " ) + e.what( ) );
    }

    for ( const auto& file : file_entries )
    {
        std::string path = file.name.substr( 0, 2 ) == "./" ? file.name.substr( 2 ) : file.name;
        m_files.emplace(
            std::move( path ),
            MemoryDescriptor(
                static_cast< const unsigned char* >( m_region.get_address( ) ) + file.offset,
                std::min( file.size, m_region.get_size( ) - file.offset ) ) );
    }
}

inline MemoryDescriptor
MemoryMappedTarFileStorage::read( const char* path ) const
{
    auto found = m_files.find( path );
    if ( found != m_files.end( ) )
    {
        return found->second;
    }

    return MemoryDescriptor( );
}

}  // namespace flatdata
