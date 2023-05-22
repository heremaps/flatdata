#pragma once

#include <iostream>
#include "GraphGenerator.h"

/// Prints the contents of a graph to the console
template < typename Graph >
void
print_graph( const Graph& graph )
{
    using namespace std;
    cout << "Nodes: " << graph.node_count( ) << endl;
    cout << "Edges: " << graph.edge_count( ) << endl;

    auto print_edge = []( typename Graph::Edge data ) {
        cout << data.from_node( ).id( ).id << "->" << data.to_node( ).id( ).id << ": "
             << data.id( ).id << "[" << hex << data.external_id( ).id << dec
             << "]: " << data.length_m( ) << "m, " << data.is_a( ) << data.is_b( ) << data.is_c( )
             << data.is_d( ) << "[+" << data.is_e( Direction::POSITIVE )
             << data.is_f( Direction::POSITIVE ) << data.is_g( Direction::POSITIVE ) << "][-"
             << data.is_e( Direction::NEGATIVE ) << data.is_f( Direction::NEGATIVE )
             << data.is_g( Direction::NEGATIVE ) << "], "
             << (uint32_t)data.speed_km_h( Direction::POSITIVE ) << "/"
             << (uint32_t)data.speed_km_h( Direction::NEGATIVE ) << "km_h, ";
        cout << endl;
    };

    for ( auto nodes = graph.nodes( ); nodes.valid( ); nodes++ )
    {
        cout << nodes.id( ).id << ": Pos (" << nodes.coordinates( ).x << ", "
             << nodes.coordinates( ).y << ")" << endl;
        for ( auto edges = nodes.edges( ); edges.valid( ); edges++ )
        {
            cout << "   " << ( edges.dir( ) == Direction::POSITIVE ? "+ " : "- " );
            print_edge( edges.data( ) );
        }
    }

    for ( uint32_t i = 0; i < graph.edge_count( ); i++ )
    {
        print_edge( graph.edge( InternalId{ i } ) );
    }
}
