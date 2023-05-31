/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "ResourceStorage.h"
#include "DebugDataAccessStatistics.h"

#include <bitset>
#include <iomanip>
#include <map>
#include <mutex>
#include <set>
#include <sstream>
#include <stdexcept>

namespace flatdata
{
class MemoryResourceStorage : public ResourceStorage
{
    struct Storage
    {
        std::map< std::string, std::shared_ptr< std::stringstream > > streams;
        std::map< std::string, std::shared_ptr< std::string > > resources;
        std::set< std::string > created_directories;
    };

public:
    static std::unique_ptr< MemoryResourceStorage > create( );
    ~MemoryResourceStorage( );
    bool exists( const char* key ) override;
    std::unique_ptr< ResourceStorage > create_directory( const char* key ) override;
    std::unique_ptr< ResourceStorage > directory( const char* key ) override;

    std::string hexdump( bool dump_schemas = false ) const;
    std::string bindump( bool dump_schemas = false ) const;

    MemoryDescriptor read_resource( const char* key ) override;

    void assign_value( const char* key, MemoryDescriptor value );
    void assign_value( const char* key, const char* value );

protected:
    std::shared_ptr< std::ostream > create_output_stream( const char* key ) override;

private:
    MemoryResourceStorage( std::shared_ptr< Storage > storage, const std::string& path );
    std::string get_path( const char* key );

    template < typename ResourceSerializer >
    std::string dump_resources( bool dump_schemas, ResourceSerializer&& f ) const;

private:
    std::shared_ptr< Storage > m_storage;
    mutable std::mutex m_storage_mutex;
    std::string m_path;
};

// -------------------------------------------------------------------------------------------------

inline std::string
MemoryResourceStorage::get_path( const char* key )
{
    return m_path + key;
}

inline std::shared_ptr< std::ostream >
MemoryResourceStorage::create_output_stream( const char* key )
{
    std::shared_ptr< std::stringstream > result(
        new std::stringstream( std::ofstream::out | std::ofstream::binary ) );

    std::lock_guard< std::mutex > lock( m_storage_mutex );
    m_storage->streams[ get_path( key ) ] = result;

    return result;
}

inline std::unique_ptr< MemoryResourceStorage >
MemoryResourceStorage::create( )
{
    return std::unique_ptr< MemoryResourceStorage >(
        new MemoryResourceStorage( std::make_shared< MemoryResourceStorage::Storage >( ), "" ) );
}

inline MemoryResourceStorage::MemoryResourceStorage( std::shared_ptr< Storage > storage,
                                                     const std::string& path )
    : m_storage( storage )
    , m_path( path )
{
}

inline MemoryResourceStorage::~MemoryResourceStorage( )
{
    for ( auto& resource : m_storage->resources )
    {
        auto& string = resource.second;
#ifdef DEBUG_DATA_ACCESS_STATISTICS
        DebugDataAccessStatistics::deregister_mapping( MemoryDescriptor(
            reinterpret_cast< const unsigned char* >( string->c_str( ) ), string->size( ) ) );
#endif
    }
}

inline MemoryDescriptor
MemoryResourceStorage::read_resource( const char* key )
{
    const auto path = get_path( key );

    std::lock_guard< std::mutex > lock( m_storage_mutex );
    if ( m_storage->resources.count( path ) == 0 )
    {
        auto found = m_storage->streams.find( path );
        if ( found == m_storage->streams.end( ) )
        {
            return MemoryDescriptor( );
        }
        m_storage->resources[ path ].reset( new std::string( found->second->str( ) ) );
#ifdef DEBUG_DATA_ACCESS_STATISTICS
        auto& string = m_storage->resources[ path ];
        DebugDataAccessStatistics::register_mapping(
            path.c_str( ),
            MemoryDescriptor( reinterpret_cast< const unsigned char* >( string->c_str( ) ),
                              string->size( ) ) );
#endif
    }
    auto& string = m_storage->resources[ path ];

    return MemoryDescriptor( reinterpret_cast< const unsigned char* >( string->c_str( ) ),
                             string->size( ) );
}

inline void
MemoryResourceStorage::assign_value( const char* key, MemoryDescriptor value )
{
    std::lock_guard< std::mutex > lock( m_storage_mutex );
    m_storage->resources.insert( std::make_pair(
        std::string( key ),
        std::make_shared< std::string >( value.char_ptr( ), value.size_in_bytes( ) ) ) );
}

inline void
MemoryResourceStorage::assign_value( const char* key, const char* value )
{
    std::lock_guard< std::mutex > lock( m_storage_mutex );
    m_storage->resources.insert(
        std::make_pair( std::string( key ), std::make_shared< std::string >( value ) ) );
}

inline std::unique_ptr< ResourceStorage >
MemoryResourceStorage::create_directory( const char* key )
{
    const auto new_path = get_path( key ) + "/";
    std::lock_guard< std::mutex > lock( m_storage_mutex );
    m_storage->created_directories.insert( new_path );
    return std::unique_ptr< MemoryResourceStorage >(
        new MemoryResourceStorage( m_storage, new_path ) );
}

inline std::unique_ptr< ResourceStorage >
MemoryResourceStorage::directory( const char* key )
{
    const auto new_path = get_path( key ) + "/";
    std::lock_guard< std::mutex > lock( m_storage_mutex );
    if ( m_storage->created_directories.find( new_path ) == m_storage->created_directories.end( ) )
    {
        return nullptr;
    }
    return std::unique_ptr< MemoryResourceStorage >(
        new MemoryResourceStorage( m_storage, new_path ) );
}

namespace helpers
{
template < typename Map >
bool
prefix_exists( const Map& m, const std::string& prefix )
{
    for ( const auto& p : m )
    {
        if ( p.first.find( prefix ) == 0 )
        {
            return true;
        }
    }
    return false;
}

template < typename CharSerializer >
void
dump_resource( const std::string& key,
               const std::string& contents,
               size_t line_size,
               CharSerializer&& serializer,
               size_t element_width,
               std::ostream& out )
{
    out << std::endl << key << " (" << std::dec << contents.size( ) << " bytes):" << std::endl;

    std::string line;
    for ( size_t char_idx = 0; char_idx < contents.size( ); ++char_idx )
    {
        const uint8_t character = contents[ char_idx ];
        if ( ( char_idx % line_size ) == 0 )
        {
            if ( char_idx != 0 )
            {
                out << "|" << line << "|" << std::endl;
                line.clear( );
            }
            out << std::setw( 8 ) << std::setfill( '0' ) << std::hex << char_idx << "  ";
        }
        else if ( ( char_idx % ( line_size / 2 ) ) == 0 )
        {
            out << " ";
        }
        serializer( out, character );
        out << " ";
        line += ( isprint( character ) ? character : ' ' );
    }

    if ( contents.size( ) % line_size != 0 )
    {
        for ( size_t i = 0; i < ( line_size - contents.size( ) % line_size ); ++i )
        {
            for ( size_t j = 0; j < element_width; ++j )
            {
                out << " ";
            }
            out << " ";
        }
    }
    out << "|" << line << "|" << std::endl << std::endl;
}
}  // namespace helpers

inline bool
MemoryResourceStorage::exists( const char* key )
{
    auto path = get_path( key );
    std::lock_guard< std::mutex > lock( m_storage_mutex );
    return m_storage->resources.count( path ) != 0 || m_storage->streams.count( path ) != 0
           || helpers::prefix_exists( m_storage->resources, path + "/" )
           || helpers::prefix_exists( m_storage->streams, path + "/" );
}

template < typename ResourceSerializer >
std::string
MemoryResourceStorage::dump_resources( bool dump_schemas, ResourceSerializer&& f ) const
{
    auto is_schema = []( const std::string& key ) {
        return key.size( ) > 7 && key.substr( key.size( ) - 7 ) == ".schema";
    };

    std::ostringstream ss;
    std::lock_guard< std::mutex > lock( m_storage_mutex );
    for ( auto& resource : m_storage->resources )
    {
        if ( !is_schema( resource.first ) || dump_schemas )
        {
            f( resource.first, *resource.second, ss );
        }
    }
    for ( auto& resource : m_storage->streams )
    {
        if ( m_storage->resources.find( resource.first ) == m_storage->resources.end( )
             && ( !is_schema( resource.first ) || dump_schemas ) )
        {
            f( resource.first, resource.second->str( ), ss );
        }
    }
    return ss.str( );
}

inline std::string
MemoryResourceStorage::bindump( bool dump_schemas ) const
{
    auto dump_binary
        = []( const std::string& key, const std::string& contents, std::ostream& out ) {
              helpers::dump_resource(
                  key, contents, 4,
                  []( std::ostream& out, uint8_t ch ) { out << std::bitset< 8 >( ch ); }, 8, out );
          };
    return dump_resources( dump_schemas, dump_binary );
}

inline std::string
MemoryResourceStorage::hexdump( bool dump_schemas ) const
{
    auto dump_hex = []( const std::string& key, const std::string& contents, std::ostream& out ) {
        helpers::dump_resource(
            key, contents, 16,
            []( std::ostream& out, uint8_t ch ) {
                out << std::setw( 2 ) << std::setfill( '0' ) << std::hex
                    << static_cast< uint32_t >( ch );
            },
            2, out );
    };
    return dump_resources( dump_schemas, dump_hex );
}

}  // namespace flatdata
