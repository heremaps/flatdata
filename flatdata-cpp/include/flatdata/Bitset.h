/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "BitsetView.h"

#include <type_traits>
#include <vector>

namespace flatdata
{
class Bitset
{
public:
    using StreamType = uint8_t*;
    using ConstStreamType = const uint8_t*;
    using CharType = uint8_t;
    static constexpr size_t BITS_PER_CHAR = 8;

    using ValueType = BitProxy;
    using ConstValueType = ConstBitProxy;

public:
    explicit Bitset( size_t size = 0 );

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

    operator BitsetView( ) const;

private:
    std::vector< CharType > m_data;
    size_t m_size;
};

// -----------------------------------------------------------------------------

inline Bitset::Bitset( size_t size )
{
    reserve( size );
    resize( size );
}

inline typename Bitset::ConstValueType Bitset::operator[]( size_t i ) const
{
    return ConstValueType{m_data.data( ), i};
}

inline typename Bitset::ValueType Bitset::operator[]( size_t i )
{
    return ValueType{m_data.data( ), i};
}

inline typename Bitset::ConstValueType
Bitset::front( ) const
{
    return this->operator[]( 0 );
}

inline typename Bitset::ValueType
Bitset::front( )
{
    return this->operator[]( 0 );
}

inline typename Bitset::ConstValueType
Bitset::back( ) const
{
    return this->operator[]( size( ) - 1 );
}

inline typename Bitset::ValueType
Bitset::back( )
{
    return this->operator[]( size( ) - 1 );
}

inline size_t
Bitset::size_in_bytes( ) const
{
    return m_data.size( );
}

inline typename Bitset::ConstStreamType
Bitset::data( ) const
{
    return m_data.data( );
}

inline typename Bitset::StreamType
Bitset::data( )
{
    return m_data.data( );
}

inline size_t
Bitset::size( ) const
{
    return m_size;
}

inline void
Bitset::resize( size_t size )
{
    m_data.resize( ( size + ( BITS_PER_CHAR - 1 ) ) / BITS_PER_CHAR );
    m_size = size;
}

inline void
Bitset::reserve( size_t size )
{
    m_data.reserve( ( size + ( BITS_PER_CHAR - 1 ) ) / BITS_PER_CHAR );
}

inline typename Bitset::ValueType
Bitset::grow( )
{
    resize( m_size + 1 );
    return back( );
}

inline void
Bitset::pop_back( )
{
    resize( m_size - 1 );
}

inline Bitset::operator BitsetView( ) const
{
    return BitsetView( m_data.data( ), 0, m_size );
}

}  // namespace flatdata
