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

template < typename T, bool is_enum >
struct IsSignedImpl
{
};

template < typename T >
struct IsSignedImpl< T, false >
{
    enum
    {
        value = std::is_signed< T >::value
    };
};

template < typename T >
struct IsSignedImpl< T, true >
{
    enum
    {
        value = std::is_signed< typename std::underlying_type< T >::type >::value
    };
};

// Since std::is_signed does not support strictly typed enums we need to add support ourselves
template < typename T >
struct IsSigned
{
    enum
    {
        value = IsSignedImpl< T, std::is_enum< T >::value >::value
    };
};

template < typename ResultType, int num_bits, typename T >
typename std::enable_if< IsSigned< ResultType >::value, ResultType >::type
extend_sign( T value )
{
    static_assert( std::is_unsigned< T >::value, "expected unsigned type" );
    struct
    {
        typename std::make_signed< T >::type x : num_bits;  // use the compiler to sign extend bits
    } temp_struct;
    temp_struct.x = value;
    return static_cast< ResultType >( temp_struct.x );
}

template < typename ResultType, int num_bits, typename T >
typename std::enable_if< !IsSigned< ResultType >::value, ResultType >::type
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
