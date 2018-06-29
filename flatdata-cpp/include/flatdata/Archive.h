/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "ResourceStorage.h"

#include <boost/optional.hpp>

#include <iomanip>
#include <iostream>
#include <memory>

namespace flatdata
{
/**
 * @brief Base class for BRF archive. Implements functionality common for all archives.
 */
class Archive
{
public:
    virtual ~Archive( ) = default;

    /**
     * @brief Construct an uninitialized archive
     */
    Archive( ) = default;

    /**
     * @brief Construct archive with a given storage
     */
    explicit Archive( std::shared_ptr< flatdata::ResourceStorage > storage );

    /**
     * @brief Returns true if archive is correctly loaded
     */
    explicit operator bool( ) const;

    /**
     * @brief Returns true if archive is correctly loaded
     */
    bool is_open( ) const;

    /**
     * @brief Returns text description of the archive and its resources' state.
     */
    std::string describe( ) const;

    /**
     * @brief Returns archive name. Is implemented by the concrete archive instances.
     */
    virtual const char* name( ) const = 0;

    /**
     * @brief Returns archive schema. Is implemented by the concrete archive instances.
     */
    virtual const char* schema( ) const = 0;

protected:
    template < typename ResourceType >
    static void describe_resource( std::ostream& stream,
                                   const char* name,
                                   const ResourceType& resource );

    template < typename ResourceType >
    static void describe_resource( std::ostream& stream,
                                   const char* name,
                                   const boost::optional< ResourceType >& resource );

protected:
    /**
     * Initialize the flatdata archive.
     * @return true on success, otherwise - false.
     */
    bool initialize( );

    template < typename ResourceType >
    void read_resource( bool& is_open,
                        ResourceType& resource,
                        const char* name,
                        const char* schema );

    template < typename ResourceType >
    void read_resource( bool& is_open,
                        boost::optional< ResourceType >& resource,
                        const char* name,
                        const char* schema );

    template < typename ArchiveType >
    void load_archive( bool& is_open, ArchiveType& archive, const char* name );

    template < typename ArchiveType >
    void load_archive( bool& is_open, boost::optional< ArchiveType >& archive, const char* name );

    /// Get archive storage
    flatdata::ResourceStorage& storage( );
    /// Get archive storage
    const flatdata::ResourceStorage& storage( ) const;

private:
    static void describe_impl(
        std::ostream& stream, const char* name, bool optional, bool loaded, const char* details );

private:
    /**
     * @brief Loads contents of the archive. Is implemented by the concrete archive instances.
     * @return true on success, otherwise - false
     */
    virtual bool load_contents( ) = 0;

    /**
     * @brief Describes all resources provided by the archive.
     * Is implemented by the concrete archive instances.
     */
    virtual void describe_resources( std::ostream& stream ) const = 0;

private:
    std::shared_ptr< flatdata::ResourceStorage > m_storage;
    flatdata::MemoryDescriptor m_signature;
    bool m_is_open = false;
};

// -------------------------------------------------------------------------------------------------

template < typename ResourceType >
void
Archive::describe_resource( std::ostream& stream, const char* name, const ResourceType& resource )
{
    auto initialized = static_cast< bool >( resource );
    describe_impl( stream, name, false, static_cast< bool >( resource ),
                   initialized ? resource.describe( ).c_str( ) : "N/A" );
}

template < typename ResourceType >
void
Archive::describe_resource( std::ostream& stream,
                            const char* name,
                            const boost::optional< ResourceType >& resource )
{
    auto initialized = static_cast< bool >( resource );
    describe_impl( stream, name, true, initialized ? static_cast< bool >( *resource ) : false,
                   initialized ? resource->describe( ).c_str( ) : "N/A" );
}

template < typename ResourceType >
void
Archive::read_resource( bool& is_open,
                        ResourceType& resource,
                        const char* name,
                        const char* schema )
{
    auto result = storage( ).read< ResourceType >( name, schema );
    if ( !result )
    {
        is_open = false;
        return;
    }
    resource = *result;
}

template < typename ResourceType >
void
Archive::read_resource( bool& /*is_open*/,
                        boost::optional< ResourceType >& resource,
                        const char* name,
                        const char* schema )
{
    auto result = storage( ).read< ResourceType >( name, schema );
    if ( result )
    {
        resource = *result;
    }
}

template < typename ArchiveType >
void
Archive::load_archive( bool& is_open, ArchiveType& archive, const char* name )
{
    auto archive_storage = storage( ).directory( name );
    if ( !archive_storage )
    {
        is_open = false;
        return;
    }

    auto result = ArchiveType::open( std::move( archive_storage ) );
    if ( !result )
    {
        is_open = false;
        return;
    }
    archive = result;
}

template < typename ArchiveType >
void
Archive::load_archive( bool& /*is_open*/,
                       boost::optional< ArchiveType >& archive,
                       const char* name )
{
    auto archive_storage = storage( ).directory( name );
    if ( !archive_storage )
    {
        return;
    }
    auto result = ArchiveType::open( std::move( archive_storage ) );
    if ( result )
    {
        archive = result;
    }
}

}  // namespace flatdata
