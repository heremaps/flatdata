/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

namespace flatdata
{
template < typename T >
ArrayView< T >::ArrayView( ConstStreamType data_begin, ConstStreamType data_end )
    : m_data( data_begin )
    , m_size( ( data_end - data_begin ) / T::size_in_bytes( ) )
{
    if ( T::IS_OVERLAPPING_WITH_NEXT )
    {
        m_size--;
    }
}

template < typename T >
ArrayView< T >::ArrayView( ConstStreamType data, size_t size )
    : m_data( data )
    , m_size( size )
{
}

template < typename T >
typename ArrayView< T >::ConstValueType ArrayView< T >::operator[]( size_t i ) const
{
    return ConstValueType{m_data + T::size_in_bytes( ) * i};
}

template < typename T >
typename ArrayView< T >::ConstValueType
ArrayView< T >::front( ) const
{
    return ( *this )[ 0 ];
}

template < typename T >
typename ArrayView< T >::ConstValueType
ArrayView< T >::back( ) const
{
    return ( *this )[ size( ) - 1 ];
}

template < typename T >
size_t
ArrayView< T >::size_in_bytes( ) const
{
    return ( m_size + ( T::IS_OVERLAPPING_WITH_NEXT ? 1 : 0 ) ) * T::size_in_bytes( );
}

template < typename T >
typename ArrayView< T >::ConstStreamType
ArrayView< T >::data( ) const
{
    return m_data;
}

template < typename T >
size_t
ArrayView< T >::size( ) const
{
    return m_size;
}

template < typename T >
bool
ArrayView< T >::empty( ) const
{
    return m_size == 0;
}

template < typename T >
ArrayView< T >
ArrayView< T >::slice( size_t pos, size_t length ) const
{
    return ArrayView( m_data + pos * T::size_in_bytes( ), length );
}

template < typename T >
ArrayView< T >
ArrayView< T >::slice( std::pair< size_t /*start*/, size_t /*end*/ > range ) const
{
    return ArrayView( m_data + range.first * T::size_in_bytes( ), range.second - range.first );
}

template < typename T >
ArrayView< T >
ArrayView< T >::slice_before( size_t pos ) const
{
    return ArrayView( m_data, pos );
}

template < typename T >
ArrayView< T >
ArrayView< T >::slice_after( size_t pos ) const
{
    return ArrayView( m_data + pos * T::size_in_bytes( ), m_size - pos );
}

template < typename T >
ArrayView< T >
ArrayView< T >::skip( size_t count ) const
{
    return slice_after( count );
}

template < typename T >
ArrayView< T >
ArrayView< T >::skip_last( size_t count ) const
{
    return slice_before( size( ) - count );
}

template < typename T >
ArrayView< T >::operator bool( ) const
{
    return m_data != nullptr;
}

template < typename T >
typename ArrayView< T >::const_iterator
ArrayView< T >::begin( ) const
{
    return const_iterator( m_data );
}

template < typename T >
typename ArrayView< T >::const_iterator
ArrayView< T >::end( ) const
{
    return const_iterator( m_data + size( ) * T::size_in_bytes( ) );
}

template < typename T >
std::string ArrayView< T >::describe( size_t /*unused*/ ) const
{
    std::ostringstream ss;
    if ( !empty( ) )
    {
        ss << "Array of size: " << size( ) << " in " << size_in_bytes( ) << " bytes";
    }
    else
    {
        ss << "Uninitialized Array";
    }
    return ss.str( );
}

}  // namespace flatdata
