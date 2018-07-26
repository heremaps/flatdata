/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

using namespace flatdata;

TEST( BitsetTest, size_in_bytes )
{
    Bitset data( 8 );
    ASSERT_EQ( size_t( 1 ), data.size_in_bytes( ) );
    data.resize( 9 );
    ASSERT_EQ( size_t( 2 ), data.size_in_bytes( ) );
}

TEST( BitsetTest, filling_data )
{
    Bitset data( 2 );
    data.front( ) = true;
    data.back( ) = false;
    data.grow( ) = true;
    data.grow( ) = false;

    ASSERT_TRUE( data[ 0 ] );
    ASSERT_FALSE( data[ 1 ] );
    ASSERT_TRUE( data[ 2 ] );
    ASSERT_FALSE( data[ 3 ] );
}

TEST( BitsetTest, slowly_growing_data )
{
    Bitset data;
    for ( size_t i = 0; i < 1024; i++ )
    {
        auto new_item = data.grow( );
        new_item = ( i % 2 ) == 0;
        ASSERT_EQ( ( i % 2 ) == 0, new_item );
    }
}

TEST( BitsetTest, exhaustive )
{
    Bitset data;

    auto test_one_bit = [&]( bool previous_value, bool new_value, bool other_values, size_t pos ) {
        data.resize( 32 );
        for ( size_t i = 0; i < 32; i++ )
        {
            data[ i ] = other_values;
        }

        for ( size_t i = 0; i < 32; i++ )
        {
            ASSERT_EQ( other_values, data[ i ] );
        }

        data[ pos ] = previous_value;
        ASSERT_EQ( previous_value, data[ pos ] );
        data[ pos ] = new_value;
        ASSERT_EQ( new_value, data[ pos ] );

        for ( size_t i = 0; i < 32; i++ )
        {
            if ( i != pos )
            {
                ASSERT_EQ( other_values, data[ i ] );
            }
        }
    };

    for ( size_t previous_value = 0; previous_value < 2; previous_value++ )
    {
        for ( size_t new_value = 0; new_value < 2; new_value++ )
        {
            for ( size_t other_values = 0; other_values < 2; other_values++ )
            {
                for ( size_t pos = 0; pos < 32; pos++ )
                {
                    test_one_bit( previous_value != 0, new_value != 0, other_values != 0, pos );
                }
            }
        }
    }
}

TEST( BitsetTest, iterators )
{
    Bitset data( 2 );
    data.front( ) = true;
    data.back( ) = false;
    data.grow( ) = true;
    data.grow( ) = false;

    BitsetView view = data.finalize( );
    bool value = true;
    size_t i = 0;
    ASSERT_EQ( 4u, view.size( ) );
    for ( auto it = view.begin( ); it != view.end( ); ++it )
    {
        ASSERT_EQ( value, *it ) << "value " << i;
        value = !value;
        i++;
    }
}