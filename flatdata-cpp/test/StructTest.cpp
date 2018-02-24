/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

using namespace flatdata;
using namespace test_structures;

TEST( StructTest, filling_data )
{
    Struct< AStruct > data;
    ( *data ).value = 10;
    EXPECT_EQ( uint64_t( 10 ), ( *data ).value );
}

TEST( StructTest, copyable )
{
    Struct< AStruct > data;
    ( *data ).value = 10;
    EXPECT_EQ( uint64_t( 10 ), ( *data ).value );

    // try to copy and make sure that they are not implicitly sharing storage
    auto copy = data;
    EXPECT_EQ( uint64_t( 10 ), ( *copy ).value );
    ( *copy ).value = 11;
    EXPECT_EQ( uint64_t( 11 ), ( *copy ).value );
    EXPECT_EQ( uint64_t( 10 ), ( *data ).value );
}