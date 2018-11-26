#pragma once

#include <iostream>
#include "GraphGenerator.h"

template < typename Graph >
void
lookup( const Graph& graph, uint32_t num_iterations )
{
    uint32_t num_buckets = sqrt( graph.node_count( ) );
    std::mt19937 gen( 1337 );
    std::uniform_int_distribution< uint32_t > dis( 0, num_buckets - 1 );
    for ( uint32_t i = 0; i < num_iterations; i++ )
    {
        std::vector< std::vector< std::pair< InternalId, ExternalId > > > buckets( num_buckets );

        for ( uint32_t edge_index = 0; edge_index < graph.edge_count( ); edge_index++ )
        {
            InternalId id{edge_index};
            auto edge = graph.edge( id );
            buckets[ dis( gen ) ].emplace_back( id, edge.external_id( ) );
        }
        uint32_t num_not_found = 0;
        for ( auto& bucket : buckets )
        {
            for ( auto& ids : bucket )
            {
                auto found = graph.find( ids.second );
                if ( found.id( ).id != ids.first.id )
                {
                    num_not_found++;
                }
            }
        }
        std::cout << "Not found: " << num_not_found << std::endl;
    }
}