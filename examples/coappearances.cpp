/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "coappearances.hpp"  // generated by flatdata from coappearances.flatdata
#include "picojson.h"

#include <flatdata/flatdata.h>
#include <fstream>
#include <iostream>
#include <string>

namespace co = coappearances;

void
convert_meta( const picojson::object& obj, co::GraphBuilder builder, std::string& strings )
{
    // Since flatdata's mutators are not holding any data, we are creating a vector with a single
    // element for holding the data.
    flatdata::Vector< co::Meta > data( 1 );
    auto meta = data[ 0 ];
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
        const auto& id = kv.first;
        const auto& data = kv.second.get< picojson::object >( );

        auto character = vertices.grow( );
        character.name_ref = strings.size( );
        strings += data.at( "name" ).get< std::string >( ) + '\0';

        if ( data.count( "nickname" ) )
        {
            auto nickname = vertices_data.add_to_current_item< co::Nickname >( );
            nickname.ref = strings.size( );
            strings += data.at( "nickname" ).get< std::string >( ) + '\0';
        }

        if ( data.count( "description" ) )
        {
            auto description = vertices_data.add_to_current_item< co::Description >( );
            description.ref = strings.size( );
            strings += data.at( "description" ).get< std::string >( ) + '\0';
        }

        if ( data.count( "relation" ) )
        {
            const auto& relation = data.at( "relation" ).get< picojson::object >( );
            if ( relation.at( "to" ).is< std::string >( ) )
            {
                auto rel = vertices_data.add_to_current_item< co::UnaryRelation >( );
                rel.kind_ref = strings.size( );
                strings += relation.at( "kind" ).get< std::string >( ) + '\0';
                rel.to_ref = characters_index.at( relation.at( "to" ).get< std::string >( ) );
            }
            else
            {
                auto rel = vertices_data.add_to_current_item< co::BinaryRelation >( );
                rel.kind_ref = strings.size( );
                strings += relation.at( "kind" ).get< std::string >( ) + '\0';
                auto to_refs = relation.at( "to" ).get< picojson::array >( );
                assert( to_refs.size( ) == 2 );
                rel.to_a_ref = characters_index.at( to_refs[ 0 ].get< std::string >( ) );
                rel.to_a_ref = characters_index.at( to_refs[ 1 ].get< std::string >( ) );
            }
        }

        vertices_data.next_item( );
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

void
read( const char* archive_path )
{
    auto storage = flatdata::FileResourceStorage::create( archive_path );
    auto graph = co::Graph::open( std::move( storage ) );

    const char* strings = graph.strings( ).char_ptr( );

    std::cout << "Meta:" << std::endl
              << "  Title: " << strings + graph.meta( ).title_ref << std::endl
              << "  Author: " << strings + graph.meta( ).author_ref << std::endl
              << std::endl;

    std::cout << "Characters:" << std::endl;
    auto vertices = graph.vertices( );
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

    std::cout << "Edges: " << std::endl;
    auto edges = graph.edges( );
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
}

static const char* USAGE = 1 + R"_(
Usage:
  coappearances convert <input.json> <output_folder>
  coappearances read <output_folder>)_";

int
main( int argc, char const* argv[] )
{
    if ( argc < 3 )
    {
        std::cerr << USAGE << std::endl;
        return 1;
    }

    std::string verb( argv[ 1 ] );
    try
    {
        if ( verb == "convert" && argc == 4 )
        {
            convert( argv[ 2 ], argv[ 3 ] );
        }
        else if ( verb == "read" && argc == 3 )
        {
            read( argv[ 2 ] );
        }
        else
        {
            std::cerr << USAGE << std::endl;
            return 1;
        }
    }
    catch ( const std::runtime_error& err )
    {
        std::cerr << "Error: " << err.what( ) << std::endl;
    }

    return 0;
}