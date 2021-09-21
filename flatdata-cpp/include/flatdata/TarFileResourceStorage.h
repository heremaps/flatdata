/**
 * Copyright (c) 2021 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "MemoryMappedTarFileStorage.h"
#include "ResourceStorage.h"

#include <boost/filesystem.hpp>

#include <fstream>

namespace flatdata
{
/**
 * @brief Read-only resource storage for reading flatdata archives inside a TAR file.
 */
class TarFileResourceStorage : public ResourceStorage
{
public:
    /**
     * @brief Create resource storage for a TAR file
     * @param tar_path The path to the TAR file
     * @param tar_path The path inside the TAR file
     * @return TarFileResourceStorage or nullptr on error
     */
    static std::unique_ptr< TarFileResourceStorage > create( const char* tar_path,
                                                             const char* sub_path = "" );

    std::unique_ptr< ResourceStorage > create_directory( const char* key ) override;
    std::unique_ptr< ResourceStorage > directory( const char* key ) override;
    bool exists( const char* key ) override;

protected:
    std::shared_ptr< std::ostream > create_output_stream( const char* key ) override;
    MemoryDescriptor read_resource( const char* key ) override;

private:
    TarFileResourceStorage( std::shared_ptr< const MemoryMappedTarFileStorage > storage,
                            const std::string& tar_path,
                            const std::string& sub_path );
    std::string get_path( const char* key ) const;

private:
    std::shared_ptr< const MemoryMappedTarFileStorage > m_storage;
    std::string m_tar_path;
    std::string m_sub_path;
};

// -------------------------------------------------------------------------------------------------

inline std::unique_ptr< TarFileResourceStorage >
TarFileResourceStorage::create( const char* tar_path, const char* sub_path )
{
    std::shared_ptr< const MemoryMappedTarFileStorage > storage;
    try
    {
        storage.reset( new MemoryMappedTarFileStorage( tar_path ) );
    }
    catch ( const std::runtime_error& e )
    {
        std::clog << e.what( ) << std::endl;
        return nullptr;
    }

    return std::unique_ptr< TarFileResourceStorage >(
        new TarFileResourceStorage( storage, tar_path, sub_path ) );
}

inline std::shared_ptr< std::ostream >
TarFileResourceStorage::create_output_stream( const char* )
{
    // Writing to TAR files is not supported
    return nullptr;
}

inline TarFileResourceStorage::TarFileResourceStorage(
    std::shared_ptr< const MemoryMappedTarFileStorage > storage,
    const std::string& tar_path,
    const std::string& sub_path )
    : m_storage( std::move( storage ) )
    , m_tar_path( tar_path )
    , m_sub_path( sub_path )
{
}

inline std::string
TarFileResourceStorage::get_path( const char* key ) const
{
    const char TAR_PATH_SEPARATOR = '/';

    return m_sub_path.empty( ) ? std::string( key ) : m_sub_path + TAR_PATH_SEPARATOR + key;
}

inline MemoryDescriptor
TarFileResourceStorage::read_resource( const char* key )
{
    if ( !exists( key ) )
    {
        return MemoryDescriptor( );
    }
    return m_storage->read( get_path( key ).c_str( ) );
}

inline std::unique_ptr< ResourceStorage >
TarFileResourceStorage::create_directory( const char* key )
{
    return directory( key );
}

inline std::unique_ptr< ResourceStorage >
TarFileResourceStorage::directory( const char* key )
{
    return std::unique_ptr< TarFileResourceStorage >(
        new TarFileResourceStorage( m_storage, m_tar_path, get_path( key ) ) );
}

inline bool
TarFileResourceStorage::exists( const char* key )
{
    return m_storage->read( get_path( key ).c_str( ) ).data( ) != nullptr;
}

}  // namespace flatdata
