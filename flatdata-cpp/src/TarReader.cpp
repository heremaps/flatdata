/**
 * Copyright (c) 2021 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/internal/TarReader.h>

#include <array>
#include <cassert>
#include <cstring>
#include <limits>
#include <stdexcept>

namespace flatdata
{
namespace internal
{
namespace
{
/* tar Header Block, from POSIX 1003.1-1990.  */
struct TarFileHeader
{                         /* byte offset */
    char name[ 100 ];     /*   0 */
    char mode[ 8 ];       /* 100 */
    char uid[ 8 ];        /* 108 */
    char gid[ 8 ];        /* 116 */
    char size[ 12 ];      /* 124 */
    char mtime[ 12 ];     /* 136 */
    char chksum[ 8 ];     /* 148 */
    char typeflag;        /* 156 */
    char linkname[ 100 ]; /* 157 */
    char magic[ 6 ];      /* 257 */
    char version[ 2 ];    /* 263 */
    char uname[ 32 ];     /* 265 */
    char gname[ 32 ];     /* 297 */
    char devmajor[ 8 ];   /* 329 */
    char devminor[ 8 ];   /* 337 */
    char prefix[ 155 ];   /* 345 */
    char padding[ 12 ];   /* 500 */
                          /* 512 */
};

/* Values used in typeflag field.  */
const char REGTYPE = '0';           /* regular file */
const char AREGTYPE = '\0';         /* regular file */
const char DIRTYPE = '5';           /* directory */
const char GNUTYPE_LONGNAME = 'L';  // Identifies the *next* file on the tape as having a long name.

uint64_t
decode_numeric_field( const char* data, size_t size )
{
    assert( size > 0 );

    const uint64_t MAX_VALUE = std::numeric_limits< size_t >::max( );

    if ( *data & 0x80 )  // Highest byte set indicates base-256 encoding
    {
        uint64_t value = ( *data & 0x7F );
        for ( size_t idx = 1; idx < size; ++idx )
        {
            if ( value > MAX_VALUE / 256 )
            {
                throw std::runtime_error( "Numeric value too large" );
            }
            value = value * 256 + static_cast< unsigned char >( data[ idx ] );
        }
        return value;
    }

    // Decode octal number
    uint64_t value = 0;
    for ( size_t idx = 0; idx < size && data[ idx ] != '\0'; ++idx )
    {
        if ( value > MAX_VALUE / 8 )
        {
            throw std::runtime_error( "Numeric value too large" );
        }
        if ( data[ idx ] < '0' || data[ idx ] > '7' )
        {
            throw std::runtime_error( "Unexpected character" );
        }
        value = value * 8 + static_cast< unsigned char >( data[ idx ] - '0' );
    }
    return value;
}

bool
verify_checksum( const TarFileHeader& header )
{
    TarFileHeader header_copy = header;

    memset( header_copy.chksum, ' ', sizeof( header_copy.chksum ) );
    uint32_t sum = 0;
    for ( size_t idx = 0; idx < sizeof( header_copy ); ++idx )
    {
        sum += ( reinterpret_cast< unsigned char* >( &header_copy ) )[ idx ];
    }

    return sum == decode_numeric_field( header.chksum, sizeof( header.chksum ) );
}
}  // namespace

std::vector< TarFileEntry >
read_tar_file_entries( MemoryDescriptor data )
{
    static const size_t BLOCK_SIZE = 512;
    static_assert( sizeof( TarFileHeader ) == BLOCK_SIZE, "" );

    if ( data.size( ) % BLOCK_SIZE != 0 )
    {
        throw std::runtime_error( "TAR size is not multiple of 512" );
    }

    std::vector< TarFileEntry > file_entries;

    std::array< char, BLOCK_SIZE > zero_block{ };
    std::string long_name;
    size_t offset = 0;

    while ( offset < data.size( ) )
    {
        TarFileHeader header;
        std::memcpy( &header, data.char_ptr( ) + offset, BLOCK_SIZE );
        offset += BLOCK_SIZE;

        if ( std::memcmp( &header, &zero_block, BLOCK_SIZE ) == 0 )
        {
            // Zero block indicates end of TAR
            break;
        }

        if ( !verify_checksum( header ) )
        {
            throw std::runtime_error( "Incorrect TAR header checksum" );
        }

        if ( header.typeflag == REGTYPE || header.typeflag == AREGTYPE )
        {
            std::string file_name( header.name, strnlen( header.name, sizeof( header.name ) ) );
            if ( !long_name.empty( ) )
            {
                file_name = std::move( long_name );
                long_name.clear( );
            }

            const size_t file_size = decode_numeric_field( header.size, sizeof( header.size ) );

            TarFileEntry file;
            file.name = file_name;
            file.offset = offset;
            file.size = file_size;
            file_entries.emplace_back( file );

            const size_t padding = ( BLOCK_SIZE - ( file_size % BLOCK_SIZE ) ) % BLOCK_SIZE;
            offset += file_size + padding;
        }
        else if ( header.typeflag == DIRTYPE )
        {
            // Skip directories
        }
        else if ( header.typeflag == GNUTYPE_LONGNAME )
        {
            const size_t name_size = decode_numeric_field( header.size, sizeof( header.size ) );
            const size_t padding = ( BLOCK_SIZE - ( name_size % BLOCK_SIZE ) ) % BLOCK_SIZE;

            assert( data.size( ) >= offset );
            if ( name_size > data.size( ) - offset )
            {
                throw std::runtime_error( "Unexpected end of TAR file" );
            }
            long_name = std::string( data.char_ptr( ) + offset,
                                     strnlen( data.char_ptr( ) + offset, name_size ) );
            offset += name_size + padding;
        }
        else
        {
            throw std::runtime_error( "Unsupported TAR file type encountered" );
        }
    }

    return file_entries;
}
}  // namespace internal
}  // namespace flatdata
