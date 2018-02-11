/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>

#include <gtest/gtest.h>

#include <array>

namespace flatdata
{
using namespace test_structures;
using namespace test_structures::backward_compatibility;
namespace tbi = test_structures::backward_compatibility::internal;

/**
 *             ,d
 *             88
 * ,adPPYba, MM88MMM ,adPPYba,  8b,dPPYba,
 * I8[    ""   88   a8"     "8a 88P'    "8a
 *  `"Y8ba,    88   8b       d8 88       d8
 * aa    ]8I   88,  "8a,   ,a8" 88b,   ,a8"
 * `"YbbdP"'   "Y888 `"YbbdP"'  88`YbbdP"'
 *                              88
 *                              88
 *
 * These tests freeze binary layout of flatdata resources:
 * - Instance
 * - Vector
 * - Multivector
 * - RawData
 *
 * As binary format is not part of flatdata schema, we freeze it. If format of existing resources
 * has to change, consider adding new resource (for example, `vector2` or `v2vector`). This will
 * save flatdata customers from undefined behavior in case software and archive are incompatible.
 *
 * If you have more questions, please contact flatdata maintainers, among which:
 * - Alexey Kolganov
 * - Christian Vetter
 * - Dimitri Wegner
 */

namespace
{
template < typename Array, typename Storage >
void
compare_byte_arrays( const Array& expected,
                     const flatdata::MemoryDescriptor actual,
                     const Storage& storage )
{
    EXPECT_EQ( expected.size( ) - 1, actual.size_in_bytes( ) )
        << "Sizes differ. Hexdump: " << std::endl
        << storage.hexdump( );

    for ( size_t i = 0; i < actual.size_in_bytes( ); ++i )
    {
        EXPECT_EQ( static_cast< uint8_t >( expected[ i ] ), actual.data( )[ i ] )
            << "Difference at position " << i << ". Hexdump: " << std::endl
            << storage.hexdump( );
    }
}

void
fill_signed_struct( SignedStructMutator s )
{
    EXPECT_EQ( size_t( 10 ), s.size_in_bytes( ) );
    s.a = -0x1;
    s.b = 0x01234567;
    s.c = -0x28;
    s.d = 0;
}

void
fill_simple_struct( SimpleStructMutator s )
{
    EXPECT_EQ( size_t( 8 ), s.size_in_bytes( ) );
    s.a = 0xFFFFFFFF;
    s.b = 0xDEADBEEF;
}

void
check_signed_struct( SignedStruct s )
{
    EXPECT_EQ( size_t( 10 ), s.size_in_bytes( ) );
    EXPECT_EQ( -0x1, s.a );
    EXPECT_EQ( 0x01234567u, s.b );
    EXPECT_EQ( -0x28, s.c );
    EXPECT_EQ( 0u, s.d );
}

void
check_simple_struct( SimpleStruct s )
{
    EXPECT_EQ( size_t( 8 ), s.size_in_bytes( ) );
    EXPECT_EQ( 0xFFFFFFFFu, s.a );
    EXPECT_EQ( 0xDEADBEEFu, s.b );
}

template < typename Archive >
std::shared_ptr< MemoryResourceStorage >
openable_storage( )
{
    std::shared_ptr< MemoryResourceStorage > storage = MemoryResourceStorage::create( );
    auto schema_key = std::string( Archive::name_definition( ) ) + ".archive.schema";
    auto signature_key = std::string( Archive::name_definition( ) ) + ".archive";

    storage->assign_value( schema_key.c_str( ), Archive::schema_definition( ) );
    storage->assign_value( signature_key.c_str( ), MemoryDescriptor( "\0\0\0\0\0\0\0\0"
                                                                     "\0\0\0\0\0\0\0\0",
                                                                     16 ) );
    return storage;
}

std::array< uint8_t, 27 > expected_instance_binary
    = {"\x0a\x00\x00\x00\x00\x00\x00\x00"    // Size of payload in bytes
       "\xff\xac\x68\x24\x00\x0b\x00\x00"    // Payload
       "\x00\x00"                            // Payload
       "\x00\x00\x00\x00\x00\x00\x00\x00"};  // Padding

std::array< uint8_t, 37 > expected_vector_binary
    = {"\x14\x00\x00\x00\x00\x00\x00\x00"    // Payload size in bytes
       "\xff\xac\x68\x24\x00\x0b\x00\x00"    // Payload
       "\x00\x00\xff\xac\x68\x24\x00\x0b"    // Payload
       "\x00\x00\x00\x00"                    // Payload
       "\x00\x00\x00\x00\x00\x00\x00\x00"};  // Padding

std::array< uint8_t, 66 > expected_multivector_data
    = {"\x31\x00\x00\x00\x00\x00\x00\x00"              // Payload size in bytes
       "\x01\xff\xac\x68\x24\x00\x0b\x00\x00\x00\x00"  // Payload
       "\x00\xff\xff\xff\xff\xef\xbe\xad\xde"          // Payload
       "\x00\xff\xff\xff\xff\xef\xbe\xad\xde"          // Payload
       "\x01\xff\xac\x68\x24\x00\x0b\x00\x00\x00\x00"  // Payload
       "\x00\xff\xff\xff\xff\xef\xbe\xad\xde"          // Payload
       "\x00\x00\x00\x00\x00\x00\x00\x00"};            // Padding

std::array< uint8_t, 37 > expected_multivector_index
    = {"\x14\x00\x00\x00\x00\x00\x00\x00"    // Index size in bytes
       "\x00\x00\x00\x00\x00"                // Data pointer 1
       "\x14\x00\x00\x00\x00"                // Data pointer 2
       "\x14\x00\x00\x00\x00"                // Data pointer 3
       "\x28\x00\x00\x00\x00"                // Data pointer 4
       "\x00\x00\x00\x00\x00\x00\x00\x00"};  // Padding

std::array< uint8_t, 22 > expected_raw_data_binary
    = {"\x05\x00\x00\x00\x00\x00\x00\x00"    // Payload size in bytes
       "\xff\xef\xbe\xad\xde"                // Payload
       "\x00\x00\x00\x00\x00\x00\x00\x00"};  // Padding

const std::string multivector_index_schema = std::string( "index(" )
                                             + tbi::TestMultivector__multivector_resource__schema__
                                             + std::string( ")" );
}  // namespace

TEST( BackwardCompatibilityTest, writing_instance_resources_layout )
{
    std::shared_ptr< MemoryResourceStorage > storage = MemoryResourceStorage::create( );
    auto builder = TestInstanceBuilder::open( storage );
    EXPECT_TRUE( builder.is_open( ) );

    Struct< SignedStruct > v;
    fill_signed_struct( *v );
    builder.set_instance_resource( *v );

    ASSERT_STREQ( tbi::TestInstance__instance_resource__schema__,
                  storage->read_resource( "instance_resource.schema" ).char_ptr( ) );
    compare_byte_arrays( expected_instance_binary, storage->read_resource( "instance_resource" ),
                         *storage );
}

TEST( BackwardCompatibilityTest, reading_instance_resources_layout )
{
    std::shared_ptr< MemoryResourceStorage > storage = openable_storage< TestInstance >( );
    storage->assign_value( "instance_resource",
                           MemoryDescriptor( expected_instance_binary.data( ),
                                             expected_instance_binary.size( ) - 1 ) );
    storage->assign_value( "instance_resource.schema",
                           tbi::TestInstance__instance_resource__schema__ );

    auto archive = TestInstance::open( storage );
    ASSERT_TRUE( archive.is_open( ) ) << archive.describe( );
    check_signed_struct( archive.instance_resource( ) );
}

TEST( BackwardCompatibilityTest, writing_vector_resources_layout )
{
    std::shared_ptr< MemoryResourceStorage > storage = MemoryResourceStorage::create( );
    auto builder = TestVectorBuilder::open( storage );
    EXPECT_TRUE( builder.is_open( ) );

    Vector< SignedStruct > v( 2 );
    fill_signed_struct( v[ 0 ] );
    fill_signed_struct( v[ 1 ] );
    builder.set_vector_resource( v );

    ASSERT_STREQ( tbi::TestVector__vector_resource__schema__,
                  storage->read_resource( "vector_resource.schema" ).char_ptr( ) );
    compare_byte_arrays( expected_vector_binary, storage->read_resource( "vector_resource" ),
                         *storage );
}

TEST( BackwardCompatibilityTest, reading_vector_resources_layout )
{
    std::shared_ptr< MemoryResourceStorage > storage = openable_storage< TestVector >( );
    storage->assign_value(
        "vector_resource",
        MemoryDescriptor( expected_vector_binary.data( ), expected_vector_binary.size( ) - 1 ) );
    storage->assign_value( "vector_resource.schema", tbi::TestVector__vector_resource__schema__ );

    auto archive = TestVector::open( storage );
    ASSERT_TRUE( archive.is_open( ) ) << archive.describe( );

    ASSERT_EQ( 2u, archive.vector_resource( ).size( ) );
    check_signed_struct( archive.vector_resource( )[ 0 ] );
    check_signed_struct( archive.vector_resource( )[ 1 ] );
}

TEST( BackwardCompatibilityTest, writing_multivector_resources_layout )
{
    std::shared_ptr< MemoryResourceStorage > storage = MemoryResourceStorage::create( );
    auto builder = TestMultivectorBuilder::open( storage );
    EXPECT_TRUE( builder.is_open( ) );

    auto mv = builder.start_multivector_resource( );
    fill_signed_struct( mv.add_to_current_item< SignedStruct >( ) );
    fill_simple_struct( mv.add_to_current_item< SimpleStruct >( ) );

    mv.next_item( );
    mv.next_item( );
    fill_simple_struct( mv.add_to_current_item< SimpleStruct >( ) );
    fill_signed_struct( mv.add_to_current_item< SignedStruct >( ) );

    mv.next_item( );
    fill_simple_struct( mv.add_to_current_item< SimpleStruct >( ) );

    mv.close( );

    ASSERT_STREQ( tbi::TestMultivector__multivector_resource__schema__,
                  storage->read_resource( "multivector_resource.schema" ).char_ptr( ) );
    ASSERT_STREQ( multivector_index_schema.c_str( ),
                  storage->read_resource( "multivector_resource_index.schema" ).char_ptr( ) );
    compare_byte_arrays( expected_multivector_data,
                         storage->read_resource( "multivector_resource" ), *storage );
    compare_byte_arrays( expected_multivector_index,
                         storage->read_resource( "multivector_resource_index" ), *storage );
}

TEST( BackwardCompatibilityTest, reading_multivector_resources_layout )
{
    auto storage = openable_storage< TestMultivector >( );
    storage->assign_value( "multivector_resource",
                           MemoryDescriptor( expected_multivector_data.data( ),
                                             expected_multivector_data.size( ) - 1 ) );
    storage->assign_value( "multivector_resource.schema",
                           tbi::TestMultivector__multivector_resource__schema__ );

    storage->assign_value( "multivector_resource_index",
                           MemoryDescriptor( expected_multivector_index.data( ),
                                             expected_multivector_index.size( ) - 1 ) );
    storage->assign_value( "multivector_resource_index.schema", multivector_index_schema.c_str( ) );

    auto archive = TestMultivector::open( storage );
    ASSERT_TRUE( archive.is_open( ) ) << archive.describe( );

    auto mv = archive.multivector_resource( );
    mv.for_each( 0, make_overload(
                        [&]( SimpleStruct s )
                        {
                            check_simple_struct( s );
                        },
                        [&]( SignedStruct s )
                        {
                            check_signed_struct( s );
                        } ) );

    mv.for_each( 1, make_overload(
                        [&]( SimpleStruct )
                        {
                            FAIL( );
                        },
                        [&]( SignedStruct )
                        {
                            FAIL( );
                        } ) );

    mv.for_each( 2, make_overload(
                        [&]( SimpleStruct s )
                        {
                            check_simple_struct( s );
                        },
                        [&]( SignedStruct s )
                        {
                            check_signed_struct( s );
                        } ) );

    mv.for_each( 3, make_overload(
                        [&]( SimpleStruct s )
                        {
                            check_simple_struct( s );
                        },
                        [&]( SignedStruct )
                        {
                            FAIL( );
                        } ) );
}

TEST( BackwardCompatibilityTest, writing_raw_data_resources_layout )
{
    std::shared_ptr< MemoryResourceStorage > storage = MemoryResourceStorage::create( );
    auto builder = TestRawDataBuilder::open( storage );
    EXPECT_TRUE( builder.is_open( ) );

    std::array< uint8_t, 6 > raw_data = {"\xff\xef\xbe\xad\xde"};
    builder.set_raw_data_resource(
        flatdata::MemoryDescriptor( raw_data.data( ), raw_data.size( ) - 1 ) );

    compare_byte_arrays( expected_raw_data_binary, storage->read_resource( "raw_data_resource" ),
                         *storage );
}

TEST( BackwardCompatibilityTest, reading_raw_data_resources_layout )
{
    auto storage = openable_storage< TestRawData >( );
    storage->assign_value( "raw_data_resource",
                           MemoryDescriptor( expected_raw_data_binary.data( ),
                                             expected_raw_data_binary.size( ) - 1 ) );
    storage->assign_value( "raw_data_resource.schema",
                           tbi::TestRawData__raw_data_resource__schema__ );

    auto archive = TestRawData::open( storage );
    ASSERT_TRUE( archive.is_open( ) ) << archive.describe( );

    std::array< uint8_t, 6 > expected = {"\xff\xef\xbe\xad\xde"};
    compare_byte_arrays( expected, archive.raw_data_resource( ), *storage );
}

}  // namespace flatdata
