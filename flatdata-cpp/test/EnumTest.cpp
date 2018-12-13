/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include "catch.hpp"

using namespace flatdata;
using namespace test_structures;

TEST_CASE( "EnumerationValues", "[Enum]" )
{
    REQUIRE( static_cast< unsigned >( Enum1::VALUE_1 ) == 0u );
    REQUIRE( static_cast< unsigned >( Enum1::VALUE_2 ) == 3u );
    REQUIRE( static_cast< unsigned >( Enum1::VALUE_3 ) == 4u );
    REQUIRE( static_cast< unsigned >( Enum1::VALUE_4 ) == 1u );
    REQUIRE( static_cast< unsigned >( Enum1::VALUE_5 ) == 2u );
}

TEST_CASE( "Enum to string", "[Enum]" )
{
    REQUIRE( to_string( Enum1::VALUE_1 ) == std::string( "Enum1::VALUE_1" ) );
    REQUIRE( to_string( Enum1::VALUE_2 ) == std::string( "Enum1::VALUE_2" ) );
    REQUIRE( to_string( Enum1::VALUE_3 ) == std::string( "Enum1::VALUE_3" ) );
    REQUIRE( to_string( Enum1::VALUE_4 ) == std::string( "Enum1::VALUE_4" ) );
    REQUIRE( to_string( Enum1::VALUE_5 ) == std::string( "Enum1::VALUE_5" ) );
    REQUIRE( to_string( static_cast< Enum1 >( 66 ) ) == std::string( "Unknown value of Enum1" ) );
}

TEST_CASE( "Struct with enum", "[Enum]" )
{
    Vector< StructWithEnum > v( 1 );
    StructWithEnumMutator x = v[ 0 ];
    REQUIRE( x.size_in_bytes( ) == size_t( 6 ) );
    x.a = 0x789ab;
    x.b = Enum1::VALUE_1;
    x.c = Enum1::VALUE_3;

    const uint8_t* data = x.data( );
    StructWithEnum reader{data};
    REQUIRE( uint32_t( reader.a ) == 0x789abu );
    Enum1 b = reader.b;
    REQUIRE( b == Enum1::VALUE_1 );
    Enum1 c = reader.c;
    REQUIRE( c == Enum1::VALUE_3 );
}

TEST_CASE( "Struct with signed enum", "[Enum]" )
{
    // we test that reading/writing signed enums of different sizes work
    // e.g. full storage width / reduced storage width
    // min / max, -1, etc
    Vector< StructWithSignedEnum > v( 1 );
    StructWithSignedEnumMutator x = v[ 0 ];
    REQUIRE( x.size_in_bytes( ) == size_t( 6 ) );
    const uint8_t* data = x.data( );
    StructWithSignedEnum reader{data};

    {
        x.a = SignedEnum1::VALUE_MINUS_ONE;
        SignedEnum1 result = reader.a;
        REQUIRE( result == SignedEnum1::VALUE_MINUS_ONE );
    }
    {
        x.a_less_bits = SignedEnum1::VALUE_MINUS_ONE;
        SignedEnum1 result = reader.a_less_bits;
        REQUIRE( result == SignedEnum1::VALUE_MINUS_ONE );
    }
    {
        x.a = SignedEnum1::VALUE_INT16_MAX;
        SignedEnum1 result = reader.a;
        REQUIRE( result == SignedEnum1::VALUE_INT16_MAX );
    }
    {
        x.a = SignedEnum1::VALUE_INT16_MIN;
        SignedEnum1 result = reader.a;
        REQUIRE( result == SignedEnum1::VALUE_INT16_MIN );
    }
    {
        x.a_less_bits = SignedEnum1::VALUE_INT8_MAX;
        SignedEnum1 result = reader.a_less_bits;
        REQUIRE( result == SignedEnum1::VALUE_INT8_MAX );
    }
    {
        x.a_less_bits = SignedEnum1::VALUE_INT8_MIN;
        SignedEnum1 result = reader.a_less_bits;
        REQUIRE( result == SignedEnum1::VALUE_INT8_MIN );
    }
}