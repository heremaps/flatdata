/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

using namespace flatdata;
using namespace test_structures;

TEST( VectorTest, slicing )
{
    Vector< AStruct > data( 10 );
    for ( size_t i = 0; i < 10; i++ )
    {
        data[ i ].value = i;
    }

    ArrayView< AStruct > view = data;
    ASSERT_TRUE( view.size( ) == 10 );
    ASSERT_EQ( 8, view.slice_after( 2 ).size( ) );
    ASSERT_EQ( 2, view.slice_after( 2 ).front( ).value );
    ASSERT_EQ( 8, view.skip( 2 ).size( ) );
    ASSERT_EQ( 2, view.skip( 2 ).front( ).value );
    ASSERT_EQ( 8, view.skip_last( 2 ).size( ) );
    ASSERT_EQ( 0, view.skip_last( 2 ).front( ).value );
    ASSERT_EQ( 8, view.slice_before( 8 ).size( ) );
    ASSERT_EQ( 0, view.slice_before( 8 ).front( ).value );
    ASSERT_EQ( 6, view.slice( 2, 6 ).size( ) );
    ASSERT_EQ( 2, view.slice( 2, 6 ).front( ).value );
}