/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "internal/ArrayViewIterator.h"

#include <cstddef>
#include <cstdlib>
#include <sstream>

namespace flatdata
{
template < typename T >
class ArrayView
{
public:
    using ConstValueType = typename T::AccessorType;
    using ConstStreamType = typename T::AccessorType::StreamType;
    using const_iterator = ArrayViewIterator< T >;

public:
    explicit ArrayView( ConstStreamType data_begin = nullptr, ConstStreamType data_end = nullptr );

    ConstValueType operator[]( size_t i ) const;
    const_iterator begin( ) const;
    const_iterator end( ) const;
    ConstStreamType data( ) const;

    ConstValueType front( ) const;
    ConstValueType back( ) const;

    ArrayView slice( size_t pos, size_t length ) const;
    ArrayView slice( std::pair< size_t /*start*/, size_t /*end*/ > range ) const;
    ArrayView slice_before( size_t pos ) const;
    ArrayView slice_after( size_t pos ) const;

    ArrayView skip( size_t count ) const;
    ArrayView skip_last( size_t count ) const;

    explicit operator bool( ) const;

    size_t size_in_bytes( ) const;
    size_t size( ) const;
    std::string describe( size_t nest_level = 0u ) const;

    bool empty( ) const;

private:
    ArrayView( ConstStreamType data, size_t size );

    ConstStreamType m_data;
    size_t m_size;
};

}  // namespace flatdata

#include "internal/ArrayView.inl"
