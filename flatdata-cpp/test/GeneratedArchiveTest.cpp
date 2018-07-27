/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/FileResourceStorage.h>
#include <flatdata/MemoryResourceStorage.h>
#include <gtest/gtest.h>
#include <boost/filesystem.hpp>
#include <type_traits>

using test_structures::AStruct;
using test_structures::BStruct;
using test_structures::OnlyOptional;
using test_structures::OnlyOptionalBuilder;
using test_structures::OuterArchive;
using test_structures::OuterArchiveBuilder;
using test_structures::OuterWithOptional;
using test_structures::OuterWithOptionalBuilder;
using test_structures::OutermostArchive;
using test_structures::OutermostArchiveBuilder;
using test_structures::SimpleResources;
using test_structures::SimpleResourcesBuilder;

namespace flatdata
{
namespace test
{
class GeneratedArchiveTestWithStorage : public ::testing::TestWithParam< bool >
{
public:
    void
    SetUp( )
    {
        if ( GetParam( ) )
        {
            storage = MemoryResourceStorage::create( );
            return;
        }

        temporary_path
            = boost::filesystem::temp_directory_path( ) / boost::filesystem::unique_path( );
        boost::system::error_code ec;
        boost::filesystem::create_directory( temporary_path, ec );
        if ( ec )
        {
            FAIL( ) << "Cannot create temporary directory to test FileResourceStorage at "
                    << temporary_path;
        }
        storage = FileResourceStorage::create( temporary_path.c_str( ) );
    }

    void
    TearDown( )
    {
        if ( GetParam( ) )
        {
            return;
        }
        boost::system::error_code ec;
        boost::filesystem::remove_all( temporary_path, ec );
        if ( ec )
        {
            FAIL( ) << "Failed to remove temporary directory " << temporary_path;
        }
        temporary_path.clear( );
    }

    std::shared_ptr< ResourceStorage > storage;
    boost::filesystem::path temporary_path;
};

TEST_P( GeneratedArchiveTestWithStorage, objects_can_be_read_and_written )
{
    auto builder = SimpleResourcesBuilder::open( storage );

    EXPECT_TRUE( builder.is_open( ) );
    std::vector< uint8_t > data( AStruct::size_in_bytes( ) );
    AStruct::MutatorType object( data.data( ) );
    object.value = 7;
    builder.set_object_resource( object );

    auto archive = SimpleResources::open( storage );
    EXPECT_FALSE( archive.is_open( ) );
    ASSERT_EQ( 7u, archive.object_resource( ).value );
}

TEST_P( GeneratedArchiveTestWithStorage, vectors_can_be_read_and_written )
{
    auto builder = SimpleResourcesBuilder::open( storage );

    EXPECT_TRUE( builder.is_open( ) );
    flatdata::Vector< AStruct > v;
    v.grow( ).value = 17;
    v.grow( ).value = 42;
    builder.set_vector_resource( v );

    auto archive = SimpleResources::open( storage );
    EXPECT_FALSE( archive.is_open( ) );
    ASSERT_EQ( 2u, archive.vector_resource( ).size( ) );
    ASSERT_EQ( 17u, archive.vector_resource( )[ 0 ].value );
    ASSERT_EQ( 42u, archive.vector_resource( )[ 1 ].value );
}

TEST_P( GeneratedArchiveTestWithStorage, vectors_can_be_created_incrementally )
{
    auto builder = SimpleResourcesBuilder::open( storage );

    EXPECT_TRUE( builder.is_open( ) );
    auto resource = builder.start_vector_resource( );
    resource.grow( ).value = 17;
    resource.grow( ).value = 42;
    resource.close( );

    auto archive = SimpleResources::open( storage );
    EXPECT_FALSE( archive.is_open( ) );
    ASSERT_EQ( 2u, archive.vector_resource( ).size( ) );
    ASSERT_EQ( 17u, archive.vector_resource( )[ 0 ].value );
    ASSERT_EQ( 42u, archive.vector_resource( )[ 1 ].value );
}

TEST_P( GeneratedArchiveTestWithStorage, multivectors_can_be_created_incrementally )
{
    auto builder = SimpleResourcesBuilder::open( storage );

    EXPECT_TRUE( builder.is_open( ) );
    auto resource = builder.start_multivector_resource( );
    auto list = resource.grow( );
    list.add< AStruct >( ).value = 17;
    list = resource.grow( );
    list.add< AStruct >( ).value = 42;
    resource.close( );

    auto archive = SimpleResources::open( storage );
    EXPECT_FALSE( archive.is_open( ) );
    ASSERT_EQ( 2u, archive.multivector_resource( ).size( ) );

    archive.multivector_resource( ).for_each< AStruct >(
        0, [&]( const AStruct& s ) { EXPECT_EQ( 17u, s.value ); } );
    archive.multivector_resource( ).for_each< AStruct >(
        1, [&]( const AStruct& s ) { EXPECT_EQ( 42u, s.value ); } );
}

TEST_P( GeneratedArchiveTestWithStorage, raw_data_can_be_read_and_written )
{
    auto builder = SimpleResourcesBuilder::open( storage );

    EXPECT_TRUE( builder.is_open( ) );
    builder.set_raw_data_resource( MemoryDescriptor( "abc\0", 4 ) );

    auto archive = SimpleResources::open( storage );
    EXPECT_FALSE( archive.is_open( ) );
    ASSERT_STREQ( "abc", archive.raw_data_resource( ).char_ptr( ) );
    ASSERT_EQ( 4u, archive.raw_data_resource( ).size_in_bytes( ) );
}

TEST_P( GeneratedArchiveTestWithStorage, bitset_can_be_read_and_written )
{
    auto builder = SimpleResourcesBuilder::open( storage );

    EXPECT_TRUE( builder.is_open( ) );
    flatdata::Bitset abitset( 65 );
    abitset.grow( ) = true;
    builder.set_bits( abitset.finalize( ) );

    auto archive = SimpleResources::open( storage );
    EXPECT_FALSE( archive.is_open( ) );
    auto view = archive.bits( );
    for ( size_t i = 0; i < 65; i++ )
    {
        ASSERT_FALSE( view[ i ] ) << "value " << i;
    }
    ASSERT_TRUE( view[ 65 ] );
}

TEST_P( GeneratedArchiveTestWithStorage, bitset_can_be_created_incrementally )
{
    auto builder = SimpleResourcesBuilder::open( storage );

    EXPECT_TRUE( builder.is_open( ) );
    auto abitset = builder.start_bits( );
    for ( size_t i = 0; i < 65; i++ )
    {
        abitset.grow( ) = false;
    }
    abitset.grow( ) = true;
    abitset.close( );

    auto archive = SimpleResources::open( storage );
    EXPECT_FALSE( archive.is_open( ) );
    auto view = archive.bits( );
    for ( size_t i = 0; i < 65; i++ )
    {
        ASSERT_FALSE( view[ i ] ) << "value " << i;
    }
    ASSERT_TRUE( view[ 65 ] );
}

TEST_P( GeneratedArchiveTestWithStorage, optional_resource_is_correct_when_available )
{
    auto builder = OnlyOptionalBuilder::open( storage );

    EXPECT_TRUE( builder.is_open( ) );
    builder.set_optional_resource( MemoryDescriptor( "abc\0", 4 ) );

    auto archive = OnlyOptional::open( storage );
    EXPECT_TRUE( archive.is_open( ) );
    ASSERT_TRUE( archive.optional_resource( ).is_initialized( ) );
    ASSERT_STREQ( "abc", archive.optional_resource( )->char_ptr( ) );
    ASSERT_EQ( 4u, archive.optional_resource( )->size_in_bytes( ) );
}

TEST_P( GeneratedArchiveTestWithStorage, optional_resource_is_uninitialized_when_not_available )
{
    auto builder = OnlyOptionalBuilder::open( storage );
    EXPECT_TRUE( builder.is_open( ) );

    auto archive = OnlyOptional::open( storage );
    EXPECT_TRUE( archive.is_open( ) );
    ASSERT_TRUE( !archive.optional_resource( ).is_initialized( ) );
}

TEST_P( GeneratedArchiveTestWithStorage, describe_outputs_resources_as_expected )
{
    {
        auto builder = SimpleResourcesBuilder::open( storage );
        EXPECT_TRUE( builder.is_open( ) );
        flatdata::Struct< AStruct > astruct;
        builder.set_object_resource( *astruct );
        flatdata::Vector< AStruct > avector( 11 );
        flatdata::Bitset abitset( 42 );
        builder.set_vector_resource( avector );
        builder.set_optional_resource( flatdata::MemoryDescriptor( "opt", 3 ) );
        builder.set_raw_data_resource( flatdata::MemoryDescriptor( "raw_data", 8 ) );
        builder.set_bits( abitset.finalize( ) );
        std::cout << "abitset : " << abitset.size( ) << " " << abitset.size_in_bytes( ) << " "
                  << std::endl;

        auto mv = builder.start_multivector_resource( );
        auto list = mv.grow( );
        list.add< AStruct >( ).value = 17;
        mv.close( );
    }

    auto archive = SimpleResources::open( storage );
    EXPECT_TRUE( archive.is_open( ) );
    const char* const expected =
        R"data(================================================================================
Flatdata Archive: SimpleResources
================================================================================

Resource                             Optional  Loaded    Details
================================================================================
object_resource                      NO        YES       Structure of size 1
vector_resource                      NO        YES       Array of size: 11 in 11 bytes
multivector_resource                 NO        YES       MultiArray of size 1, with index: Array of size: 2 in 10 bytes
raw_data_resource                    NO        YES       Raw data of size 8
optional_resource                    YES       YES       Raw data of size 3
bits                                 NO        YES       Bitset of size: 42 in 6 bytes
================================================================================
)data";
    ASSERT_EQ( expected, archive.describe( ) );
}

TEST_P( GeneratedArchiveTestWithStorage, describe_ouputs_resources_failed_to_load )
{
    {
        auto builder = SimpleResourcesBuilder::open( storage );
        EXPECT_TRUE( builder.is_open( ) );
        flatdata::Struct< AStruct > astruct;
        builder.set_object_resource( *astruct );
        builder.set_optional_resource( flatdata::MemoryDescriptor( "opt", 3 ) );
    }

    auto archive = SimpleResources::open( storage );
    EXPECT_FALSE( archive.is_open( ) );
    const char* const expected =
        R"data(================================================================================
Flatdata Archive: SimpleResources
================================================================================
  FATAL: Archive initialization failed. Failed loading mandatory resources.

Resource                             Optional  Loaded    Details
================================================================================
object_resource                      NO        YES       Structure of size 1
vector_resource                      NO        NO        N/A
multivector_resource                 NO        NO        N/A
raw_data_resource                    NO        NO        N/A
optional_resource                    YES       YES       Raw data of size 3
bits                                 NO        NO        N/A
================================================================================
)data";
    ASSERT_EQ( expected, archive.describe( ) );
}

TEST_P( GeneratedArchiveTestWithStorage, archive_resources_can_be_created )
{
    auto outer_builder = OuterArchiveBuilder::open( storage );
    EXPECT_TRUE( outer_builder );

    flatdata::Struct< AStruct > s;
    ( *s ).value = 17u;
    outer_builder.set_outer1( *s );
    outer_builder.set_outer2( *s );

    {
        auto inner_builder = outer_builder.inner( );
        flatdata::Struct< AStruct > s;
        ( *s ).value = 42u;
        inner_builder.set_inner( *s );
    }

    auto outer = OuterArchive::open( storage );
    EXPECT_TRUE( outer.is_open( ) );
    ASSERT_EQ( 17u, outer.outer1( ).value );
    ASSERT_EQ( 17u, outer.outer2( ).value );
    ASSERT_TRUE( outer.inner( ).is_open( ) );
    ASSERT_EQ( 42u, outer.inner( ).inner( ).value );
}

TEST_P( GeneratedArchiveTestWithStorage, archive_resources_wont_load_if_missing )
{
    auto outer_builder = OuterArchiveBuilder::open( storage );
    EXPECT_TRUE( outer_builder );

    flatdata::Struct< AStruct > o;
    ( *o ).value = 17u;
    outer_builder.set_outer1( *o );
    outer_builder.set_outer2( *o );

    auto outer = OuterArchive::open( storage );
    ASSERT_FALSE( outer.is_open( ) );
    ASSERT_FALSE( outer.inner( ).is_open( ) );
}

TEST_P( GeneratedArchiveTestWithStorage,
        only_archive_resources_can_be_incrementally_added_if_nonexisting )
{
    {
        auto outer_builder = OuterArchiveBuilder::open( storage );
        EXPECT_TRUE( outer_builder );

        flatdata::Struct< AStruct > o;
        ( *o ).value = 17u;
        outer_builder.set_outer1( *o );
    }

    auto outer_builder = OuterArchiveBuilder::open( storage );
    ASSERT_TRUE( outer_builder.is_open( ) );
    flatdata::Struct< AStruct > o;
    ASSERT_THROW( outer_builder.set_outer1( *o ), std::runtime_error );
    ASSERT_THROW( outer_builder.set_outer2( *o ), std::runtime_error );

    auto inner_builder = outer_builder.inner( );
    flatdata::Struct< AStruct > i;
    ( *i ).value = 42u;
    inner_builder.set_inner( *i );

    auto outer = OuterArchive::open( storage );
    EXPECT_FALSE( outer.is_open( ) );
    ASSERT_EQ( 17u, outer.outer1( ).value );
    ASSERT_TRUE( outer.inner( ).is_open( ) );
    ASSERT_EQ( 42u, outer.inner( ).inner( ).value );
}

TEST_P( GeneratedArchiveTestWithStorage, optional_archive_resources_behave_as_others )
{
    {
        auto outer_builder = OuterWithOptionalBuilder::open( storage );
        EXPECT_TRUE( outer_builder );

        flatdata::Struct< AStruct > o;
        ( *o ).value = 17u;
        outer_builder.set_outer( *o );
    }

    {
        auto outer = OuterWithOptional::open( storage );
        EXPECT_TRUE( outer.is_open( ) );
        ASSERT_FALSE( outer.archive_resource( ).is_initialized( ) );
        ASSERT_EQ( 17u, outer.outer( ).value );
    }

    auto outer_builder = OuterWithOptionalBuilder::open( storage );
    ASSERT_TRUE( outer_builder.is_open( ) );

    auto inner_builder = outer_builder.archive_resource( );
    flatdata::Struct< AStruct > i;
    ( *i ).value = 42u;
    inner_builder.set_inner( *i );

    auto outer = OuterWithOptional::open( storage );
    EXPECT_TRUE( outer.is_open( ) );
    ASSERT_EQ( 17u, outer.outer( ).value );
    ASSERT_TRUE( outer.archive_resource( )->is_open( ) );
    ASSERT_EQ( 42u, outer.archive_resource( )->inner( ).value );
}

TEST_P( GeneratedArchiveTestWithStorage, nested_archives_can_be_created_incrementally )
{
    flatdata::Struct< AStruct > o;
    ( *o ).value = 17u;
    {
        auto outermost_builder = OutermostArchiveBuilder::open( storage );
        outermost_builder.set_outermost( *o );
        auto outer_builder = outermost_builder.outer( );
        outer_builder.set_outer1( *o );
    }

    auto outermost_builder = OutermostArchiveBuilder::open( storage );
    ASSERT_THROW( outermost_builder.set_outermost( *o ), std::runtime_error );
    auto outer_builder = outermost_builder.outer( );
    ASSERT_THROW( outer_builder.set_outer1( *o ), std::runtime_error );
    ASSERT_THROW( outer_builder.set_outer2( *o ), std::runtime_error );

    auto inner_builder = outer_builder.inner( );
    inner_builder.set_inner( *o );
}

TEST_P( GeneratedArchiveTestWithStorage, opening_archive_doesnt_create_waste_at_target_path )
{
    auto outermost = OutermostArchive::open( storage );
    EXPECT_FALSE( outermost.is_open( ) );
    if ( !temporary_path.empty( ) )
    {
        ASSERT_TRUE( boost::filesystem::is_empty( temporary_path ) );
    }
}

INSTANTIATE_TEST_CASE_P( TestWithMemoryResourceStorage,
                         GeneratedArchiveTestWithStorage,
                         ::testing::Values( true ) );
INSTANTIATE_TEST_CASE_P( TestWithFileResourceStorage,
                         GeneratedArchiveTestWithStorage,
                         ::testing::Values( false ) );

TEST( GeneratedArchiveTest, optional_resource_is_returned_by_reference )
{
    static_assert(
        std::is_reference< decltype(
            std::declval< OnlyOptional >( ).optional_resource( ) ) >::value,
        "Optional members should be returned as references to avoid memory potential issues"
        " in release mode" );
    ASSERT_TRUE( true );
}

TEST( GeneratedArchiveTest, describe_ouputs_fatal_errors )
{
    auto archive = SimpleResources::open( std::shared_ptr< flatdata::ResourceStorage >( ) );
    EXPECT_FALSE( archive.is_open( ) );
    const char* const expected =
        R"data(================================================================================
Flatdata Archive: SimpleResources
================================================================================
  FATAL: Resource storage not initialized. Please check archive path.
================================================================================
)data";
    ASSERT_EQ( expected, archive.describe( ) );
}

TEST( GeneratedArchiveTest, describe_mismatch_schema )
{
    std::shared_ptr< MemoryResourceStorage > storage = MemoryResourceStorage::create( );
    storage->assign_value( "OutermostArchive.archive.schema",
                           R"(namespace test_structures {
struct AStruct
{
    value : u64 : 8;
}
}

namespace test_structures {
archive InnerArchive
{
    inner : .test_structures.AStruct; // THIS LINE WAS MODIFIED
}
}

namespace test_structures {
archive OuterArchive
{
    outer1 : .test_structures.AStruct;
    outer2 : .test_structures.AStruct;
    inner : archive .test_structures.InnerArchive;
}
}

namespace test_structures {
archive OutermostArchive
{
    outermost : .test_structures.AStruct; // THIS LINE WAS MODIFIED AND THE NEXT REMOVED
}
}

)" );

    std::string description = OutermostArchive::open( storage ).describe( );
    std::string expectation =
        R"(================================================================================
Flatdata Archive: OutermostArchive
================================================================================
  FATAL: Archive signature does not match software expectations.
================================================================================
 "namespace test_structures {"
 "archive InnerArchive"
 "{"
+"    inner : .test_structures.AStruct; // THIS LINE WAS MODIFIED"
-"    inner : .test_structures.AStruct;"
 "}"
 "}"
...
 "namespace test_structures {"
 "archive OutermostArchive"
 "{"
+"    outermost : .test_structures.AStruct; // THIS LINE WAS MODIFIED AND THE NEXT REMOVED"
-"    outermost : .test_structures.AStruct;"
-"    outer : archive .test_structures.OuterArchive;"
 "}"
 "}"
...
  FATAL: Archive initialization failed. Failed loading mandatory resources.

Resource                             Optional  Loaded    Details
================================================================================
outermost                            NO        NO        N/A
================================================================================
)";
    ASSERT_EQ( expectation, description );
}

}  // test
}  // flatdata
