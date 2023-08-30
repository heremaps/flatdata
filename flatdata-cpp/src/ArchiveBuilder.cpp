/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/ArchiveBuilder.h>
#include <flatdata/internal/ArchiveUtils.h>

namespace flatdata
{
ArchiveBuilder::operator bool( ) const
{
    return is_open( );
}

bool
ArchiveBuilder::is_open( ) const
{
    return static_cast< bool >( m_storage );
}

ArchiveBuilder::ArchiveBuilder( std::shared_ptr< flatdata::ResourceStorage > storage )
    : m_storage( std::move( storage ) )
{
}

flatdata::ResourceStorage&
ArchiveBuilder::storage( )
{
    return *m_storage;
}

const flatdata::ResourceStorage&
ArchiveBuilder::storage( ) const
{
    return *m_storage;
}

bool
ArchiveBuilder::is_created( ) const
{
    return m_created;
}

void
ArchiveBuilder::check_created( ) const
{
    if ( !is_created( ) )
    {
        throw std::runtime_error(
            "Attempting to modify existing archive. Only subarchive creation is allowed" );
    }
}

bool
ArchiveBuilder::initialize( )
{
    const auto signature_name = internal::signature_name( name( ) );
    if ( !storage( ).exists( signature_name.c_str( ) ) )
    {
        m_created = true;
        return m_storage->write< flatdata::MemoryDescriptor >(
            signature_name.c_str( ), schema( ), flatdata::MemoryDescriptor{ "", 0 } );
    }
    auto signature
        = storage( ).read< flatdata::MemoryDescriptor >( signature_name.c_str( ), schema( ) );
    return static_cast< bool >( signature );
}

}  // namespace flatdata
