/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "MemoryMappedFileStorage.h"
#include "ResourceStorage.h"

#include <boost/filesystem.hpp>

#include <fstream>
#include <mutex>

namespace flatdata
{
class FileResourceStorage : public ResourceStorage
{
public:
    /**
     * @brief Create resource storage at a given path
     * @return FileResourceStorage or nullptr on error
     */
    static std::unique_ptr< FileResourceStorage > create( const char* path );

    /**
     * @brief Open resource storage at a given path
     * @return FileResourceStorage or nullptr on error
     */
    static std::unique_ptr< FileResourceStorage > open( const char* path );

    /**
     * @brief Creates a directory at a given key and returns it as a resource storage
     */
    std::unique_ptr< ResourceStorage > create_directory( const char* key ) override;

    /**
     * @brief Opens a directory at a given key and returns it as a resource storage
     */
    std::unique_ptr< ResourceStorage > directory( const char* key ) override;

    bool exists( const char* key ) override;

protected:
    std::shared_ptr< std::ostream > create_output_stream( const char* key ) override;
    MemoryDescriptor read_resource( const char* key ) override;

private:
    static std::unique_ptr< FileResourceStorage > create( const char* path,
                                                          bool create_on_missing );
    explicit FileResourceStorage( const std::string& path );
    std::string get_path( const char* key ) const;

private:
    std::unique_ptr< MemoryMappedFileStorage > m_storage;
    std::mutex m_storage_mutex;
    std::string m_path;
};

// -------------------------------------------------------------------------------------------------

inline std::unique_ptr< FileResourceStorage >
FileResourceStorage::create( const char* path )
{
    return FileResourceStorage::create( path, /*create_on_missing*/ true );
}

inline std::unique_ptr< FileResourceStorage >
FileResourceStorage::open( const char* path )
{
    return FileResourceStorage::create( path, /*create_on_missing*/ false );
}

inline std::unique_ptr< FileResourceStorage >
FileResourceStorage::create( const char* path, bool create_on_missing )
{
    boost::filesystem::path p( path );
    if ( p.filename( ).string( ) != "." )
    {
        p += boost::filesystem::path::preferred_separator;
    }

    boost::system::error_code ec;
    boost::filesystem::is_directory( p, ec );
    if ( ec )
    {
        if ( !create_on_missing )
        {
            return nullptr;
        }

        boost::filesystem::create_directory( p, ec );
        if ( ec )
        {
            return nullptr;
        }
    }

    return std::unique_ptr< FileResourceStorage >( new FileResourceStorage( p.string( ) ) );
}

inline std::shared_ptr< std::ostream >
FileResourceStorage::create_output_stream( const char* key )
{
    return std::shared_ptr< std::ostream >(
        new std::ofstream( get_path( key ).c_str( ), std::ofstream::out | std::ofstream::binary ) );
}

inline FileResourceStorage::FileResourceStorage( const std::string& path )
    : m_storage( new MemoryMappedFileStorage )
    , m_path( path )
{
}

inline std::string
FileResourceStorage::get_path( const char* key ) const
{
    return std::string( m_path ) + key;
}

inline MemoryDescriptor
FileResourceStorage::read_resource( const char* key )
{
    if ( !exists( key ) )
    {
        return MemoryDescriptor( );
    }
    std::lock_guard<std::mutex> lock(m_storage_mutex);
    return m_storage->read( get_path( key ).c_str( ) );
}

inline std::unique_ptr< ResourceStorage >
FileResourceStorage::create_directory( const char* key )
{
    return FileResourceStorage::create( std::string( m_path + key ).c_str( ) );
}

inline std::unique_ptr< ResourceStorage >
FileResourceStorage::directory( const char* key )
{
    return FileResourceStorage::open( std::string( m_path + key ).c_str( ) );
}

inline bool
FileResourceStorage::exists( const char* key )
{
    boost::system::error_code ec;
    boost::filesystem::exists( get_path( key ).c_str( ), ec );
    return !ec;
}

}  // namespace flatdata
