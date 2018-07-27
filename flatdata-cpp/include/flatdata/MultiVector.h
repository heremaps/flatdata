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
 *        auto list = vector.grow( );
 *        auto a = list.add< A >( );
 *        fill_data_a( a );
 *        auto another_a = list.add< A >( );
 *        fill_data_a( another_a );
 *        auto c = list.add< B >( );
 *        fill_data_b( b );
 *        auto d = list.add< C >( );
 *        fill_data_c( c );
 *    }
 *    if ( !vector.close( ) )
 *    {
 *        throw std::runtime_error( "Failed to write to disk" );
 *    }
 * Internally data is stored like this:
 *  Index: Array< uint64_t > encodes start/end byte in Data array for each element i
 *  Data: ListBuilder of serialized (Tag,Type) tuples
 */
template < typename IndexType, typename... Args >
class MultiVector
{
public:
    /// Class used to fill an individual list
    class ListBuilder
    {
    public:
        template < typename T >
        typename T::MutatorType add( );

    private:
        friend class MultiVector;
        std::vector< unsigned char >* m_data;

        explicit ListBuilder( std::vector< unsigned char >* data );
    };

public:
    using View = MultiArrayView< IndexType, Args... >;

    MultiVector( ExternalVector< IndexType > index,
                 std::unique_ptr< ResourceHandle > data_handle,
                 size_t flush_size_bytes );

    ListBuilder grow( );

    bool close( );

private:
    void add_to_index( );

    void flush( );

private:
    std::vector< unsigned char > m_data;
    ExternalVector< IndexType > m_index;

    std::unique_ptr< ResourceHandle > m_handle;
    size_t m_size_flushed = 0;
    size_t m_flush_size_bytes = 0;
};

// -------------------------------------------------------------------------------------------------

template < typename IndexType, typename... Args >
MultiVector< IndexType, Args... >::ListBuilder::ListBuilder( std::vector< unsigned char >* data )
    : m_data( data )
{
}

template < typename IndexType, typename... Args >
template < typename T >
typename T::MutatorType
MultiVector< IndexType, Args... >::ListBuilder::add( )
{
    static_assert(
        index_of< typename T::AccessorType, typename Args::AccessorType... >::value < 256,
        "Too many types or missing type" );
    unsigned char index
        = index_of< typename T::AccessorType, typename Args::AccessorType... >::value;
    size_t old_size = m_data->size( );
    size_t increment = 1 + T::size_in_bytes( );
    m_data->resize( old_size + increment );
    ( *m_data )[ old_size - PADDING_SIZE ] = index;
    return typename T::MutatorType( m_data->data( ) + 1 + old_size - PADDING_SIZE );
}

template < typename IndexType, typename... Args >
MultiVector< IndexType, Args... >::MultiVector( ExternalVector< IndexType > index,
                                                std::unique_ptr< ResourceHandle > data_handle,
                                                size_t flush_size_bytes )
    : m_index( std::move( index ) )
    , m_handle( std::move( data_handle ) )
    , m_flush_size_bytes( flush_size_bytes )
{
    m_data.resize( PADDING_SIZE );
}

template < typename IndexType, typename... Args >
typename MultiVector< IndexType, Args... >::ListBuilder
MultiVector< IndexType, Args... >::grow( )
{
    if ( m_data.size( ) > m_flush_size_bytes )
    {
        flush( );
    }
    add_to_index( );

    return ListBuilder{&m_data};
}

template < typename IndexType, typename... Args >
bool
MultiVector< IndexType, Args... >::close( )
{
    add_to_index( );  // sentinel for last item
    flush( );
    return m_handle->close( ) && m_index.close( );
}

template < typename IndexType, typename... Args >
void
MultiVector< IndexType, Args... >::add_to_index( )
{
    m_index.grow( ).value = m_size_flushed + m_data.size( ) - PADDING_SIZE;
}

template < typename IndexType, typename... Args >
void
MultiVector< IndexType, Args... >::flush( )
{
    m_handle->write( m_data.data( ), m_data.size( ) - PADDING_SIZE );
    m_size_flushed += m_data.size( ) - PADDING_SIZE;
    m_data.resize( 0 );
    m_data.resize( PADDING_SIZE );
}

}  // namespace flatdata
