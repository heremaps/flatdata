/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"
#include "ranges.hpp"

#include <flatdata/flatdata.h>
#include "catch.hpp"

using namespace flatdata;
using namespace test_structures;

TEST_CASE( "Reading from ArrayView", "[ArrayView]" )
{
    Vector< AStruct > data( 10 );
    for ( size_t i = 0; i < 10; i++ )
    {
        data[ i ].value = i;
    }

    ArrayView< AStruct > view = data;
    for ( size_t i = 0; i < 10; i++ )
    {
        REQUIRE( view[ i ].value == i );
    }
}

TEST_CASE( "Slicing ArrayView", "[ArrayView]" )
{
    Vector< AStruct > data( 10 );
    for ( size_t i = 0; i < 10; i++ )
    {
        data[ i ].value = i;
    }

    ArrayView< AStruct > view = data;
    REQUIRE( view.size( ) == 10 );
    REQUIRE( view.slice_after( 2 ).size( ) == 8 );
    REQUIRE( view.slice_after( 2 ).front( ).value == 2 );
    REQUIRE( view.skip( 2 ).size( ) == 8 );
    REQUIRE( view.skip( 2 ).front( ).value == 2 );
    REQUIRE( view.skip_last( 2 ).size( ) == 8 );
    REQUIRE( view.skip_last( 2 ).front( ).value == 0 );
    REQUIRE( view.slice_before( 8 ).size( ) == 8 );
    REQUIRE( view.slice_before( 8 ).front( ).value == 0 );
    REQUIRE( view.slice( 2, 6 ).size( ) == 6 );
    REQUIRE( view.slice( 2, 6 ).front( ).value == 2 );
}

TEST_CASE( "Slicing ArrayView with Ranges", "[ArrayView]" )
{
    Vector< n::S > data( 11 );
    for ( size_t i = 0; i < 11; i++ )
    {
        data[ i ].x = i;
    }

    ArrayView< n::S > view = data;
    REQUIRE( view.size( ) == 10 );
    REQUIRE( view.slice_after( 2 ).size( ) == 8 );
    REQUIRE( view.slice_after( 2 ).front( ).x == 2 );
    REQUIRE( view.skip( 2 ).size( ) == 8 );
    REQUIRE( view.skip( 2 ).front( ).x == 2 );
    REQUIRE( view.skip_last( 2 ).size( ) == 8 );
    REQUIRE( view.skip_last( 2 ).front( ).x == 0 );
    REQUIRE( view.slice_before( 8 ).size( ) == 8 );
    REQUIRE( view.slice_before( 8 ).front( ).x == 0 );
    REQUIRE( view.slice( 2, 6 ).size( ) == 6 );
    REQUIRE( view.slice( 2, 6 ).front( ).x == 2 );
}
