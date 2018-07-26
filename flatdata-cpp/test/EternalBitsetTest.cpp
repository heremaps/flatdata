/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

using namespace flatdata;

TEST( ExernalBitsetTest, FillingData )
{
    auto storage = MemoryResourceStorage::create( );
    auto data = storage->create_external_bitset( "data", "foo" );
    ASSERT_EQ( size_t( 0 ), data.size( ) );
    data.grow( ) = true;
    data.grow( ) = false;
    data.grow( ) = true;
    ASSERT_EQ( size_t( 3 ), data.size( ) );
    data.close( );

    auto view = *storage->read< BitsetView >( "data", "foo" );
    ASSERT_EQ( size_t( 3 ), view.size( ) );
    EXPECT_TRUE( view[ 0 ] );
    EXPECT_FALSE( view[ 1 ] );
    EXPECT_TRUE( view[ 2 ] );
}

TEST( ExernalBitsetTest, Flush )
{
    auto storage = MemoryResourceStorage::create( );
    auto data = storage->create_external_bitset( "data", "foo" );
    for ( size_t i = 0; i < 33 * 1024 * 1024 * 8; i++ )
    {
        ASSERT_EQ( i, data.size( ) );
        data.grow( ) = ( i % 2 ) == 0;
    }

    data.close( );

    auto view = *storage->read< BitsetView >( "data", "foo" );
    for ( size_t i = 0; i < 33 * 1024 * 1024 * 8; i++ )
    {
        ASSERT_EQ( ( i % 2 ) == 0, view[ i ] );
    }
}