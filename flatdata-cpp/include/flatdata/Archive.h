/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

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
    std::string describe( size_t nest_level = 0u ) const;

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
    static void describe_resource( size_t nest_level,
                                   std::ostream& stream,
                                   const char* name,
                                   const ResourceType& resource,
                                   bool too_large = false );

    template < typename ResourceType >
    static void describe_resource( size_t nest_level,
                                   std::ostream& stream,
                                   const char* name,
                                   const boost::optional< ResourceType >& resource,
                                   bool too_large = false );

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
    static void describe_impl( std::ostream& stream,
                               const char* name,
                               bool optional,
                               bool loaded,
                               const char* details,
                               bool are_details_nested,
                               bool too_large,
                               size_t nest_level );

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
    virtual void describe_resources( std::ostream& stream, size_t nest_level ) const = 0;

private:
    std::shared_ptr< flatdata::ResourceStorage > m_storage;
    flatdata::MemoryDescriptor m_signature;
    bool m_is_open = false;
};

// -------------------------------------------------------------------------------------------------

template < typename ResourceType >
std::string
get_description( const ResourceType& resource, bool is_archive, size_t nest_level )
{
    std::string description;
    if ( is_archive )
    {
        ++nest_level;
    }
    description = resource.describe( nest_level );
    return description;
}

template < typename ResourceType >
void
Archive::describe_resource( size_t nest_level,
                            std::ostream& stream,
                            const char* name,
                            const ResourceType& resource,
                            bool too_large )
{
    const auto initialized = static_cast< bool >( resource );
    const bool is_archive = std::is_base_of< Archive, ResourceType >::value;

    describe_impl( stream, name, false, initialized,
                   get_description( resource, is_archive, nest_level ).c_str( ), is_archive,
                   too_large, nest_level );
}

template < typename ResourceType >
void
Archive::describe_resource( size_t nest_level,
                            std::ostream& stream,
                            const char* name,
                            const boost::optional< ResourceType >& resource,
                            bool too_large )
{
    const auto initialized = static_cast< bool >( resource );
    const bool is_archive = std::is_base_of< Archive, ResourceType >::value;

    const ResourceType ref = initialized ? *resource         // valid ref
                                         : ResourceType( );  // ref to dummy, not used

    describe_impl( stream, name, true, initialized ? static_cast< bool >( *resource ) : false,
                   get_description( ref, is_archive, nest_level ).c_str( ), is_archive, too_large,
                   nest_level );
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
