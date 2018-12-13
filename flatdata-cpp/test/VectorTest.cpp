/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include "catch.hpp"

using namespace flatdata;
using namespace test_structures;

TEST_CASE( "Filling data in vector", "[Vector]" )
{
    Vector< AStruct > data( 2 );
    REQUIRE( data.size( ) == size_t( 2 ) );
    data.front( ).value = 10;
    data.back( ).value = 11;
    data.grow( ).value = 12;
    REQUIRE( data.size( ) == size_t( 3 ) );
    data.resize( 5 );
    REQUIRE( data.size( ) == size_t( 5 ) );
    data[ 3 ].value = 13;
    data.pop_back( );
    REQUIRE( data.size( ) == size_t( 4 ) );

    ArrayView< AStruct > view = data;
    REQUIRE( view.size( ) == 4 );
    CHECK( view[ 0 ].value == uint64_t( 10 ) );
    CHECK( view[ 1 ].value == uint64_t( 11 ) );
    CHECK( view[ 2 ].value == uint64_t( 12 ) );
    CHECK( view[ 3 ].value == uint64_t( 13 ) );
}

TEST_CASE( "Size in bytes of vector", "[Vector]" )
{
    Vector< BStruct > data( 2 );
    REQUIRE( data.size_in_bytes( ) == size_t( 2 * 2 ) );
    data.resize( 10 );
    REQUIRE( data.size_in_bytes( ) == size_t( 10 * 2 ) );
}

TEST_CASE( "Vector iterators", "[Vector]" )
{
    Vector< AStruct > data( 2 );
    data.front( ).value = 1;
    data.back( ).value = 2;
    data.grow( ).value = 3;
    data.grow( ).value = 4;

    ArrayView< AStruct > view = data;
    size_t i = 1;
    for ( auto it = view.begin( ); it != view.end( ); ++it )
    {
        REQUIRE( ( *it ).value == i );
        REQUIRE( it->value == i );
        i += 1;
    }
}

TEST_CASE( "Slowly growing data in vector", "[Vector]" )
{
    Vector< AStruct > data;
    for ( size_t i = 0; i < 256; i++ )
    {
        auto new_item = data.grow( );
        new_item.value = i;
        REQUIRE( new_item.value == i );
    }
}