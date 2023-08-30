#pragma once

#include "GraphGenerator.h"

#include <iostream>
#include <random>
#include <vector>

/// Computes a simple Breath-First-Search. For big graphs this can mean a lot of random memory
/// access.
template < typename Graph >
void
bfs( const Graph& graph, uint32_t num_iterations )
{
    std::mt19937 gen( 1337 );
    std::uniform_int_distribution< uint32_t > dis( 0, graph.edge_count( ) * 2 - 1 );
    for ( uint32_t i = 0; i < num_iterations; i++ )
    {
        std::vector< uint32_t > depths( graph.edge_count( ) * 2,
                                        std::numeric_limits< uint32_t >::max( ) );
        std::vector< uint32_t > q;
        q.push_back( dis( gen ) );
        depths[ q.front( ) ] = 0;
        for ( size_t q_pos = 0; q_pos < q.size( ); q_pos++ )
        {
            uint32_t id = q[ q_pos ];
            uint32_t depth = depths[ id ];
            auto edge = graph.edge( InternalId{ id >> 1 } );
            auto edges
                = ( q[ q_pos ] & 1 ) == 0 ? edge.to_node( ).edges( ) : edge.from_node( ).edges( );
            for ( ; edges.valid( ); edges++ )
            {
                auto data = edges.data( );
                uint32_t target_id
                    = ( data.id( ).id << 1 ) | ( edges.dir( ) == Direction::POSITIVE ? 0 : 1 );
                if ( depths[ target_id ] != std::numeric_limits< uint32_t >::max( ) )
                    continue;
                depths[ target_id ] = depth + 1;
                q.push_back( target_id );
            }
        }
        std::cout << "Start " << ( ( ( q.front( ) & 1 ) == 0 ) ? "+" : "-" ) << ( q.front( ) >> 1 )
                  << " MaxDepth: " << depths[ q.back( ) ] << " Queue Size: " << q.size( )
                  << std::endl;
    }
}
