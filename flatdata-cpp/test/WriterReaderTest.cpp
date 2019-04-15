/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/flatdata.h>
#include "catch.hpp"

using namespace flatdata;

namespace
{
enum class TestEnum
{
    Value_1,
    Value_2
};

/**
 * Use `#define GENERATE_PYTHON_READER_TESTS 1` to generate reader tests for python implementation.
 * The result is output to STDOUT. Example:
 *     buffer = "\x00\x01\x00\x00\x00\x00\x00\x00\x00\x00"
 *     assert_equal(byte_reader(buffer, 8, 3, ctypes.c_uint8), 1)
 */
#define GENERATE_PYTHON_READER_TESTS 0
#if ( GENERATE_PYTHON_READER_TESTS )
/* clang-format off */
template < typename T > struct ctypes_type { };
template <> struct ctypes_type< uint8_t > { static const char* value( ) { return "False"; } };
template <> struct ctypes_type< uint16_t > { static const char* value( ) { return "False"; } };
template <> struct ctypes_type< uint32_t > { static const char* value( ) { return "False"; } };
template <> struct ctypes_type< uint64_t > { static const char* value( ) { return "False"; } };
template <> struct ctypes_type< int8_t > { static const char* value( ) { return "True"; } };
template <> struct ctypes_type< int16_t > { static const char* value( ) { return "True"; } };
template <> struct ctypes_type< int32_t > { static const char* value( ) { return "True"; } };
template <> struct ctypes_type< int64_t > { static const char* value( ) { return "True"; } };
template <> struct ctypes_type< TestEnum > { static const char* value( ) { return "True"; } };
template < typename T > T as_numeric( T value ) { return value; }
int32_t as_numeric( TestEnum value ) { return static_cast< int32_t >( value ); }
uint16_t as_numeric( uint8_t value ) { return static_cast< uint16_t >( value ); }
/* clang-format on */
#endif  // GENERATE_PYTHON_READER_TESTS

template < template < typename, int, int, int > class Reader,
           typename type,
           int offset,
           int num_bits,
           bool pre_fill_buffer >
void
test_value( type value )
{
    std::array< uint8_t, sizeof( type ) + ( offset + 7 ) / 8 + 8 > buffer = {{0}};
    if ( pre_fill_buffer )
    {
        for ( auto& value : buffer )
        {
            value = 0xff;
        }
    }
    Writer< type, offset, num_bits, 0 > writer{buffer.data( )};
    writer = value;

#if ( GENERATE_PYTHON_READER_TESTS )
    std::cout << "_test_reader(\"";
    for ( uint8_t c : buffer )
    {
        std::cout << "\\x" << std::hex << std::setfill( '0' ) << std::setw( 2 )
                  << static_cast< uint16_t >( c );
    }
    std::cout << std::dec << "\""
              << ", " << offset << ", " << num_bits << ", " << ctypes_type< type >::value( ) << ", "
              << as_numeric( value ) << ")" << std::endl
              << std::endl;
#endif  // GENERATE_PYTHON_READER_TESTS

    Reader< type, offset, num_bits, 0 > reader{buffer.data( )};
    type result = reader;
    REQUIRE( result == value );
}

/// tests all possible num_bits
template < template < typename, int, int, int > class Reader,
           typename type,
           int offset,
           int num_bits >
struct TestBits
{
    void
    operator( )( )
    {
        type value = 1;
#if ( !GENERATE_PYTHON_READER_TESTS )
        for ( int i = 0; i < num_bits; i++ )
        {
#endif  // !GENERATE_PYTHON_READER_TESTS
            test_value< Reader, type, offset, num_bits, false >( value );
            test_value< Reader, type, offset, num_bits, true >( value );
#if ( !GENERATE_PYTHON_READER_TESTS )
            value <<= 1;
        }
#endif  // !GENERATE_PYTHON_READER_TESTS
        TestBits< Reader, type, offset, num_bits - 1 >( )( );
    }
};

template < template < typename, int, int, int > class Reader, typename type, int offset >
struct TestBits< Reader, type, offset, 0 >
{
    void
    operator( )( )
    {
    }
};

template < template < typename, int, int, int > class Reader, typename type, int offset >
struct TestOffsets
{
    void
    operator( )( )
    {
        TestBits< Reader, type, offset, sizeof( type ) * 8 >( )( );
        TestOffsets< Reader, type, offset - 1 >( )( );
    }
};

template < template < typename, int, int, int > class Reader, typename type >
struct TestOffsets< Reader, type, 0 >
{
    void
    operator( )( )
    {
        TestBits< Reader, type, 0, sizeof( type ) * 8 >( )( );
    }
};
}  // namespace

TEST_CASE( "Read uint8", "[Reader]" )
{
    TestOffsets< Reader, uint8_t, 8 >( )( );
}

TEST_CASE( "Read uint16", "[Reader]" )
{
    TestOffsets< Reader, uint16_t, 8 >( )( );
}

TEST_CASE( "Read uint32", "[Reader]" )
{
    TestOffsets< Reader, uint32_t, 8 >( )( );
}

TEST_CASE( "Read uint64", "[Reader]" )
{
    TestOffsets< Reader, uint64_t, 8 >( )( );
}

TEST_CASE( "Read negative number with sign extension", "[Reader]" )
{
    int32_t value = -1;
    for ( int i = 0; i < 15; i++ )
    {
        test_value< Reader, int32_t, 3, 16, false >( value );
        test_value< Reader, int32_t, 3, 16, true >( value );
        value *= 2;
    }
}

TEST_CASE( "Read positive signed number with sign extension", "[Reader]" )
{
    int32_t value = 1;
    for ( int i = 0; i < 15; i++ )
    {
        test_value< Reader, int32_t, 3, 16, false >( value );
        test_value< Reader, int32_t, 3, 16, true >( value );
        value *= 2;
    }
}

TEST_CASE( "Read enum", "[Reader]" )
{
    test_value< Reader, TestEnum, 3, 2, true >( TestEnum::Value_1 );
    test_value< Reader, TestEnum, 3, 2, false >( TestEnum::Value_1 );
}

TEST_CASE( "Range", "[Read/Write]" )
{
    std::array< uint8_t, 64 > buffer = {{0}};
    Writer< uint32_t, 7, 7, 0 > writer_start{buffer.data( )};
    writer_start = 16;
    Writer< uint32_t, 7, 7, 0 > writer_end{buffer.data( ) + 32};
    writer_end = 32;

    Reader< std::pair< uint32_t, uint32_t >, 7, 7, 32 > reader{buffer.data( )};
    std::pair< uint32_t, uint32_t > result = reader;
    REQUIRE( result.first == 16 );
    REQUIRE( result.second == 32 );
}

#if ( GENERATE_PYTHON_READER_TESTS )
TEST_CASE( "Fail if python test output is enabled", "[Reader]" )
{
    // This test will fail if GENERATE_PYTHON_READER_TESTS is specified so that it cannot be
    // accidentally submitted.
    REQUIRE( false );
}
#endif  // GENERATE_PYTHON_READER_TESTS
