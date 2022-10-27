/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include "catch_amalgamated.hpp"

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

TEST_CASE( "Invalid values are handled", "[Struct]" )
{
    Struct< TestInvalidValue > data;
    TestInvalidValueMutator writer = *data;
    TestInvalidValue reader = *data;

    writer.invalid_zero = 10;
    REQUIRE( *writer.invalid_zero == 10 );
    REQUIRE( static_cast< bool >( writer.invalid_zero ) == true );
    REQUIRE( static_cast< boost::optional< int8_t > >( writer.invalid_zero )
             == boost::optional< int8_t >( 10 ) );
    REQUIRE( *reader.invalid_zero == 10 );
    REQUIRE( static_cast< bool >( reader.invalid_zero ) == true );
    REQUIRE( static_cast< boost::optional< int8_t > >( reader.invalid_zero )
             == boost::optional< int8_t >( 10 ) );

    writer.invalid_zero = 0;
    REQUIRE( *writer.invalid_zero == 0 );
    REQUIRE( static_cast< bool >( writer.invalid_zero ) == false );
    REQUIRE( static_cast< boost::optional< int8_t > >( writer.invalid_zero ) == boost::none );
    REQUIRE( *reader.invalid_zero == 0 );
    REQUIRE( static_cast< bool >( reader.invalid_zero ) == false );
    REQUIRE( static_cast< boost::optional< int8_t > >( reader.invalid_zero ) == boost::none );

    writer.invalid_min_int = 10;
    REQUIRE( *writer.invalid_min_int == 10 );
    REQUIRE( static_cast< bool >( writer.invalid_min_int ) == true );
    REQUIRE( static_cast< boost::optional< int8_t > >( writer.invalid_min_int )
             == boost::optional< int8_t >( 10 ) );
    REQUIRE( *reader.invalid_min_int == 10 );
    REQUIRE( static_cast< bool >( reader.invalid_min_int ) == true );
    REQUIRE( static_cast< boost::optional< int8_t > >( reader.invalid_min_int )
             == boost::optional< int8_t >( 10 ) );

    writer.invalid_min_int = -128;
    REQUIRE( *writer.invalid_min_int == -128 );
    REQUIRE( static_cast< bool >( writer.invalid_min_int ) == false );
    REQUIRE( static_cast< boost::optional< int8_t > >( writer.invalid_min_int ) == boost::none );
    REQUIRE( *reader.invalid_min_int == -128 );
    REQUIRE( static_cast< bool >( reader.invalid_min_int ) == false );
    REQUIRE( static_cast< boost::optional< int8_t > >( reader.invalid_min_int ) == boost::none );

    writer.invalid_max_int = 10;
    REQUIRE( *writer.invalid_max_int == 10 );
    REQUIRE( static_cast< bool >( writer.invalid_max_int ) == true );
    REQUIRE( static_cast< boost::optional< int8_t > >( writer.invalid_max_int )
             == boost::optional< int8_t >( 10 ) );
    REQUIRE( *reader.invalid_max_int == 10 );
    REQUIRE( static_cast< bool >( reader.invalid_max_int ) == true );
    REQUIRE( static_cast< boost::optional< int8_t > >( reader.invalid_max_int )
             == boost::optional< int8_t >( 10 ) );

    writer.invalid_max_int = 127;
    REQUIRE( *writer.invalid_max_int == 127 );
    REQUIRE( static_cast< bool >( writer.invalid_max_int ) == false );
    REQUIRE( static_cast< boost::optional< int8_t > >( writer.invalid_max_int ) == boost::none );
    REQUIRE( *reader.invalid_max_int == 127 );
    REQUIRE( static_cast< bool >( reader.invalid_max_int ) == false );
    REQUIRE( static_cast< boost::optional< int8_t > >( reader.invalid_max_int ) == boost::none );
}

TEST_CASE( "Invalid values can be converted to string", "[Struct]" )
{
    Struct< TestInvalidValue > data;
    constexpr auto EXPECTED = R"(TestInvalidValue {
    invalid_zero : 0,
    invalid_min_int : 0,
    invalid_max_int : 0,
})";
    REQUIRE( ( *data ).to_string( ) == EXPECTED );
}