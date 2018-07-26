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
    data.close( );

    auto view = *storage->read< ArrayView< AStruct > >( "data", "foo" );
    ASSERT_EQ( size_t( 3 ), view.size( ) );
    EXPECT_EQ( uint64_t( 10 ), view[ 0 ].value );
    EXPECT_EQ( uint64_t( 11 ), view[ 1 ].value );
    EXPECT_EQ( uint64_t( 12 ), view[ 2 ].value );
}

TEST( ExternalVectorTest, Flush )
{
    auto storage = MemoryResourceStorage::create( );
    auto data = storage->create_external_vector< CStruct >( "data", "foo" );
    for ( size_t i = 0; i < 32 * 1024 * 1024; i++ )
    {
        ASSERT_EQ( i, data.size( ) );
        data.grow( ).value = i;
    }

    data.close( );

    auto view = *storage->read< ArrayView< CStruct > >( "data", "foo" );
    for ( size_t i = 0; i < 32 * 1024 * 1024; i++ )
    {
        ASSERT_EQ( i, view[ i ].value );
    }
}
