/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include "catch.hpp"

using namespace flatdata;
using namespace test_structures;

TEST_CASE( "Filling data in struct", "[Struct]" )
{
    Struct< AStruct > data;
    ( *data ).value = 10;
    REQUIRE( ( *data ).value == uint64_t( 10 ) );
}

TEST_CASE( "Struct is copyable", "[Struct]" )
{
    Struct< AStruct > data;
    ( *data ).value = 10;
    REQUIRE( ( *data ).value == uint64_t( 10 ) );

    // try to copy and make sure that they are not implicitly sharing storage
    auto copy = data;
    REQUIRE( ( *copy ).value == uint64_t( 10 ) );
    ( *copy ).value = 11;
    REQUIRE( ( *copy ).value == uint64_t( 11 ) );
    REQUIRE( ( *data ).value == uint64_t( 10 ) );
}