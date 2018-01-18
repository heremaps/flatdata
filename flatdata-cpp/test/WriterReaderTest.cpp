/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

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

template < template < typename, int, int > class Reader,
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
    Writer< type, offset, num_bits > writer{buffer.data( )};
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

    Reader< type, offset, num_bits > reader{buffer.data( )};
    type result = reader;
    ASSERT_EQ( value, result );
}

/// tests all possible num_bits
template < template < typename, int, int > class Reader, typename type, int offset, int num_bits >
struct TestBits
{
    void operator( )( )
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

template < template < typename, int, int > class Reader, typename type, int offset >
struct TestBits< Reader, type, offset, 0 >
{
    void operator( )( )
    {
    }
};

template < template < typename, int, int > class Reader, typename type, int offset >
struct TestOffsets
{
    void operator( )( )
    {
        TestBits< Reader, type, offset, sizeof( type ) * 8 >( )( );
        TestOffsets< Reader, type, offset - 1 >( )( );
    }
};

template < template < typename, int, int > class Reader, typename type >
struct TestOffsets< Reader, type, 0 >
{
    void operator( )( )
    {
        TestBits< Reader, type, 0, sizeof( type ) * 8 >( )( );
    }
};

}  // anonymous namespace

TEST( TestReader, TestUint8 )
{
    TestOffsets< Reader, uint8_t, 8 >( )( );
}

TEST( TestReader, TestUint16 )
{
    TestOffsets< Reader, uint16_t, 8 >( )( );
}

TEST( TestReader, TestUint32 )
{
    TestOffsets< Reader, uint32_t, 8 >( )( );
}

TEST( TestReader, TestUint64 )
{
    TestOffsets< Reader, uint64_t, 8 >( )( );
}

TEST( TestReader, TestNegativeNumberWithSignExtension )
{
    int32_t value = -1;
    for ( int i = 0; i < 15; i++ )
    {
        test_value< Reader, int32_t, 3, 16, false >( value );
        test_value< Reader, int32_t, 3, 16, true >( value );
        value *= 2;
    }
}

TEST( TestReader, TestPositiveSignedNumberWithSignExtension )
{
    int32_t value = 1;
    for ( int i = 0; i < 15; i++ )
    {
        test_value< Reader, int32_t, 3, 16, false >( value );
        test_value< Reader, int32_t, 3, 16, true >( value );
        value *= 2;
    }
}

TEST( TestReader, TestEnum )
{
    test_value< Reader, TestEnum, 3, 2, true >( TestEnum::Value_1 );
    test_value< Reader, TestEnum, 3, 2, false >( TestEnum::Value_1 );
}

#if ( GENERATE_PYTHON_READER_TESTS )
TEST( TestReader, FailIfPythonTestOutputIsEnabled )
{
    // This test will fail if GENERATE_PYTHON_READER_TESTS is specified so that it cannot be
    // accidentally submitted.
    FAIL( );
}
#endif  // GENERATE_PYTHON_READER_TESTS
