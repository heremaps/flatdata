/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "ArrayView.h"
#include "internal/ResourceHandle.h"

#include <type_traits>
#include <vector>

#include <boost/noncopyable.hpp>
#include <boost/optional.hpp>

namespace flatdata
{
template < typename T >
class ExternalVector
{
public:
    using ValueType = typename T::MutatorType;
    using ConstValueType = typename T::AccessorType;
    using StreamType = typename T::MutatorType::StreamType;
    using ConstStreamType = typename T::AccessorType::StreamType;

public:
    explicit ExternalVector( std::unique_ptr< ResourceHandle > impl );

    size_t size( ) const;

    bool empty( ) const;

    /**
     * @brief Add new element to the array.
     * @note might flush existing elements to disk
     * @return element mutator
     */
    ValueType grow( );

    /**
     * @brief Flushes remaining elements in buffer to disk and closes the vector.
     *        After the vector is closed, no elements can be added to it anymore.
     * @return ArrayView to the written data. May fail, in this case ArrayView is
     *         invalid (cf. bool operator of ArrayView).
     */
    ArrayView< T > close( );

private:
    void flush( );

private:
    std::vector< typename std::remove_pointer< StreamType >::type > m_data;
    std::unique_ptr< ResourceHandle > m_array;
    size_t m_size = 0;
};

// -----------------------------------------------------------------------------

template < typename T >
ExternalVector< T >::ExternalVector( std::unique_ptr< ResourceHandle > impl )
    : m_array( std::move( impl ) )
{
    m_data.resize( PADDING_SIZE );
}

template < typename T >
size_t
ExternalVector< T >::size( ) const
{
    return m_size;
}

template < typename T >
bool
ExternalVector< T >::empty( ) const
{
    return m_size == 0;
}

template < typename T >
typename ExternalVector< T >::ValueType
ExternalVector< T >::grow( )
{
    if ( m_data.size( ) * sizeof( T ) > 1024 * 1024 * 32 )
    {
        flush( );
    }
    size_t old_size = m_data.size( );
    m_data.resize( old_size + ValueType::size_in_bytes( ) );
    m_size++;
    return ValueType{m_data.data( ) + old_size - PADDING_SIZE};
}

template < typename T >
void
ExternalVector< T >::flush( )
{
    m_array->write( m_data.data( ), m_data.size( ) - PADDING_SIZE );
    m_data.resize( 0 );
    m_data.resize( PADDING_SIZE );
}

template < typename T >
ArrayView< T >
ExternalVector< T >::close( )
{
    flush( );
    MemoryDescriptor data = m_array->close( );
    return data ? ArrayView< T >{data.data( ), data.data( ) + data.size_in_bytes( )}
                : ArrayView< T >{};
}

}  // namespace flatdata
