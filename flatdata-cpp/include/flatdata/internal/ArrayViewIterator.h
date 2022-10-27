/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <cstddef>
#include <iterator>

namespace flatdata
{
template < typename T >
class ArrayViewIterator
{
private:
    using ConstValueType = typename T::AccessorType;
    using ConstStreamType = typename T::AccessorType::StreamType;

public:
    using iterator_category = std::random_access_iterator_tag;
    using value_type = ConstValueType;
    using difference_type = ptrdiff_t;
    using pointer = const ConstValueType*;
    using reference = const ConstValueType&;

    class PointerWrapper
    {
    public:
        PointerWrapper( ConstStreamType ptr );
        pointer operator->( ) const;

    private:
        ConstValueType m_data;
    };

public:
    explicit ArrayViewIterator( ConstStreamType ptr = nullptr );
    ArrayViewIterator& operator++( );
    ArrayViewIterator operator++( int );
    ArrayViewIterator& operator--( );
    ArrayViewIterator operator--( int );
    ArrayViewIterator& operator+=( difference_type offset );
    ArrayViewIterator& operator-=( difference_type offset );
    ArrayViewIterator operator+( difference_type offset ) const;
    ArrayViewIterator operator-( difference_type offset ) const;
    difference_type operator-( const ArrayViewIterator& other ) const;

    bool operator==( const ArrayViewIterator& rhs ) const;
    bool operator!=( const ArrayViewIterator& rhs ) const;
    bool operator<( const ArrayViewIterator& rhs ) const;
    bool operator<=( const ArrayViewIterator& rhs ) const;
    bool operator>( const ArrayViewIterator& rhs ) const;
    bool operator>=( const ArrayViewIterator& rhs ) const;

    ConstValueType operator*( ) const;
    ConstValueType operator[]( difference_type index ) const;
    PointerWrapper operator->( ) const;

private:
    ConstStreamType m_ptr;
};

}  // namespace flatdata

#include "ArrayViewIterator.inl"
