/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <cstddef>
#include <cstdint>
#include <type_traits>

namespace flatdata
{
template < int SizeInBytes >
struct MakeUnsignedInt
{
    using type = typename MakeUnsignedInt< SizeInBytes + 1 >::type;
};

template <>
struct MakeUnsignedInt< 1 >
{
    using type = uint8_t;
};

template <>
struct MakeUnsignedInt< 2 >
{
    using type = uint16_t;
};

template <>
struct MakeUnsignedInt< 4 >
{
    using type = uint32_t;
};

template <>
struct MakeUnsignedInt< 8 >
{
    using type = uint64_t;
};

template < int SizeInBytes >
struct BitsToUnsigned
{
    using type = typename MakeUnsignedInt< ( SizeInBytes + 7 ) / 8 >::type;
};

template < typename ResultType, int num_bits, typename T >
typename std::enable_if< std::is_signed< ResultType >::value, ResultType >::type
extend_sign( T value )
{
    static_assert( std::is_unsigned< T >::value, "expected unsigned type" );
    struct
    {
        ResultType x : num_bits;  // use the compiler to sign extend bits
    } temp_struct;
    return temp_struct.x = value;
}

template < typename ResultType, int num_bits, typename T >
typename std::enable_if< !std::is_signed< ResultType >::value, ResultType >::type
extend_sign( T value )
{
    return static_cast< ResultType >( value );
}

// keeps only the specified amount of bits in an integer, masks out the rest
template < std::size_t bits, typename T >
typename std::enable_if< bits != sizeof( T ) * 8, T >::type
masked( T value )
{
    return value & ( ( typename std::make_unsigned< T >::type( 1 ) << bits ) - 1 );
}

// specialization for full bit width, does nothing
template < std::size_t bits, typename T >
typename std::enable_if< bits == sizeof( T ) * 8, T >::type
masked( T value )
{
    return value;
}

// Depending on 'which' either returns a (false), or b (true)
template < int a, int b, bool which >
struct int_choice
{
};

template < int a, int b >
struct int_choice< a, b, true >
{
    static const int value = b;
};

template < int a, int b >
struct int_choice< a, b, false >
{
    static const int value = a;
};

}  // namespace flatdata
