/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <cassert>
#include <cstddef>
#include <iterator>

namespace flatdata
{
class BitProxy
{
    static constexpr size_t BITS_PER_CHAR = 8;

public:
    BitProxy( uint8_t* data, size_t pos );

    operator bool( ) const;
    BitProxy& operator=( bool value );

private:
    uint8_t* m_data;
    size_t m_pos;
};

struct ConstBitProxy
{
    static constexpr size_t BITS_PER_CHAR = 8;

public:
    ConstBitProxy( const uint8_t* data, size_t pos );

    operator bool( ) const;

private:
    const uint8_t* m_data;
    size_t m_pos;
};

class BitsetViewIterator
{
private:
    using ConstStreamType = const uint8_t*;

public:
    using iterator_category = std::random_access_iterator_tag;
    using value_type = bool;
    using difference_type = ptrdiff_t;
    using pointer = const bool*;
    using reference = const bool&;

public:
    explicit BitsetViewIterator( ConstStreamType ptr = nullptr, size_t pos = 0 );
    BitsetViewIterator& operator++( );
    BitsetViewIterator operator++( int );
    BitsetViewIterator& operator--( );
    BitsetViewIterator operator--( int );
    BitsetViewIterator& operator+=( difference_type offset );
    BitsetViewIterator& operator-=( difference_type offset );
    BitsetViewIterator operator+( difference_type offset ) const;
    BitsetViewIterator operator-( difference_type offset ) const;
    difference_type operator-( const BitsetViewIterator& other ) const;

    bool operator==( const BitsetViewIterator& rhs ) const;
    bool operator!=( const BitsetViewIterator& rhs ) const;
    bool operator<( const BitsetViewIterator& rhs ) const;
    bool operator<=( const BitsetViewIterator& rhs ) const;
    bool operator>( const BitsetViewIterator& rhs ) const;
    bool operator>=( const BitsetViewIterator& rhs ) const;

    bool operator*( ) const;
    bool operator[]( difference_type index ) const;

private:
    ConstStreamType m_ptr;
    size_t m_pos;
};

inline ConstBitProxy::ConstBitProxy( const uint8_t* data, size_t pos )
    : m_data( data )
    , m_pos( pos )
{
}

inline ConstBitProxy::operator bool( ) const
{
    return ( m_data[ m_pos / BITS_PER_CHAR ] & ( size_t( 1 ) << ( m_pos % BITS_PER_CHAR ) ) ) != 0;
}

inline BitProxy::BitProxy( uint8_t* data, size_t pos )
    : m_data( data )
    , m_pos( pos )
{
}

inline BitProxy::operator bool( ) const
{
    // re-use const version
    return ConstBitProxy( m_data, m_pos );
}

inline BitProxy&
BitProxy::operator=( bool value )
{
    size_t byte_pos = m_pos / BITS_PER_CHAR;
    size_t bit_pos = m_pos % BITS_PER_CHAR;
    unsigned mask = ( unsigned( 1 ) << bit_pos );
    if ( !value )
    {
        // delete bit
        m_data[ byte_pos ] &= ~mask;
    }
    else
    {
        // set bit
        m_data[ byte_pos ] |= mask;
    }
    return *this;
}

inline BitsetViewIterator::BitsetViewIterator( ConstStreamType ptr, size_t pos )
    : m_ptr( ptr )
    , m_pos( pos )
{
}

inline BitsetViewIterator& BitsetViewIterator::operator++( )
{
    m_pos++;
    return *this;
}

inline BitsetViewIterator BitsetViewIterator::operator++( int )
{
    auto copy = *this;
    operator++( );
    return copy;
}

inline BitsetViewIterator& BitsetViewIterator::operator--( )
{
    m_pos--;
    return *this;
}

inline BitsetViewIterator BitsetViewIterator::operator--( int )
{
    auto copy = *this;
    operator--( );
    return copy;
}

inline BitsetViewIterator&
BitsetViewIterator::operator+=( difference_type offset )
{
    m_pos += offset;
    return *this;
}

inline BitsetViewIterator&
BitsetViewIterator::operator-=( difference_type offset )
{
    m_pos -= offset;
    return *this;
}

inline BitsetViewIterator
BitsetViewIterator::operator+( difference_type offset ) const
{
    auto copy = *this;
    copy += offset;
    return copy;
}

inline BitsetViewIterator
BitsetViewIterator::operator-( difference_type offset ) const
{
    auto copy = *this;
    copy -= offset;
    return copy;
}

inline typename BitsetViewIterator::difference_type
BitsetViewIterator::operator-( const BitsetViewIterator& other ) const
{
    assert( m_ptr == other.m_ptr );  // iters must came from the same container
    return ( m_pos - other.m_pos );
}

inline bool
BitsetViewIterator::operator==( const BitsetViewIterator& other ) const
{
    assert( m_ptr == other.m_ptr );  // iters must came from the same container
    return m_pos == other.m_pos;
}

inline bool
BitsetViewIterator::operator!=( const BitsetViewIterator& other ) const
{
    assert( m_ptr == other.m_ptr );  // iters must came from the same container
    return m_pos != other.m_pos;
}

inline bool
BitsetViewIterator::operator<( const BitsetViewIterator& other ) const
{
    assert( m_ptr == other.m_ptr );  // iters must came from the same container
    return m_pos < other.m_pos;
}

inline bool
BitsetViewIterator::operator<=( const BitsetViewIterator& other ) const
{
    assert( m_ptr == other.m_ptr );  // iters must came from the same container
    return m_pos <= other.m_pos;
}

inline bool
BitsetViewIterator::operator>( const BitsetViewIterator& other ) const
{
    assert( m_ptr == other.m_ptr );  // iters must came from the same container
    return m_pos > other.m_pos;
}

inline bool
BitsetViewIterator::operator>=( const BitsetViewIterator& other ) const
{
    assert( m_ptr == other.m_ptr );  // iters must came from the same container
    return m_pos >= other.m_pos;
}

inline bool BitsetViewIterator::operator*( ) const
{
    return ConstBitProxy{m_ptr, m_pos};
}

inline bool BitsetViewIterator::operator[]( difference_type index ) const
{
    return ConstBitProxy{m_ptr, m_pos + index};
}

}  // namespace flatdata
