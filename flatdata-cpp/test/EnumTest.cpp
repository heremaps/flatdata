/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

using namespace flatdata;
using namespace test_structures;

TEST( EnumTest, EnumerationValues )
{
    ASSERT_EQ( 0u, static_cast< unsigned >( Enum1::VALUE_1 ) );
    ASSERT_EQ( 3u, static_cast< unsigned >( Enum1::VALUE_2 ) );
    ASSERT_EQ( 4u, static_cast< unsigned >( Enum1::VALUE_3 ) );
    ASSERT_EQ( 1u, static_cast< unsigned >( Enum1::VALUE_4 ) );
    ASSERT_EQ( 2u, static_cast< unsigned >( Enum1::VALUE_5 ) );
}

TEST( EnumTest, ToString )
{
    ASSERT_STREQ( "Enum1::VALUE_1", to_string( Enum1::VALUE_1 ) );
    ASSERT_STREQ( "Enum1::VALUE_2", to_string( Enum1::VALUE_2 ) );
    ASSERT_STREQ( "Enum1::VALUE_3", to_string( Enum1::VALUE_3 ) );
    ASSERT_STREQ( "Enum1::VALUE_4", to_string( Enum1::VALUE_4 ) );
    ASSERT_STREQ( "Enum1::VALUE_5", to_string( Enum1::VALUE_5 ) );
    ASSERT_STREQ( "Unknown value of Enum1", to_string( static_cast< Enum1 >( 66 ) ) );
}

TEST( EnumTest, StructWithEnum )
{
    Vector< StructWithEnum > v( 1 );
    StructWithEnumMutator x = v[ 0 ];
    ASSERT_EQ( size_t( 6 ), x.size_in_bytes( ) );
    x.a = 0x789ab;
    x.b = Enum1::VALUE_1;
    x.c = Enum1::VALUE_3;

    const uint8_t* data = x.data( );
    StructWithEnum reader{data};
    ASSERT_EQ( 0x789ab, uint32_t( reader.a ) );
    Enum1 b = reader.b;
    ASSERT_EQ( Enum1::VALUE_1, b );
    Enum1 c = reader.c;
    ASSERT_EQ( Enum1::VALUE_3, c );
}

TEST( EnumTest, StructWithSignedEnum )
{
    // we test that reading/writing signed enums of different sizes work
    // e.g. full storage width / reduced storage width
    // min / max, -1, etc
    Vector< StructWithSignedEnum > v( 1 );
    StructWithSignedEnumMutator x = v[ 0 ];
    ASSERT_EQ( size_t( 6 ), x.size_in_bytes( ) );
    const uint8_t* data = x.data( );
    StructWithSignedEnum reader{data};

    {
        x.a = SignedEnum1::VALUE_MINUS_ONE;
        SignedEnum1 result = reader.a;
        ASSERT_EQ( SignedEnum1::VALUE_MINUS_ONE, result );
    }
    {
        x.a_less_bits = SignedEnum1::VALUE_MINUS_ONE;
        SignedEnum1 result = reader.a_less_bits;
        ASSERT_EQ( SignedEnum1::VALUE_MINUS_ONE, result );
    }

    {
        x.a = SignedEnum1::VALUE_INT16_MAX;
        SignedEnum1 result = reader.a;
        ASSERT_EQ( SignedEnum1::VALUE_INT16_MAX, result );
    }
    {
        x.a = SignedEnum1::VALUE_INT16_MIN;
        SignedEnum1 result = reader.a;
        ASSERT_EQ( SignedEnum1::VALUE_INT16_MIN, result );
    }
    {
        x.a_less_bits = SignedEnum1::VALUE_INT8_MAX;
        SignedEnum1 result = reader.a_less_bits;
        ASSERT_EQ( SignedEnum1::VALUE_INT8_MAX, result );
    }
    {
        x.a_less_bits = SignedEnum1::VALUE_INT8_MIN;
        SignedEnum1 result = reader.a_less_bits;
        ASSERT_EQ( SignedEnum1::VALUE_INT8_MIN, result );
    }
}