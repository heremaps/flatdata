/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "ExternalVector.h"
#include "MultiArrayView.h"
#include "internal/functional/Utility.h"

#include <cstdint>
#include <tuple>

namespace flatdata
{
/*
 * Associates a list of heterogeneous data items with an index
 * Usage:
 *    MultiVector< 32, A, C, B > vector = ...;
 *    for ( uint64_t index = 0; index < num_items; index++ )
 *    {
 *        vector.add_to_current_item< A >( A( ... ) );
 *        vector.add_to_current_item< A >( A( ... ) );
 *        vector.add_to_current_item< B >( B( ... ) );
 *        vector.add_to_current_item< C >( C( ... ) );
 *        vector.next_item( );
 *    }
 *    if ( !vector.close( ) )
 *    {
 *        throw std::runtime_error( "Failed to write to disk" );
 *    }
 * Internally data is stored like this:
 *  Index: Array< uint64_t > encodes start/end byte in Data array for each element i
 *  Data: List of serialized (Tag,Type) tuples
 */
template < typename IndexType, typename... Args >
class MultiVector
{
public:
    using View = MultiArrayView< IndexType, Args... >;

    MultiVector( ExternalVector< IndexType > index, std::unique_ptr< ResourceHandle > data_handle );

    template < typename T >
    typename T::MutatorType add_to_current_item( );
    void next_item( );
    bool close( );

private:
    void add_to_index( );

    void flush( );

private:
    std::vector< unsigned char > m_data;
    ExternalVector< IndexType > m_index;

    std::unique_ptr< ResourceHandle > m_handle;
    size_t m_size = 0;
};

// -------------------------------------------------------------------------------------------------

template < typename IndexType, typename... Args >
MultiVector< IndexType, Args... >::MultiVector( ExternalVector< IndexType > index,
                                                std::unique_ptr< ResourceHandle > data_handle )
    : m_index( std::move( index ) )
    , m_handle( std::move( data_handle ) )
{
    add_to_index( );
    m_data.resize( PADDING_SIZE );
}

template < typename IndexType, typename... Args >
template < typename T >
typename T::MutatorType
MultiVector< IndexType, Args... >::add_to_current_item( )
{
    static_assert(
        index_of< typename T::AccessorType, typename Args::AccessorType... >::value < 256,
        "Too many types or missing type" );
    unsigned char index
        = index_of< typename T::AccessorType, typename Args::AccessorType... >::value;
    size_t old_size = m_data.size( );
    size_t increment = 1 + T::size_in_bytes( );
    m_data.resize( old_size + increment );
    m_size += increment;
    m_data[ old_size - PADDING_SIZE ] = index;
    return typename T::MutatorType( m_data.data( ) + 1 + old_size - PADDING_SIZE );
}

template < typename IndexType, typename... Args >
void
MultiVector< IndexType, Args... >::next_item( )
{
    add_to_index( );
    if ( m_data.size( ) > 1024 * 1024 * 32 )
    {
        flush( );
    }
}

template < typename IndexType, typename... Args >
bool
MultiVector< IndexType, Args... >::close( )
{
    flush( );
    return m_handle->close( ) && m_index.close( );
}

template < typename IndexType, typename... Args >
void
MultiVector< IndexType, Args... >::add_to_index( )
{
    m_index.grow( ).value = m_size;
}

template < typename IndexType, typename... Args >
void
MultiVector< IndexType, Args... >::flush( )
{
    m_handle->write( m_data.data( ), m_data.size( ) - PADDING_SIZE );
    m_data.resize( 0 );
    m_data.resize( PADDING_SIZE );
}

}  // namespace flatdata
