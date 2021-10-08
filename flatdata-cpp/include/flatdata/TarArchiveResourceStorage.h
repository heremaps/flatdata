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
 * @brief Read-only resource storage for reading flatdata archives inside a TAR archive file.
 */
class TarArchiveResourceStorage : public ResourceStorage
{
public:
    /**
     * @brief Create resource storage for a TAR archive file
     * @param tar_path The path to the TAR archive file
     * @param sub_path The working path inside the TAR archive
     * @return TarArchiveResourceStorage or nullptr on error
     */
    static std::unique_ptr< TarArchiveResourceStorage > create( const char* tar_path,
                                                                const char* sub_path = "" );

    std::unique_ptr< ResourceStorage > create_directory( const char* key ) override;
    std::unique_ptr< ResourceStorage > directory( const char* key ) override;
    bool exists( const char* key ) override;

protected:
    std::shared_ptr< std::ostream > create_output_stream( const char* key ) override;
    MemoryDescriptor read_resource( const char* key ) override;

private:
    TarArchiveResourceStorage( std::shared_ptr< const MemoryMappedTarFileStorage > storage,
                               const std::string& tar_path,
                               const std::string& sub_path );
    std::string get_path( const char* key ) const;

private:
    std::shared_ptr< const MemoryMappedTarFileStorage > m_storage;
    std::string m_tar_path;
    std::string m_sub_path;
};

// -------------------------------------------------------------------------------------------------

inline std::unique_ptr< TarArchiveResourceStorage >
TarArchiveResourceStorage::create( const char* tar_path, const char* sub_path )
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

    return std::unique_ptr< TarArchiveResourceStorage >(
        new TarArchiveResourceStorage( storage, tar_path, sub_path ) );
}

inline std::shared_ptr< std::ostream >
TarArchiveResourceStorage::create_output_stream( const char* )
{
    // Writing to TAR files is not supported
    return nullptr;
}

inline TarArchiveResourceStorage::TarArchiveResourceStorage(
    std::shared_ptr< const MemoryMappedTarFileStorage > storage,
    const std::string& tar_path,
    const std::string& sub_path )
    : m_storage( std::move( storage ) )
    , m_tar_path( tar_path )
    , m_sub_path( sub_path )
{
}

inline std::string
TarArchiveResourceStorage::get_path( const char* key ) const
{
    const char TAR_PATH_SEPARATOR = '/';

    return m_sub_path.empty( ) ? std::string( key ) : m_sub_path + TAR_PATH_SEPARATOR + key;
}

inline MemoryDescriptor
TarArchiveResourceStorage::read_resource( const char* key )
{
    if ( !exists( key ) )
    {
        return MemoryDescriptor( );
    }
    return m_storage->read( get_path( key ).c_str( ) );
}

inline std::unique_ptr< ResourceStorage >
TarArchiveResourceStorage::create_directory( const char* key )
{
    return directory( key );
}

inline std::unique_ptr< ResourceStorage >
TarArchiveResourceStorage::directory( const char* key )
{
    return std::unique_ptr< TarArchiveResourceStorage >(
        new TarArchiveResourceStorage( m_storage, m_tar_path, get_path( key ) ) );
}

inline bool
TarArchiveResourceStorage::exists( const char* key )
{
    return m_storage->read( get_path( key ).c_str( ) ).data( ) != nullptr;
}

}  // namespace flatdata
