/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "coappearances.hpp"  // generated by flatdata from coappearances.flatdata

#include "picojson.h"
#define DOCOPT_HEADER_ONLY
#include "docopt/docopt.h"

#include <flatdata/flatdata.h>
#include <fstream>
#include <iostream>
#include <string>

namespace co = coappearances;

void
convert_meta( const picojson::object& obj, co::GraphBuilder builder, std::string& strings )
{
    flatdata::Struct< co::Meta > data;
    auto meta = *data;
    meta.title_ref = strings.size( );
    strings += obj.at( "title" ).get< std::string >( ) + '\0';
    meta.author_ref = strings.size( );
    strings += obj.at( "author" ).get< std::string >( ) + '\0';
    builder.set_meta( meta );
}

using CharactersIndex
    = std::map< std::string /* character id */, uint16_t /* position in Graph::vertices */ >;

CharactersIndex
build_characters_index( const picojson::object& characters )
{
    CharactersIndex index;
    if ( std::numeric_limits< uint16_t >::max( ) <= characters.size( ) )
    {
        throw std::runtime_error( "Too many characters for indexing by 16 bits" );
    }

    uint16_t ref = 0;
    for ( const auto& kv : characters )
    {
        auto id = kv.first;
        if ( index.count( id ) )
        {
            throw std::runtime_error( std::string( "Duplicate index " ) + kv.first );
        }
        index.emplace( id, ref );
        ref++;
    }
    return index;
}

std::pair< uint8_t /* major */, uint8_t /* minor */ >
convert_chapter( const std::string& s )
{
    auto pos = s.find( '.' );
    if ( pos == std::string::npos )
    {
        throw std::runtime_error( std::string( "Unexpected chapter format for " ) + s );
    }

    auto major = std::stoul( s.substr( 0, pos ) );
    if ( ( 1 << co::Chapter::MajorType::bit_width ) <= major )
    {
        throw std::runtime_error( "Major chapter overflow" );
    }

    auto minor = std::stoul( s.substr( pos + 1 ) );
    if ( ( 1 << co::Chapter::MinorType::bit_width ) <= minor )
    {
        throw std::runtime_error( "Major chapter overflow" );
    }

    return {static_cast< uint8_t >( major ), static_cast< uint8_t >( minor )};
}

void
convert_characters( const picojson::object& characters,
                    const CharactersIndex& characters_index,
                    co::GraphBuilder builder,
                    std::string& strings )
{
    auto vertices = builder.start_vertices( );
    auto vertices_data = builder.start_vertices_data( );

    for ( const auto& kv : characters )
    {
        const auto& data = kv.second.get< picojson::object >( );

        auto character = vertices.grow( );
        auto character_data = vertices_data.grow( );

        character.name_ref = strings.size( );
        strings += data.at( "name" ).get< std::string >( ) + '\0';

        if ( data.count( "nickname" ) )
        {
            auto nickname = character_data.add< co::Nickname >( );
            nickname.ref = strings.size( );
            strings += data.at( "nickname" ).get< std::string >( ) + '\0';
        }

        if ( data.count( "description" ) )
        {
            auto description = character_data.add< co::Description >( );
            description.ref = strings.size( );
            strings += data.at( "description" ).get< std::string >( ) + '\0';
        }

        if ( data.count( "relation" ) )
        {
            const auto& relation = data.at( "relation" ).get< picojson::object >( );
            if ( relation.at( "to" ).is< std::string >( ) )
            {
                auto rel = character_data.add< co::UnaryRelation >( );
                rel.kind_ref = strings.size( );
                strings += relation.at( "kind" ).get< std::string >( ) + '\0';
                rel.to_ref = characters_index.at( relation.at( "to" ).get< std::string >( ) );
            }
            else
            {
                auto rel = character_data.add< co::BinaryRelation >( );
                rel.kind_ref = strings.size( );
                strings += relation.at( "kind" ).get< std::string >( ) + '\0';
                auto to_refs = relation.at( "to" ).get< picojson::array >( );
                assert( to_refs.size( ) == 2 );
                rel.to_a_ref = characters_index.at( to_refs[ 0 ].get< std::string >( ) );
                rel.to_b_ref = characters_index.at( to_refs[ 1 ].get< std::string >( ) );
            }
        }
    }

    vertices.close( );
    vertices_data.close( );
}

void
convert_coappearances( const picojson::array& coappearances,
                       const CharactersIndex& characters_index,
                       co::GraphBuilder builder )
{
    auto edges = builder.start_edges( );
    auto chapters = builder.start_chapters( );
    for ( const auto& object : coappearances )
    {
        const auto& data = object.get< picojson::object >( );

        auto coappearance = edges.grow( );
        coappearance.a_ref = characters_index.at( data.at( "a" ).get< std::string >( ) );
        coappearance.b_ref = characters_index.at( data.at( "b" ).get< std::string >( ) );
        const auto& data_chapters = data.at( "chapters" ).get< picojson::array >( );
        coappearance.count = data_chapters.size( );
        coappearance.first_chapter_ref = chapters.size( );

        for ( const auto& data_chapter : data_chapters )
        {
            const auto& major_minor = convert_chapter( data_chapter.get< std::string >( ) );
            auto chapter = chapters.grow( );
            chapter.major = major_minor.first;
            chapter.minor = major_minor.second;
        }
    }

    // add sentinel to edges for easier access of chapters range
    auto sentinel = edges.grow( );
    sentinel.a_ref = std::numeric_limits< uint16_t >::max( );
    sentinel.b_ref = std::numeric_limits< uint16_t >::max( );
    sentinel.first_chapter_ref = chapters.size( );

    edges.close( );
    chapters.close( );
}

void
convert( const char* json_path, const char* archive_path )
{
    // parse json
    std::ifstream json_file( json_path );
    if ( !json_file.is_open( ) )
    {
        throw std::runtime_error( std::string( "Could not open " ) + json_path );
    }
    picojson::value json;
    json_file >> json;
    auto err = picojson::get_last_error( );
    if ( !err.empty( ) )
    {
        throw std::runtime_error( err );
    }
    const auto& root = json.get< picojson::object >( );

    // create new flatdata archive
    auto storage = flatdata::FileResourceStorage::create( archive_path );
    if ( !storage )
    {
        throw std::runtime_error( std::string( "Could not initialize storage at: " )
                                  + archive_path );
    }
    auto builder = co::GraphBuilder::open( std::move( storage ) );

    // container holding a list of zero-terminated strings
    std::string strings;

    // convert and serialize
    const auto& meta = root.at( "meta" ).get< picojson::object >( );
    convert_meta( meta, builder, strings );
    const auto& characters = root.at( "characters" ).get< picojson::object >( );
    const auto& characters_index = build_characters_index( characters );
    convert_characters( characters, characters_index, builder, strings );
    const auto& coappearances = root.at( "coappearances" ).get< picojson::array >( );
    convert_coappearances( coappearances, characters_index, builder );
    builder.set_strings( {strings.data( ), strings.size( )} );
}

// Note: The linear search below is slow for big graphs for obvious reasons, where it is much better
// to build a lookup table. However, for our small graph this works fine.
std::vector< uint32_t >
get_neighbors( co::Graph graph, uint32_t vertex_ref )
{
    std::vector< uint32_t > neighbors;
    for ( auto e : graph.edges( ) )
    {
        if ( e.a_ref == vertex_ref )
        {
            neighbors.push_back( e.b_ref );
        }
        else if ( e.b_ref == vertex_ref )
        {
            neighbors.push_back( e.a_ref );
        }
    }
    return neighbors;
}

size_t
calculate_num_connected_components( co::Graph graph )
{
    size_t num_connected_components = 0;

    std::vector< bool > seen( graph.vertices( ).size( ) );
    std::vector< uint32_t > stack;

    for ( uint32_t vertex_ref = 0; vertex_ref < graph.vertices( ).size( ); ++vertex_ref )
    {
        // skip already seen vertices
        if ( seen[ vertex_ref ] )
        {
            continue;
        }

        stack.clear( );
        stack.push_back( vertex_ref );
        seen[ vertex_ref ] = true;

        // depth first search
        while ( !stack.empty( ) )
        {
            uint32_t vertex_ref = stack.back( );
            stack.pop_back( );
            for ( auto neighbor_ref : get_neighbors( graph, vertex_ref ) )
            {
                if ( !seen[ neighbor_ref ] )
                {
                    stack.push_back( neighbor_ref );
                    seen[ neighbor_ref ] = true;
                }
            }
        }
        num_connected_components++;
    }
    return num_connected_components;
}

void
calculate( const char* archive_path )
{
    std::shared_ptr< flatdata::FileResourceStorage > storage
        = flatdata::FileResourceStorage::create( archive_path );
    auto graph = co::Graph::open( storage );
    if ( !graph )
    {
        std::cerr << graph.describe( ) << std::endl;
        throw std::runtime_error( std::string( "Could not open graph at: " ) + archive_path );
    }
    auto builder = co::GraphBuilder::open( storage ).statistics( );

    flatdata::Vector< co::Degree > vertex_degrees( graph.vertices( ).size( ) );
    for ( auto e : graph.edges( ) )
    {
        vertex_degrees[ e.a_ref ].value = vertex_degrees[ e.a_ref ].value + 1;
        vertex_degrees[ e.b_ref ].value = vertex_degrees[ e.b_ref ].value + 1;
    }
    builder.set_vertex_degrees( vertex_degrees );

    flatdata::ArrayView< co::Degree > degrees_view = vertex_degrees;
    auto minmax = std::minmax_element( degrees_view.begin( ), degrees_view.end( ) );

    flatdata::Struct< co::Invariants > data;
    auto inv = *data;
    inv.min_degree = minmax.first->value;
    inv.min_degree_ref = std::distance( degrees_view.begin( ), minmax.first );
    inv.max_degree = minmax.second->value;
    inv.max_degree_ref = std::distance( degrees_view.begin( ), minmax.second );
    inv.num_connected_components = calculate_num_connected_components( graph );
    builder.set_invariants( inv );
}

void
read( const char* archive_path )
{
    auto storage = flatdata::FileResourceStorage::create( archive_path );
    auto graph = co::Graph::open( std::move( storage ) );
    if ( !graph )
    {
        std::cerr << graph.describe( ) << std::endl;
        throw std::runtime_error( std::string( "Could not open graph at: " ) + archive_path );
    }

    const char* strings = graph.strings( ).char_ptr( );

    std::cout << "Meta:" << std::endl
              << "  Title: " << strings + graph.meta( ).title_ref << std::endl
              << "  Author: " << strings + graph.meta( ).author_ref << std::endl
              << std::endl;

    auto vertices = graph.vertices( );
    std::cout << "Characters (" << vertices.size( ) << "):" << std::endl;
    for ( uint32_t vertex_ref = 0; vertex_ref < vertices.size( ); ++vertex_ref )
    {
        auto vertex = vertices[ vertex_ref ];
        std::cout << strings + vertex.name_ref;
        auto visitor = flatdata::make_overload(
            [strings]( co::Nickname nickname ) {
                std::cout << " (" << strings + nickname.ref << ")";
            },
            [strings]( co::Description desciption ) {
                std::cout << ": Description: " << strings + desciption.ref;
            },
            [strings, vertices]( co::UnaryRelation relation ) {
                std::cout << ": Relation(kind=" << strings + relation.kind_ref
                          << ", to=" << strings + vertices[ relation.to_ref ].name_ref << ")";
            },
            [strings, vertices]( co::BinaryRelation relation ) {
                std::cout << ": Relation(kind=" << strings + relation.kind_ref
                          << ", to=" << strings + vertices[ relation.to_a_ref ].name_ref
                          << ", to=" << strings + vertices[ relation.to_b_ref ].name_ref << ")";
            } );
        graph.vertices_data( ).for_each( vertex_ref, visitor );
        std::cout << std::endl;
    }
    std::cout << std::endl;

    auto edges = graph.edges( );
    std::cout << "Coappearances (" << edges.size( ) << "):" << std::endl;
    // Skip the last edge since it is a sentinel
    for ( uint32_t edge_ref = 0; edge_ref + 1 < edges.size( ); ++edge_ref )
    {
        auto edge = edges[ edge_ref ];
        std::cout << strings + vertices[ edge.a_ref ].name_ref << " meets "
                  << strings + vertices[ edge.b_ref ].name_ref << " " << edge.count
                  << " time(s) in chapters ";
        // The end of the chapters assigned to this edge is the first chapter from the next edge.
        // This is a typical trick when storing ranges. That's why a sentinel was added to edges.
        uint32_t next_edge_ref = edge_ref + 1;
        uint32_t chapters_begin = edge.first_chapter_ref;
        uint32_t chapters_size = edges[ next_edge_ref ].first_chapter_ref - chapters_begin;
        auto chapters = graph.chapters( ).slice( chapters_begin, chapters_size );
        for ( size_t chapter_ref = 0; chapter_ref < chapters.size( ); ++chapter_ref )
        {
            auto chapter = graph.chapters( )[ chapter_ref ];
            std::cout << chapter.major.as< uint32_t >( ) << "." << chapter.minor.as< uint32_t >( )
                      << ( chapter_ref + 1 < chapters.size( ) ? ", " : "." );
        }
        std::cout << std::endl;
    }

    auto statistics = graph.statistics( );
    if ( statistics )
    {
        std::cout << std::endl;
        std::cout << "Statistics: " << std::endl;

        auto inv = statistics->invariants( );
        std::cout << "max_degree: " << inv.max_degree << " ("
                  << strings + graph.vertices( )[ inv.max_degree_ref ].name_ref << ")" << std::endl;
        std::cout << "min_degree: " << inv.min_degree << " ("
                  << strings + graph.vertices( )[ inv.min_degree_ref ].name_ref << ")" << std::endl;
        std::cout << "num_connected_components: " << inv.num_connected_components << std::endl;

        std::cout << "Vertex degrees: " << std::endl;
        for ( auto degree : statistics->vertex_degrees( ) )
        {
            std::cout << degree.value << " ";
        }
        std::cout << std::endl;
    }
}

static const char* USAGE = 1 + R"_(
Coappearances graph example.

Usage:
  coappearances convert <input.json> <output_folder>
  coappearances calculate <output_folder>
  coappearances read <output_folder>
  coappearances -h | --help

Commands:
  convert       Convert example json into flatdata archive.
  calculate     Calculate invariants and other graph data from converted archive,
                and store the results in a subarchive embedded into the archive.
  read          Read and print out the data.
)_";

int
main( int argc, char const* argv[] )
{
    auto args = docopt::docopt( USAGE, {argv + 1, argv + argc} );
    try
    {
        const auto& output_folder = args.at( "<output_folder>" ).asString( );
        if ( args.at( "convert" ).asBool( ) )
        {
            convert( args.at( "<input.json>" ).asString( ).c_str( ), output_folder.c_str( ) );
        }
        else if ( args.at( "calculate" ).asBool( ) )
        {
            calculate( output_folder.c_str( ) );
        }
        else
        {
            read( output_folder.c_str( ) );
        }
    }
    catch ( const std::runtime_error& err )
    {
        std::cerr << "Error: " << err.what( ) << std::endl;
    }

    return 0;
}
