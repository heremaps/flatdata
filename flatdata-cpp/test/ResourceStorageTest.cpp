/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include "catch.hpp"

using test_structures::AStruct;
using test_structures::TestIndexType32;

namespace flatdata
{
namespace test
{
const char* const resource_name = "resource1";
const char* const correct_schema = "foo";
const char* const incorrect_schema = "bar";

TEST_CASE( "schema is checked for multivector", "[ResourceStorage]" )
{
    std::unique_ptr< ResourceStorage > a = MemoryResourceStorage::create( );
    auto create = [&] {
        auto v
            = a->create_multi_vector< TestIndexType32, AStruct >( resource_name, correct_schema );
        v.close( );
    };
    REQUIRE_NOTHROW( create( ) );

    REQUIRE_FALSE(
        ( a->read< MultiArrayView< TestIndexType32, AStruct > >( resource_name, incorrect_schema )
              .is_initialized( ) ) );
    REQUIRE(
        ( a->read< MultiArrayView< TestIndexType32, AStruct > >( resource_name, correct_schema )
              .is_initialized( ) ) );
}

TEST_CASE( "schema is checked for external vector", "[ResourceStorage]" )
{
    std::unique_ptr< ResourceStorage > a = MemoryResourceStorage::create( );
    auto create = [&] {
        auto v = a->create_external_vector< AStruct >( resource_name, correct_schema );
        v.close( );
    };
    REQUIRE_NOTHROW( create( ) );

    REQUIRE_FALSE(
        ( a->read< ArrayView< AStruct > >( resource_name, incorrect_schema ).is_initialized( ) ) );
    REQUIRE(
        ( a->read< ArrayView< AStruct > >( resource_name, correct_schema ).is_initialized( ) ) );
}

TEST_CASE( "schema is checked for written structure", "[ResourceStorage]" )
{
    std::unique_ptr< ResourceStorage > a = MemoryResourceStorage::create( );
    auto create = [&] {
        std::vector< uint8_t > data( AStruct::size_in_bytes( ) );
        AStruct v( data.data( ) );
        a->write( resource_name, correct_schema, v );
    };
    REQUIRE_NOTHROW( create( ) );

    REQUIRE_FALSE( ( a->read< AStruct >( resource_name, incorrect_schema ).is_initialized( ) ) );
    REQUIRE( ( a->read< AStruct >( resource_name, correct_schema ).is_initialized( ) ) );
}

TEST_CASE( "schema is checked for written vector", "[ResourceStorage]" )
{
    std::unique_ptr< ResourceStorage > a = MemoryResourceStorage::create( );
    auto create = [&] {
        Vector< AStruct > v;
        a->write( resource_name, correct_schema, v );
    };
    REQUIRE_NOTHROW( create( ) );

    REQUIRE_FALSE(
        ( a->read< ArrayView< AStruct > >( resource_name, incorrect_schema ).is_initialized( ) ) );
    REQUIRE(
        ( a->read< ArrayView< AStruct > >( resource_name, correct_schema ).is_initialized( ) ) );
}

}  // namespace test
}  // namespace flatdata
