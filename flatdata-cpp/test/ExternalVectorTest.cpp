/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

using namespace flatdata;
using namespace test_structures;

TEST( ExternalVectorTest, FillingData )
{
    auto storage = MemoryResourceStorage::create( );
    auto data = storage->create_external_vector< AStruct >( "data", "foo" );
    ASSERT_EQ( size_t( 0 ), data.size( ) );
    data.grow( ).value = 10;
    data.grow( ).value = 11;
    data.grow( ).value = 12;
    ASSERT_EQ( size_t( 3 ), data.size( ) );

    boost::optional< ArrayView< AStruct > > view_from_close = data.close( );
    ASSERT_TRUE( view_from_close );
    ASSERT_EQ( size_t( 3 ), view_from_close->size( ) );
    EXPECT_EQ( uint64_t( 10 ), ( *view_from_close )[ 0 ].value );
    EXPECT_EQ( uint64_t( 11 ), ( *view_from_close )[ 1 ].value );
    EXPECT_EQ( uint64_t( 12 ), ( *view_from_close )[ 2 ].value );

    auto view_from_storage = storage->read< ArrayView< AStruct > >( "data", "foo" );
    ASSERT_TRUE( view_from_storage );
    ASSERT_EQ( size_t( 3 ), view_from_storage->size( ) );
    EXPECT_EQ( uint64_t( 10 ), ( *view_from_storage )[ 0 ].value );
    EXPECT_EQ( uint64_t( 11 ), ( *view_from_storage )[ 1 ].value );
    EXPECT_EQ( uint64_t( 12 ), ( *view_from_storage )[ 2 ].value );
}
