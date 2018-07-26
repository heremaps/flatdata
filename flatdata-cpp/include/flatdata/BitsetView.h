/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "internal/BitsetViewIterator.h"

#include <cstddef>
#include <cstdlib>
#include <sstream>

namespace flatdata
{
class BitsetView
{
public:
    using StreamType = uint8_t*;
    using ConstStreamType = const uint8_t*;
    using CharType = uint8_t;
    static constexpr size_t BITS_PER_CHAR = 8;

    using ValueType = BitProxy;
    using ConstValueType = ConstBitProxy;

    using const_iterator = BitsetViewIterator;

public:
    BitsetView( ConstStreamType data_begin, ConstStreamType data_end );
    explicit BitsetView( ConstStreamType data_begin = nullptr, size_t begin = 0, size_t end = 0 );

    ConstValueType operator[]( size_t i ) const;
    const_iterator begin( ) const;
    const_iterator end( ) const;
    ConstStreamType data( ) const;

    BitsetView slice( size_t pos, size_t length ) const;
    BitsetView slice_before( size_t pos ) const;
    BitsetView slice_after( size_t pos ) const;

    explicit operator bool( ) const;

    size_t size_in_bytes( ) const;
    size_t size( ) const;
    std::string describe( ) const;

    bool empty( ) const;

private:
    ConstStreamType m_data;
    size_t m_begin;
    size_t m_end;
};

}  // namespace flatdata

#include "internal/BitsetView.inl"
