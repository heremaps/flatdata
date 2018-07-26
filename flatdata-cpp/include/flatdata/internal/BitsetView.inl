/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

namespace flatdata
{
inline BitsetView::BitsetView( ConstStreamType data_begin, ConstStreamType data_end )
    : m_data( data_begin )
    , m_begin( 0 )
    , m_end( ( data_end - data_begin ) * 8 )
{
    if ( data_end > data_begin )
    {
        // inspect last byte to find sentinel
        CharType last_byte = *( data_end - 1 );
        CharType bit = 0x80;
        while ( ( last_byte & bit ) != 0 )
        {
            bit >>= 1;
            m_end--;
        }
        m_end--;
    }
}

inline BitsetView::BitsetView( ConstStreamType data_begin, size_t begin, size_t end )
    : m_data( data_begin )
    , m_begin( begin )
    , m_end( end )
{
}

inline typename BitsetView::ConstValueType BitsetView::operator[]( size_t i ) const
{
    return ConstValueType{m_data, m_begin + i};
}

inline size_t
BitsetView::size_in_bytes( ) const
{
    return ( m_end + ( BITS_PER_CHAR - 1 ) ) / BITS_PER_CHAR - m_begin / BITS_PER_CHAR;
}

inline typename BitsetView::ConstStreamType
BitsetView::data( ) const
{
    return m_data + m_begin / BITS_PER_CHAR;
}

inline size_t
BitsetView::size( ) const
{
    return m_end - m_begin;
}

inline bool
BitsetView::empty( ) const
{
    return m_begin == m_end;
}

inline BitsetView
BitsetView::slice( size_t pos, size_t length ) const
{
    return BitsetView( m_data, m_begin + pos, m_begin + pos + length );
}

inline BitsetView
BitsetView::slice_before( size_t pos ) const
{
    return BitsetView( m_data, m_begin, m_begin + pos );
}

inline BitsetView
BitsetView::slice_after( size_t pos ) const
{
    return BitsetView( m_data, m_begin + pos, m_end );
}

inline BitsetView::operator bool( ) const
{
    return m_data != nullptr;
}

inline typename BitsetView::const_iterator
BitsetView::begin( ) const
{
    return const_iterator( m_data, m_begin );
}

inline typename BitsetView::const_iterator
BitsetView::end( ) const
{
    return const_iterator( m_data, m_end );
}

inline std::string
BitsetView::describe( ) const
{
    std::ostringstream ss;
    ss << "Bitset of size: " << size( ) << " in " << size_in_bytes( ) << " bytes";
    return ss.str( );
}

}  // namespace flatdata
