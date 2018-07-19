/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

using namespace flatdata;
using namespace test_structures;

TEST( SerializedStructTest, SimpleLayoutIsInLittleEndian )
{
    Struct< SimpleStruct > v;
    SimpleStructMutator x = *v;
    ASSERT_EQ( size_t( 8 ), x.size_in_bytes( ) );
    x.a = 0x01234567;
    x.b = 0x89abcdef;

    const uint8_t* data = x.data( );
    SimpleStruct reader{data};
    ASSERT_EQ( 0x01234567u, uint32_t( reader.a ) );
    ASSERT_EQ( 0x89abcdefu, uint32_t( reader.b ) );

    ASSERT_EQ( 0x67, data[ 0 ] );
    ASSERT_EQ( 0x45, data[ 1 ] );
    ASSERT_EQ( 0x23, data[ 2 ] );
    ASSERT_EQ( 0x01, data[ 3 ] );
    ASSERT_EQ( 0xef, data[ 4 ] );
    ASSERT_EQ( 0xcd, data[ 5 ] );
    ASSERT_EQ( 0xab, data[ 6 ] );
    ASSERT_EQ( 0x89, data[ 7 ] );
}

TEST( SerializedStructTest, BitPackedWorks )
{
    Struct< BitPackedStruct > v;
    BitPackedStructMutator x = *v;
    ASSERT_EQ( size_t( 5 ), x.size_in_bytes( ) );
    x.a = true;
    x.b = 0x01234567;
    x.c = 0x48;

    const uint8_t* data = x.data( );
    BitPackedStruct reader{data};
    ASSERT_EQ( true, uint32_t( reader.a ) );
    ASSERT_EQ( 0x01234567u, uint32_t( reader.b ) );
    ASSERT_EQ( 0x48u, uint32_t( reader.c ) );

    ASSERT_EQ( 0xCF, data[ 0 ] );
    ASSERT_EQ( 0x8A, data[ 1 ] );
    ASSERT_EQ( 0x46, data[ 2 ] );
    ASSERT_EQ( 0x02, data[ 3 ] );
    ASSERT_EQ( 0x90, data[ 4 ] );
}

TEST( SerializedStructTest, SignedStructWorks )
{
    Struct< SignedStruct > v;
    SignedStructMutator x = *v;
    ASSERT_EQ( size_t( 10 ), x.size_in_bytes( ) );
    x.a = -0x1;
    x.b = 0x01234567;
    x.c = -0x28;
    x.d = 0;

    const uint8_t* data = x.data( );  // 17002468ACFF
    SignedStruct reader{data};
    ASSERT_EQ( -0x1, int16_t( reader.a ) );
    ASSERT_EQ( 0x01234567u, uint32_t( reader.b ) );
    ASSERT_EQ( -0x28, int32_t( reader.c ) );
    ASSERT_EQ( 0u, uint32_t( reader.d ) );

    ASSERT_EQ( 0xFF, data[ 0 ] );
    ASSERT_EQ( 0xAC, data[ 1 ] );
    ASSERT_EQ( 0x68, data[ 2 ] );
    ASSERT_EQ( 0x24, data[ 3 ] );
    ASSERT_EQ( 0x00, data[ 4 ] );
    ASSERT_EQ( 0x0B, data[ 5 ] );
    ASSERT_EQ( 0x00, data[ 6 ] );
    ASSERT_EQ( 0x00, data[ 7 ] );
    ASSERT_EQ( 0x00, data[ 8 ] );
    ASSERT_EQ( 0x00, data[ 9 ] );
}

TEST( SerializedStructTest, Schema )
{
    ASSERT_STRNE( SimpleStruct::schema( ).c_str( ), OtherSimpleStruct::schema( ).c_str( ) );
}
