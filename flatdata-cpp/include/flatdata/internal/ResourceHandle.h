/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "ResourceStorageCommon.h"

#include <boost/noncopyable.hpp>

#include <exception>
#include <fstream>
#include <memory>

namespace flatdata
{
class ResourceHandle : private boost::noncopyable
{
public:
    ~ResourceHandle( ) noexcept( false );
    template < typename T >
    void write( T* data, size_t size_in_bytes );
    bool close( );

    static std::unique_ptr< ResourceHandle > create( std::shared_ptr< std::ostream > stream );

private:
    std::shared_ptr< std::ostream > m_stream;
    resource_storage::size_type m_size_in_bytes = 0;
};

// -------------------------------------------------------------------------------------------------

inline ResourceHandle::~ResourceHandle( ) noexcept( false )
{
    if ( m_stream != nullptr )
    {
        throw std::logic_error(
            "Resource not closed before destruction. Close and check for errors." );
    }
}

template < typename T >
void
ResourceHandle::write( T* data, size_t size_in_bytes )
{
    m_stream->write( reinterpret_cast< const char* >( data ), size_in_bytes );
    m_size_in_bytes += size_in_bytes;
}

inline std::unique_ptr< ResourceHandle >
ResourceHandle::create( std::shared_ptr< std::ostream > stream )
{
    std::unique_ptr< ResourceHandle > result( new ResourceHandle( ) );
    result->m_stream = std::move( stream );
    if ( !result->m_stream )
    {
        return std::unique_ptr< ResourceHandle >( );
    }

    resource_storage::write_to_stream< resource_storage::size_type >( *result->m_stream, 0 );
    return result;
}

inline bool
ResourceHandle::close( )
{
    if ( m_stream == nullptr )
    {
        return false;
    }
    resource_storage::write_padding( *m_stream );

    // update the size in the beginning of the file
    m_stream->seekp( 0 );
    resource_storage::write_to_stream( *m_stream, m_size_in_bytes );
    m_stream->flush( );
    bool success = static_cast< bool >( *m_stream );
    m_stream.reset( );
    return success;
}

}  // namespace flatdata
