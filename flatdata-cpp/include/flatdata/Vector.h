/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "ArrayView.h"
#include "internal/Constants.h"

#include <type_traits>
#include <vector>

namespace flatdata
{
template < typename T >
class Vector
{
public:
    using ValueType = typename T::MutatorType;
    using ConstValueType = typename T::AccessorType;
    using StreamType = typename T::MutatorType::StreamType;
    using ConstStreamType = typename T::AccessorType::StreamType;

public:
    explicit Vector( size_t size = 0 );

    ConstValueType operator[]( size_t i ) const;
    ValueType operator[]( size_t i );

    ConstValueType front( ) const;
    ValueType front( );

    ConstValueType back( ) const;
    ValueType back( );

    size_t size_in_bytes( ) const;
    size_t size( ) const;

    ConstStreamType data( ) const;
    StreamType data( );

    void resize( size_t size );
    void reserve( size_t size );
    ValueType grow( );
    void pop_back( );

    operator ArrayView< ConstValueType >( ) const;

private:
    std::vector< typename std::remove_pointer< StreamType >::type > m_data;
};

// -----------------------------------------------------------------------------

template < typename T >
Vector< T >::Vector( size_t size )
{
    reserve( size );
    resize( size );
}

template < typename T >
typename Vector< T >::ConstValueType Vector< T >::operator[]( size_t i ) const
{
    return ConstValueType{m_data.data( ) + T::size_in_bytes( ) * i};
}

template < typename T >
typename Vector< T >::ValueType Vector< T >::operator[]( size_t i )
{
    return ValueType{m_data.data( ) + T::size_in_bytes( ) * i};
}

template < typename T >
typename Vector< T >::ConstValueType
Vector< T >::front( ) const
{
    return this->operator[]( 0 );
}

template < typename T >
typename Vector< T >::ValueType
Vector< T >::front( )
{
    return this->operator[]( 0 );
}

template < typename T >
typename Vector< T >::ConstValueType
Vector< T >::back( ) const
{
    return this->operator[]( size( ) - 1 );
}

template < typename T >
typename Vector< T >::ValueType
Vector< T >::back( )
{
    return this->operator[]( size( ) - 1 );
}

template < typename T >
size_t
Vector< T >::size_in_bytes( ) const
{
    return m_data.size( ) - PADDING_SIZE;
}

template < typename T >
typename Vector< T >::ConstStreamType
Vector< T >::data( ) const
{
    return m_data.data( );
}

template < typename T >
typename Vector< T >::StreamType
Vector< T >::data( )
{
    return m_data.data( );
}

template < typename T >
size_t
Vector< T >::size( ) const
{
    return size_in_bytes( ) / T::size_in_bytes( );
}

template < typename T >
void
Vector< T >::resize( size_t size )
{
    m_data.resize( size * T::size_in_bytes( ) + PADDING_SIZE );
}

template < typename T >
void
Vector< T >::reserve( size_t size )
{
    m_data.reserve( size * T::size_in_bytes( ) + PADDING_SIZE );
}

template < typename T >
typename Vector< T >::ValueType
Vector< T >::grow( )
{
    size_t old_size = m_data.size( );
    m_data.resize( old_size + T::size_in_bytes( ) );
    return back( );
}

template < typename T >
void
Vector< T >::pop_back( )
{
    m_data.resize( m_data.size( ) - T::size_in_bytes( ) );
}

template < typename T >
Vector< T >::operator ArrayView< typename Vector<T>::ConstValueType >( ) const
{
    return ArrayView< ConstValueType >( m_data.data( ),
                                        m_data.data( ) + m_data.size( ) - PADDING_SIZE );
}

}  // namespace flatdata
