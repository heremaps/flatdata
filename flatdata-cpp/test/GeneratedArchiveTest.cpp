/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/FileResourceStorage.h>
#include <flatdata/MemoryResourceStorage.h>
#include <boost/filesystem.hpp>
#include "catch.hpp"

#include <type_traits>

using test_structures::AStruct;
using test_structures::BStruct;
using test_structures::OnlyOptional;
using test_structures::OnlyOptionalBuilder;
using test_structures::OuterArchive;
using test_structures::OuterArchiveBuilder;
using test_structures::OutermostArchive;
using test_structures::OutermostArchiveBuilder;
using test_structures::OuterWithOptional;
using test_structures::OuterWithOptionalBuilder;
using test_structures::SimpleResources;
using test_structures::SimpleResourcesBuilder;

namespace flatdata
{
namespace test
{
template < typename UseMemoryResourceStorage >
struct Fixture
{
    Fixture( )
    {
        if ( UseMemoryResourceStorage::value )
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
            FAIL( "Cannot create temporary directory to test FileResourceStorage at "
                  << temporary_path );
        }
        storage = FileResourceStorage::create( temporary_path.c_str( ) );
    }

    ~Fixture( )
    {
        if ( UseMemoryResourceStorage::value )
        {
            return;
        }
        boost::system::error_code ec;
        boost::filesystem::remove_all( temporary_path, ec );
        if ( ec )
        {
            FAIL( "Failed to remove temporary directory " << temporary_path );
        }
        temporary_path.clear( );
    }

    std::shared_ptr< ResourceStorage > storage;
    boost::filesystem::path temporary_path;
};

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "objects_can_be_read_and_written",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto builder = SimpleResourcesBuilder::open( Fixture< TestType >::storage );

    CHECK( builder.is_open( ) );
    std::vector< uint8_t > data( AStruct::size_in_bytes( ) );
    AStruct::MutatorType object( data.data( ) );
    object.value = 7;
    builder.set_object_resource( object );

    auto archive = SimpleResources::open( Fixture< TestType >::storage );
    CHECK_FALSE( archive.is_open( ) );
    REQUIRE( archive.object_resource( ).value == 7u );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Vectors can be read and written",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto builder = SimpleResourcesBuilder::open( Fixture< TestType >::storage );

    CHECK( builder.is_open( ) );
    flatdata::Vector< AStruct > v;
    v.grow( ).value = 17;
    v.grow( ).value = 42;
    builder.set_vector_resource( v );

    auto archive = SimpleResources::open( Fixture< TestType >::storage );
    CHECK_FALSE( archive.is_open( ) );
    REQUIRE( archive.vector_resource( ).size( ) == 2u );
    REQUIRE( archive.vector_resource( )[ 0 ].value == 17u );
    REQUIRE( archive.vector_resource( )[ 1 ].value == 42u );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Vectors can be created incrementally",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto builder = SimpleResourcesBuilder::open( Fixture< TestType >::storage );

    CHECK( builder.is_open( ) );
    auto resource = builder.start_vector_resource( );
    resource.grow( ).value = 17;
    resource.grow( ).value = 42;
    resource.close( );

    auto archive = SimpleResources::open( Fixture< TestType >::storage );
    CHECK_FALSE( archive.is_open( ) );
    REQUIRE( archive.vector_resource( ).size( ) == 2u );
    REQUIRE( archive.vector_resource( )[ 0 ].value == 17u );
    REQUIRE( archive.vector_resource( )[ 1 ].value == 42u );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Multivectors can be created incrementally",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto builder = SimpleResourcesBuilder::open( Fixture< TestType >::storage );

    CHECK( builder.is_open( ) );
    auto resource = builder.start_multivector_resource( );
    auto list = resource.grow( );
    list.template add< AStruct >( ).value = 17;
    list = resource.grow( );
    list.template add< AStruct >( ).value = 42;
    resource.close( );

    auto archive = SimpleResources::open( Fixture< TestType >::storage );
    CHECK_FALSE( archive.is_open( ) );
    REQUIRE( archive.multivector_resource( ).size( ) == 2u );

    archive.multivector_resource( ).template for_each< AStruct >(
        0, [&]( const AStruct& s ) { REQUIRE( s.value == 17u ); } );
    archive.multivector_resource( ).template for_each< AStruct >(
        1, [&]( const AStruct& s ) { REQUIRE( s.value == 42u ); } );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Raw data can be read and written",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto builder = SimpleResourcesBuilder::open( Fixture< TestType >::storage );

    CHECK( builder.is_open( ) );
    builder.set_raw_data_resource( MemoryDescriptor( "abc\0", 4 ) );

    auto archive = SimpleResources::open( Fixture< TestType >::storage );
    CHECK_FALSE( archive.is_open( ) );
    REQUIRE( archive.raw_data_resource( ).char_ptr( ) == std::string( "abc" ) );
    REQUIRE( archive.raw_data_resource( ).size_in_bytes( ) == 4u );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Optional resource is correct when available",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto builder = OnlyOptionalBuilder::open( Fixture< TestType >::storage );

    CHECK( builder.is_open( ) );
    builder.set_optional_resource( MemoryDescriptor( "abc\0", 4 ) );

    auto archive = OnlyOptional::open( Fixture< TestType >::storage );
    CHECK( archive.is_open( ) );
    REQUIRE( archive.optional_resource( ).is_initialized( ) );
    REQUIRE( archive.optional_resource( )->char_ptr( ) == std::string( "abc" ) );
    REQUIRE( archive.optional_resource( )->size_in_bytes( ) == 4u );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Optional resource is uninitialized when not available",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto builder = OnlyOptionalBuilder::open( Fixture< TestType >::storage );
    CHECK( builder.is_open( ) );

    auto archive = OnlyOptional::open( Fixture< TestType >::storage );
    CHECK( archive.is_open( ) );
    REQUIRE( !archive.optional_resource( ).is_initialized( ) );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Describe outputs resources as expected",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    {
        auto builder = SimpleResourcesBuilder::open( Fixture< TestType >::storage );
        CHECK( builder.is_open( ) );
        flatdata::Struct< AStruct > astruct;
        builder.set_object_resource( *astruct );
        flatdata::Vector< AStruct > avector( 11 );
        builder.set_vector_resource( avector );
        builder.set_optional_resource( flatdata::MemoryDescriptor( "opt", 3 ) );
        builder.set_raw_data_resource( flatdata::MemoryDescriptor( "raw_data", 8 ) );

        auto mv = builder.start_multivector_resource( );
        auto list = mv.grow( );
        list.template add< AStruct >( ).value = 17;
        mv.close( );
    }

    auto archive = SimpleResources::open( Fixture< TestType >::storage );
    CHECK( archive.is_open( ) );
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
================================================================================
)data";
    REQUIRE( archive.describe( ) == expected );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Describe ouputs resources failed to load",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    {
        auto builder = SimpleResourcesBuilder::open( Fixture< TestType >::storage );
        CHECK( builder.is_open( ) );
        flatdata::Struct< AStruct > astruct;
        builder.set_object_resource( *astruct );
        builder.set_optional_resource( flatdata::MemoryDescriptor( "opt", 3 ) );
    }

    auto archive = SimpleResources::open( Fixture< TestType >::storage );
    CHECK_FALSE( archive.is_open( ) );
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
================================================================================
)data";
    REQUIRE( archive.describe( ) == expected );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Archive resources can be created",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto outer_builder = OuterArchiveBuilder::open( Fixture< TestType >::storage );
    CHECK( outer_builder );

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

    auto outer = OuterArchive::open( Fixture< TestType >::storage );
    CHECK( outer.is_open( ) );
    REQUIRE( outer.outer1( ).value == 17u );
    REQUIRE( outer.outer2( ).value == 17u );
    REQUIRE( outer.inner( ).is_open( ) );
    REQUIRE( outer.inner( ).inner( ).value == 42u );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Archive resources wont load if missing",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto outer_builder = OuterArchiveBuilder::open( Fixture< TestType >::storage );
    CHECK( outer_builder );

    flatdata::Struct< AStruct > o;
    ( *o ).value = 17u;
    outer_builder.set_outer1( *o );
    outer_builder.set_outer2( *o );

    auto outer = OuterArchive::open( Fixture< TestType >::storage );
    REQUIRE_FALSE( outer.is_open( ) );
    REQUIRE_FALSE( outer.inner( ).is_open( ) );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Only archive resources can be incrementally added if nonexisting",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    {
        auto outer_builder = OuterArchiveBuilder::open( Fixture< TestType >::storage );
        CHECK( outer_builder );

        flatdata::Struct< AStruct > o;
        ( *o ).value = 17u;
        outer_builder.set_outer1( *o );
    }

    auto outer_builder = OuterArchiveBuilder::open( Fixture< TestType >::storage );
    REQUIRE( outer_builder.is_open( ) );
    flatdata::Struct< AStruct > o;
    REQUIRE_THROWS_AS( outer_builder.set_outer1( *o ), std::runtime_error );
    REQUIRE_THROWS_AS( outer_builder.set_outer2( *o ), std::runtime_error );

    auto inner_builder = outer_builder.inner( );
    flatdata::Struct< AStruct > i;
    ( *i ).value = 42u;
    inner_builder.set_inner( *i );

    auto outer = OuterArchive::open( Fixture< TestType >::storage );
    CHECK_FALSE( outer.is_open( ) );
    REQUIRE( outer.outer1( ).value == 17u );
    REQUIRE( outer.inner( ).is_open( ) );
    REQUIRE( outer.inner( ).inner( ).value == 42u );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Optional archive resources behave as others",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    {
        auto outer_builder = OuterWithOptionalBuilder::open( Fixture< TestType >::storage );
        CHECK( outer_builder );

        flatdata::Struct< AStruct > o;
        ( *o ).value = 17u;
        outer_builder.set_outer( *o );
    }

    {
        auto outer = OuterWithOptional::open( Fixture< TestType >::storage );
        CHECK( outer.is_open( ) );
        REQUIRE_FALSE( outer.archive_resource( ).is_initialized( ) );
        REQUIRE( outer.outer( ).value == 17u );
    }

    auto outer_builder = OuterWithOptionalBuilder::open( Fixture< TestType >::storage );
    REQUIRE( outer_builder.is_open( ) );

    auto inner_builder = outer_builder.archive_resource( );
    flatdata::Struct< AStruct > i;
    ( *i ).value = 42u;
    inner_builder.set_inner( *i );

    auto outer = OuterWithOptional::open( Fixture< TestType >::storage );
    CHECK( outer.is_open( ) );
    REQUIRE( outer.outer( ).value == 17u );
    REQUIRE( outer.archive_resource( )->is_open( ) );
    REQUIRE( outer.archive_resource( )->inner( ).value == 42u );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Nested archives can be created incrementally",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    flatdata::Struct< AStruct > o;
    ( *o ).value = 17u;
    {
        auto outermost_builder = OutermostArchiveBuilder::open( Fixture< TestType >::storage );
        outermost_builder.set_outermost( *o );
        auto outer_builder = outermost_builder.outer( );
        outer_builder.set_outer1( *o );
    }

    auto outermost_builder = OutermostArchiveBuilder::open( Fixture< TestType >::storage );
    REQUIRE_THROWS_AS( outermost_builder.set_outermost( *o ), std::runtime_error );
    auto outer_builder = outermost_builder.outer( );
    REQUIRE_THROWS_AS( outer_builder.set_outer1( *o ), std::runtime_error );
    REQUIRE_THROWS_AS( outer_builder.set_outer2( *o ), std::runtime_error );

    auto inner_builder = outer_builder.inner( );
    inner_builder.set_inner( *o );
}

TEMPLATE_TEST_CASE_METHOD( Fixture,
                           "Opening archive doesn't create waste at target path",
                           "[GeneratedArchive]",
                           std::true_type,
                           std::false_type )
{
    auto outermost = OutermostArchive::open( Fixture< TestType >::storage );
    CHECK_FALSE( outermost.is_open( ) );
    if ( !Fixture< TestType >::temporary_path.empty( ) )
    {
        REQUIRE( boost::filesystem::is_empty( Fixture< TestType >::temporary_path ) );
    }
}

TEST_CASE( "Optional resource is returned by reference", "[GeneratedArchive]" )
{
    static_assert(
        std::is_reference< decltype(
            std::declval< OnlyOptional >( ).optional_resource( ) ) >::value,
        "Optional members should be returned as references to avoid memory potential issues"
        " in release mode" );
    REQUIRE( true );
}

TEST_CASE( "Describe ouputs fatal errors", "[GeneratedArchive]" )
{
    auto archive = SimpleResources::open( std::shared_ptr< flatdata::ResourceStorage >( ) );
    CHECK_FALSE( archive.is_open( ) );
    const char* const expected =
        R"data(================================================================================
Flatdata Archive: SimpleResources
================================================================================
  FATAL: Resource storage not initialized. Please check archive path.
================================================================================
)data";
    REQUIRE( archive.describe( ) == expected );
}

TEST_CASE( "Describe mismatch schema", "[GeneratedArchive]" )
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
    REQUIRE( description == expectation );
}

}  // namespace test
}  // namespace flatdata
