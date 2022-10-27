/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "ranges.hpp"
#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include "catch_amalgamated.hpp"

using namespace flatdata;
using namespace test_structures;

TEST_CASE( "Simple layout is in little endian", "[SerializedStruct]" )
{
    Struct< SimpleStruct > v;
    SimpleStructMutator x = *v;
    REQUIRE( x.size_in_bytes( ) == size_t( 8 ) );
    x.a = 0x01234567;
    x.b = 0x89abcdef;

    const uint8_t* data = x.data( );
    SimpleStruct reader{data};
    REQUIRE( uint32_t( reader.a ) == 0x01234567u );
    REQUIRE( uint32_t( reader.b ) == 0x89abcdefu );

    REQUIRE( data[ 0 ] == 0x67 );
    REQUIRE( data[ 1 ] == 0x45 );
    REQUIRE( data[ 2 ] == 0x23 );
    REQUIRE( data[ 3 ] == 0x01 );
    REQUIRE( data[ 4 ] == 0xef );
    REQUIRE( data[ 5 ] == 0xcd );
    REQUIRE( data[ 6 ] == 0xab );
    REQUIRE( data[ 7 ] == 0x89 );
}

TEST_CASE( "Bit packed works", "[SerializedStruct]" )
{
    Struct< BitPackedStruct > v;
    BitPackedStructMutator x = *v;
    REQUIRE( x.size_in_bytes( ) == size_t( 5 ) );
    x.a = true;
    x.b = 0x01234567;
    x.c = 0x48;

    const uint8_t* data = x.data( );
    BitPackedStruct reader{data};
    REQUIRE( uint32_t( reader.a ) == true );
    REQUIRE( uint32_t( reader.b ) == 0x01234567u );
    REQUIRE( uint32_t( reader.c ) == 0x48u );

    REQUIRE( data[ 0 ] == 0xCF );
    REQUIRE( data[ 1 ] == 0x8A );
    REQUIRE( data[ 2 ] == 0x46 );
    REQUIRE( data[ 3 ] == 0x02 );
    REQUIRE( data[ 4 ] == 0x90 );
}

TEST_CASE( "Signed struct works", "[SerializedStruct]" )
{
    Struct< SignedStruct > v;
    SignedStructMutator x = *v;
    REQUIRE( x.size_in_bytes( ) == size_t( 10 ) );
    x.a = -0x1;
    x.b = 0x01234567;
    x.c = -0x28;
    x.d = 0;

    const uint8_t* data = x.data( );  // 17002468ACFF
    SignedStruct reader{data};
    REQUIRE( int16_t( reader.a ) == -0x1 );
    REQUIRE( uint32_t( reader.b ) == 0x01234567u );
    REQUIRE( int32_t( reader.c ) == -0x28 );
    REQUIRE( uint32_t( reader.d ) == 0u );

    REQUIRE( data[ 0 ] == 0xFF );
    REQUIRE( data[ 1 ] == 0xAC );
    REQUIRE( data[ 2 ] == 0x68 );
    REQUIRE( data[ 3 ] == 0x24 );
    REQUIRE( data[ 4 ] == 0x00 );
    REQUIRE( data[ 5 ] == 0x0B );
    REQUIRE( data[ 6 ] == 0x00 );
    REQUIRE( data[ 7 ] == 0x00 );
    REQUIRE( data[ 8 ] == 0x00 );
    REQUIRE( data[ 9 ] == 0x00 );
}

TEST_CASE( "Struct with ranges works", "[SerializedStruct]" )
{
    Vector< n::S > vec;
    for ( size_t i = 0; i < 100; i++ )
    {
        auto item = vec.grow( );
        item.x = i;
        item.first_y = 10 * i;
    }

    ArrayView< n::S > view = vec;
    REQUIRE( view.size( ) == 99 );

    for ( size_t i = 0; i < 99; i++ )
    {
        REQUIRE( view[ i ].x == i );
        REQUIRE( view[ i ].first_y == i * 10 );
        std::pair< uint32_t, uint32_t > range = view[ i ].y_range;
        REQUIRE( range.first == ( i * 10 ) );
        REQUIRE( range.second == ( ( i + 1 ) * 10 ) );
    }
}

TEST_CASE( "Struct schemas are different", "[SerializedStruct]" )
{
    REQUIRE( std::string( SimpleStruct::schema( ).c_str( ) )
             != OtherSimpleStruct::schema( ).c_str( ) );
}
