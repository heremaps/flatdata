/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

using namespace flatdata;
using namespace test_structures;

TEST( VectorTest, filling_data )
{
    Vector< AStruct > data( 2 );
    ASSERT_EQ( size_t( 2 ), data.size( ) );
    data.front( ).value = 10;
    data.back( ).value = 11;
    data.grow( ).value = 12;
    ASSERT_EQ( size_t( 3 ), data.size( ) );
    data.resize( 5 );
    ASSERT_EQ( size_t( 5 ), data.size( ) );
    data[ 3 ].value = 13;
    data.pop_back( );
    ASSERT_EQ( size_t( 4 ), data.size( ) );

    ArrayView< AStruct > view = data;
    ASSERT_TRUE( view.size( ) == 4 );
    EXPECT_EQ( uint64_t( 10 ), view[ 0 ].value );
    EXPECT_EQ( uint64_t( 11 ), view[ 1 ].value );
    EXPECT_EQ( uint64_t( 12 ), view[ 2 ].value );
    EXPECT_EQ( uint64_t( 13 ), view[ 3 ].value );
}

TEST( VectorTest, size_in_bytes )
{
    Vector< BStruct > data( 2 );
    ASSERT_EQ( size_t( 2 * 2 ), data.size_in_bytes( ) );
    data.resize( 10 );
    ASSERT_EQ( size_t( 10 * 2 ), data.size_in_bytes( ) );
}

TEST( VectorTest, iterators )
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
        ASSERT_EQ( ( *it ).value, i );
        ASSERT_EQ( it->value, i );
        i += 1;
    }
}

TEST( VectorTest, slowly_growing_data )
{
    Vector< AStruct > data;
    for ( size_t i = 0; i < 256; i++ )
    {
        auto new_item = data.grow( );
        new_item.value = i;
        ASSERT_EQ( i, new_item.value );
    }
}