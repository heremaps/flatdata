/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "internal/Constants.h"

#include <array>
#include <type_traits>

namespace flatdata
{
/*
 * A class that owns the memory of a single flatdata structure (+neccessary padding)
 * Useful for creating individual struct that are to be stored in an archive one their own
 *     (as opposed to being stored in a vector/multi_vector)
 */
template < typename T >
class Struct
{
public:
    using ValueType = typename T::MutatorType;
    using ConstValueType = typename T::AccessorType;
    using StreamType = typename T::MutatorType::StreamType;
    using ConstStreamType = typename T::AccessorType::StreamType;

    static_assert( !T::IS_OVERLAPPING_WITH_NEXT,
                   "Cannot use range/overlapping structs standalone" );

public:
    Struct( );

    ValueType operator*( );
    ConstValueType operator*( ) const;

private:
    std::array< typename std::remove_pointer< StreamType >::type,
                T::size_in_bytes( ) + PADDING_SIZE >
        m_data;
};

// -----------------------------------------------------------------------------

template < typename T >
Struct< T >::Struct( )
{
    m_data.fill( 0 );
}

template < typename T >
typename Struct< T >::ConstValueType
Struct< T >::operator*( ) const
{
    return ConstValueType{ m_data.data( ) };
}

template < typename T >
typename Struct< T >::ValueType
Struct< T >::operator*( )
{
    return ValueType{ m_data.data( ) };
}

}  // namespace flatdata
