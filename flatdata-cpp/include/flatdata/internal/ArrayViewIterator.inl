/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

namespace flatdata
{
template < typename T >
ArrayViewIterator< T >::PointerWrapper::PointerWrapper( ConstStreamType ptr )
    : m_data{ptr}
{
}

template < typename T >
typename ArrayViewIterator< T >::pointer ArrayViewIterator< T >::PointerWrapper::operator->( )
    const
{
    return &m_data;
}

template < typename T >
ArrayViewIterator< T >::ArrayViewIterator( ConstStreamType ptr )
    : m_ptr( ptr )
{
}

template < typename T >
ArrayViewIterator< T >& ArrayViewIterator< T >::operator++( )
{
    m_ptr += T::size_in_bytes( );
    return *this;
}

template < typename T >
ArrayViewIterator< T > ArrayViewIterator< T >::operator++( int )
{
    auto copy = *this;
    operator++( );
    return copy;
}

template < typename T >
ArrayViewIterator< T >& ArrayViewIterator< T >::operator--( )
{
    m_ptr -= T::size_in_bytes( );
    return *this;
}

template < typename T >
ArrayViewIterator< T > ArrayViewIterator< T >::operator--( int )
{
    auto copy = *this;
    operator--( );
    return copy;
}

template < typename T >
ArrayViewIterator< T >& ArrayViewIterator< T >::operator+=( difference_type offset )
{
    m_ptr += T::size_in_bytes( ) * offset;
    return *this;
}

template < typename T >
ArrayViewIterator< T >& ArrayViewIterator< T >::operator-=( difference_type offset )
{
    m_ptr -= T::size_in_bytes( ) * offset;
    return *this;
}

template < typename T >
ArrayViewIterator< T > ArrayViewIterator< T >::operator+( difference_type offset ) const
{
    auto copy = *this;
    copy += offset;
    return copy;
}

template < typename T >
ArrayViewIterator< T > ArrayViewIterator< T >::operator-( difference_type offset ) const
{
    auto copy = *this;
    copy -= offset;
    return copy;
}

template < typename T >
typename ArrayViewIterator< T >::difference_type
ArrayViewIterator< T >::operator-( const ArrayViewIterator& other ) const
{
    return ( m_ptr - other.m_ptr ) / T::size_in_bytes( );
}

template < typename T >
bool ArrayViewIterator< T >::operator==( const ArrayViewIterator& rhs ) const
{
    return m_ptr == rhs.m_ptr;
}

template < typename T >
bool ArrayViewIterator< T >::operator!=( const ArrayViewIterator& rhs ) const
{
    return m_ptr != rhs.m_ptr;
}

template < typename T >
bool ArrayViewIterator< T >::operator<( const ArrayViewIterator& rhs ) const
{
    return m_ptr < rhs.m_ptr;
}

template < typename T >
bool ArrayViewIterator< T >::operator<=( const ArrayViewIterator& rhs ) const
{
    return m_ptr <= rhs.m_ptr;
}

template < typename T >
bool ArrayViewIterator< T >::operator>( const ArrayViewIterator& rhs ) const
{
    return m_ptr > rhs.m_ptr;
}

template < typename T >
bool ArrayViewIterator< T >::operator>=( const ArrayViewIterator& rhs ) const
{
    return m_ptr >= rhs.m_ptr;
}

template < typename T >
typename ArrayViewIterator< T >::ConstValueType ArrayViewIterator< T >::operator*( ) const
{
    return ConstValueType{m_ptr};
}

template < typename T >
typename ArrayViewIterator< T >::ConstValueType ArrayViewIterator< T >::operator[](
    difference_type index ) const
{
    return ConstValueType{m_ptr + T::size_in_bytes( ) * index};
}

template < typename T >
typename ArrayViewIterator< T >::PointerWrapper ArrayViewIterator< T >::operator->( ) const
{
    return PointerWrapper( m_ptr );
}

} // namespace flatdata
