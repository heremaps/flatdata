/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/Archive.h>
#include <flatdata/internal/ArchiveUtils.h>

#include <cstring>

namespace flatdata
{
constexpr size_t TAB_WIDTH = 4;

Archive::Archive( std::shared_ptr< flatdata::ResourceStorage > storage )
    : m_storage( std::move( storage ) )
{
}

Archive::operator bool( ) const
{
    return is_open( );
}

bool
Archive::is_open( ) const
{
    return m_is_open;
}

bool
Archive::initialize( )
{
    if ( !m_storage )
    {
        return false;
    }

    auto signature = storage( ).read< flatdata::MemoryDescriptor >(
        internal::signature_name( name( ) ).c_str( ), schema( ) );
    if ( signature )
    {
        m_signature = *signature;
    }
    m_is_open = load_contents( ) && static_cast< bool >( signature );
    return m_is_open;
}

flatdata::ResourceStorage&
Archive::storage( )
{
    return *m_storage;
}

const flatdata::ResourceStorage&
Archive::storage( ) const
{
    return *m_storage;
}

namespace
{
std::vector< std::string >
to_lines( const char* string )
{
    std::vector< std::string > result;
    if ( string )
    {
        while ( *string )
        {
            auto begin = string;
            while ( *string && *string != '\n' )
            {
                string++;
            }
            result.emplace_back( begin, string );
            if ( *string )
            {
                string++;
            }
        }
    }

    return result;
}

std::string
create_context( const std::vector< std::string >& diff )
{
    std::string result;
    std::vector< bool > result_mask( diff.size( ) );
    size_t context_lines = 3;

    int lines_to_go = -1;
    for ( size_t i = diff.size( ); i > 0; i--, lines_to_go-- )
    {
        if ( diff[ i - 1 ][ 0 ] != ' ' )
        {
            lines_to_go = context_lines;
        }

        result_mask[ i ] = lines_to_go >= 0;
    }
    lines_to_go = -1;
    for ( size_t i = 0; i < diff.size( ); i++, lines_to_go-- )
    {
        if ( diff[ i ][ 0 ] != ' ' )
        {
            lines_to_go = context_lines;
        }

        result_mask[ i ] = result_mask[ i ] || ( lines_to_go >= 0 );
    }
    bool printed_last = false;
    for ( size_t i = diff.size( ); i > 0; i-- )
    {
        if ( result_mask[ i - 1 ] )
        {
            result += diff[ i - 1 ] + '\n';
            printed_last = true;
        }
        else if ( printed_last )
        {
            result += "...\n";
            printed_last = false;
        }
    }
    return result;
}

std::string
compute_diff( const char* expected, const char* found )
{
    if ( !found )
    {
        return {};
    }
    std::vector< std::string > lines_expected = to_lines( expected );
    std::vector< std::string > lines_found = to_lines( found );
    size_t expected_length = lines_expected.size( ) + 1;
    size_t found_length = lines_found.size( ) + 1;

    std::vector< size_t > distances;

    distances.resize( expected_length * found_length );

    auto entry = [&]( size_t expected_pos, size_t found_pos ) -> size_t& {
        assert( expected_pos < expected_length );
        assert( found_pos < found_length );
        return distances[ expected_pos + found_pos * expected_length ];
    };

    for ( size_t i = 0; i < expected_length; i++ )
    {
        entry( i, 0 ) = i;
    }
    for ( size_t i = 0; i < found_length; i++ )
    {
        entry( 0, i ) = i;
    }

    // fill in distance table
    for ( size_t found_pos = 0; found_pos < lines_found.size( ); found_pos++ )
    {
        for ( size_t expected_pos = 0; expected_pos < lines_expected.size( ); expected_pos++ )
        {
            size_t min_cost = std::min( entry( expected_pos, found_pos + 1 ) + 1,
                                        entry( expected_pos + 1, found_pos ) + 1 );

            if ( lines_expected[ expected_pos ] == lines_found[ found_pos ] )
            {
                min_cost = std::min( min_cost, entry( expected_pos, found_pos ) );
            }

            entry( expected_pos + 1, found_pos + 1 ) = min_cost;
        }
    }

    std::vector< std::string > diff;
    size_t found_pos = lines_found.size( );
    size_t expected_pos = lines_expected.size( );
    while ( found_pos != 0 || expected_pos != 0 )
    {
        size_t next_found_pos = found_pos;
        size_t next_expected_pos = expected_pos;
        auto check = [&]( size_t new_expected_pos, size_t new_found_pos, size_t transition_cost ) {
            size_t cost = entry( new_expected_pos, new_found_pos );
            if ( cost + transition_cost == entry( expected_pos, found_pos ) )
            {
                next_found_pos = new_found_pos;
                next_expected_pos = new_expected_pos;
            }
        };
        if ( found_pos != 0 )
        {
            check( expected_pos, found_pos - 1, 1 );
        }
        if ( expected_pos != 0 )
        {
            check( expected_pos - 1, found_pos, 1 );
        }
        if ( expected_pos != 0 && found_pos != 0
             && lines_expected[ expected_pos - 1 ] == lines_found[ found_pos - 1 ] )
        {
            check( expected_pos - 1, found_pos - 1, 0 );
        }
        assert( expected_pos != next_expected_pos || found_pos != next_found_pos );
        if ( next_found_pos == found_pos )
        {
            diff.emplace_back( std::string( "-" ) + "\"" + lines_expected[ next_expected_pos ]
                               + "\"" );
        }
        else if ( next_expected_pos == expected_pos )
        {
            diff.emplace_back( std::string( "+" ) + "\"" + lines_found[ next_found_pos ] + "\"" );
        }
        else
        {
            diff.emplace_back( std::string( " " ) + "\"" + lines_expected[ next_expected_pos ]
                               + "\"" );
        }
        found_pos = next_found_pos;
        expected_pos = next_expected_pos;
    }

    return create_context( diff );
}
}

std::string
Archive::describe( size_t nest_level ) const
{
    const auto newl = std::string( "\n" );
    const auto hline = std::string( 80, '=' ) + newl;
    const auto empty = std::string( "" );
    const bool is_root_node = ( nest_level == 0 );

    std::ostringstream result;

    if ( !m_storage )
    {
        if ( is_root_node )
        {
            result << hline << "FATAL: Resource storage not initialized. Please check archive path."
                   << newl;
        }
        else
        {
            result << "Uninitialized Archive " << name( );
        }
    }

    if ( m_storage && !m_signature )
    {
        result << ( is_root_node ? hline : empty )
               << "FATAL: Archive signature does not match software expectations."
               << ( is_root_node ? newl : empty ) << hline;
        result << compute_diff(
            schema( ),
            m_storage->read_schema( internal::signature_name( name( ) ).c_str( ) ).char_ptr( ) );
    }

    if ( m_storage && !m_is_open )
    {
        // Error propagated to root and storage is not initialized in respective child. No root
        // check needed.
        result << hline
               << "FATAL: Archive initialization failed. Failed loading mandatory resources."
               << std::endl;
    }

    if ( is_root_node )
    {
        result << hline << "Flatdata Archive: " << name( ) << std::endl
               << hline
               << "Resource                             Optional  Too Large  Loaded    Details"
               << std::endl
               << hline;
    }
    else
    {
        const std::string indent( ( nest_level - 1 ) * TAB_WIDTH, ' ' );
        result << newl + indent + std::string( "|" ) + newl + indent + std::string( "|->" )
               << " Flatdata Archive: " << name( ) << std::endl;
    }

    describe_resources( result, nest_level );

    if ( is_root_node )
    {
        result << hline;
    }

    return result.str( );
}

void
Archive::describe_impl( std::ostream& stream,
                        const char* name,
                        bool optional,
                        bool loaded,
                        const char* details,
                        bool has_nested_details,
                        bool too_large,
                        size_t nest_level )
{
    const std::string indent( nest_level * TAB_WIDTH, ' ' );
    size_t offset = indent.size( );
    stream << indent << std::left << std::setw( 37 - offset ) << std::setfill( ' ' )
           << std::string( name ).substr( 0, 30 ) << std::left << std::setw( 10 )
           << std::setfill( ' ' ) << ( optional ? "YES" : "NO" ) << std::left << std::setw( 11 )
           << std::setfill( ' ' ) << ( too_large ? "YES" : "NO" ) << std::left << std::setw( 10 )
           << std::setfill( ' ' ) << ( static_cast< bool >( loaded ) ? "YES" : "NO" ) << details
           << ( has_nested_details ? "" : "\n" );
}

}  // namespace flatdata
