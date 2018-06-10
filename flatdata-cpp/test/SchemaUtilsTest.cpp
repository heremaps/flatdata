/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/internal/SchemaUtils.h>
#include <gtest/gtest.h>

namespace flatdata
{
namespace internal
{
namespace
{
bool
string_schema_equal( const std::string& s1, const std::string& s2 )
{
    return schema_equal( s1.data( ), s1.size( ), s2.data( ), s2.size( ) );
}

bool
string_schema_unequal( const std::string& s1, const std::string& s2 )
{
    return !string_schema_equal( s1, s2 );
}

template < size_t N >
std::string
S( const char ( &s )[ N ] )
{
    static_assert( N > 0, "zero sized string array passed" );
    return std::string( s, s + ( N - 1 ) );
}
}  // anonymous namespace

TEST( TestSchemaEqual, empty )
{
    EXPECT_PRED2( string_schema_equal, "", "" );
    EXPECT_PRED2( string_schema_unequal, "x", "" );
}

TEST( TestSchemaEqual, whitespace )
{
    EXPECT_PRED2( string_schema_equal, "    ", "" );
    EXPECT_PRED2( string_schema_equal, "    \tx\n    ", "x" );
}

TEST( TestSchemaEqual, number )
{
    EXPECT_PRED2( string_schema_equal, "10", "10" );
    EXPECT_PRED2( string_schema_equal, "010", "10" );
    EXPECT_PRED2( string_schema_equal, "0010", "000010" );
    EXPECT_PRED2( string_schema_equal, "10", "0xa" );
    EXPECT_PRED2( string_schema_equal, "0xdeadbeef", "0xDeadBeef" );
    EXPECT_PRED2( string_schema_equal, "10", "0x0000000a" );
    EXPECT_PRED2( string_schema_unequal, "10", "00x0000000a" );
    EXPECT_PRED2( string_schema_unequal, "10a", "10 a" );
    EXPECT_PRED2( string_schema_unequal, "0xabcdefg", "0xabcdef g" );
}

TEST( TestSchemaEqual, invalid )
{
    EXPECT_PRED2( string_schema_equal, "!", "!" );
    EXPECT_PRED2( string_schema_equal, "   !    ", "!" );
    EXPECT_PRED2( string_schema_equal, "   !    ", "!" );
    EXPECT_PRED2( string_schema_unequal, "!", "?" );
    EXPECT_PRED2( string_schema_unequal, "123abc", "123 abc" );
    EXPECT_PRED2( string_schema_equal, "10a", "10a" );
    EXPECT_PRED2( string_schema_equal, "0xabcdefg", "0xabcdefg" );
}

TEST( TestSchemaEqual, comment )
{
    EXPECT_PRED2( string_schema_unequal, "", "/" );
    EXPECT_PRED2( string_schema_equal, "", "// insightful comment\n" );
    EXPECT_PRED2( string_schema_equal, "", "// insightful comment" );
    EXPECT_PRED2( string_schema_equal, "", "// insightful comment" );
    EXPECT_PRED2( string_schema_equal, "a b", "a /* insightful comment */ b" );
}

TEST( TestSchemaEqual, embedded_nul )
{
    EXPECT_PRED2( string_schema_unequal, "", S( "\x00" ) );
    EXPECT_PRED2( string_schema_unequal, "a b", S( "a\x00b" ) );
    EXPECT_PRED2( string_schema_unequal, "a\x00 b", S( "a \x00b" ) );
}

TEST( TestSchemaEqual, test_structures )
{
    const char* s1 = R"(
// this is the test_structures namespace
namespace test_structures {

// a structure
struct AStruct {
    /* field name */ value
    :
    /* field type */ u64
    :
    /* bit width of field */
    8;
}

// an archive
archive SimpleResources {
    object_resource: AStruct;
    vector_resource: vector< AStruct >;
    multivector_resource: multivector< 33, AStruct >;
    raw_data_resource: raw_data;
    @optional // optional field
    optional_resource: raw_data;
}

} // namespace test_structures
)";
    const char* s2
        = "namespace test_structures{"
          "struct AStruct{value:u64:8;}"
          "archive SimpleResources{"
          "object_resource:AStruct;"
          "vector_resource:vector<AStruct>;"
          "multivector_resource:multivector<33,AStruct>;"
          "raw_data_resource:raw_data;"
          "@optional optional_resource: raw_data;"
          "}}";

    EXPECT_PRED2( string_schema_equal, s1, s2 );
}

}  // namespace internal
}  // namespace flatdata
