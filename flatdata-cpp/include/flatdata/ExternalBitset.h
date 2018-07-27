/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "BitsetView.h"
#include "internal/ResourceHandle.h"

#include <type_traits>
#include <vector>

namespace flatdata
{
class ExternalBitset
{
public:
    using StreamType = uint8_t*;
    using ConstStreamType = const uint8_t*;
    using CharType = uint8_t;
    static constexpr size_t BITS_PER_CHAR = 8;

    using ValueType = BitProxy;
    using ConstValueType = ConstBitProxy;

public:
    ExternalBitset( std::unique_ptr< ResourceHandle > impl, size_t flush_size_bytes );

    size_t size_in_bytes( ) const;
    size_t size( ) const;
    bool empty( ) const;

    ValueType grow( );

    bool close( );

private:
    void flush( );

private:
    std::vector< CharType > m_data;
    std::unique_ptr< ResourceHandle > m_array;
    size_t m_size = 0;
    size_t m_pos = 0;
    size_t m_flush_size_bytes;
};

// -----------------------------------------------------------------------------

inline void
ExternalBitset::flush( )
{
    m_array->write( m_data.data( ), m_data.size( ) );
    m_data.resize( 0 );
    m_pos = 0;
}

inline bool
ExternalBitset::close( )
{
    // add trailing 0 and fill up remaining bits with 1
    grow( ) = false;
    while ( ( m_size % BITS_PER_CHAR ) != 0 )
    {
        grow( ) = true;
    }
    flush( );
    return m_array->close( );
}

inline ExternalBitset::ExternalBitset( std::unique_ptr< ResourceHandle > impl,
                                       size_t flush_size_bytes )
    : m_array( std::move( impl ) )
    , m_flush_size_bytes( flush_size_bytes )
{
}

inline size_t
ExternalBitset::size_in_bytes( ) const
{
    return m_data.size( );
}

inline size_t
ExternalBitset::size( ) const
{
    return m_size;
}

inline bool
ExternalBitset::empty( ) const
{
    return m_size == 0;
}

inline typename ExternalBitset::ValueType
ExternalBitset::grow( )
{
    if ( m_pos + 1 > m_flush_size_bytes * 8 )
    {
        flush( );
    }
    m_data.resize( ( m_pos + 1 + ( BITS_PER_CHAR - 1 ) ) / BITS_PER_CHAR );
    m_size++;
    m_pos++;
    return ValueType{m_data.data( ), m_pos - 1};
}

}  // namespace flatdata
