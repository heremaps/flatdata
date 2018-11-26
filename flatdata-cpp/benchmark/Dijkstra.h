#pragma once

#include "GraphGenerator.h"

#include <iostream>
#include <queue>
#include <random>
#include <vector>

template < typename Edge >
uint32_t
cost_function( Edge edge, Direction dir )
{
    // A very simple (arbitrary) cost function that uses each attribute to benchmark reading all the
    // data;
    uint32_t multiplier[ 5 ] = {20, 19, 18, 17, 16};
    uint32_t divident = 200 * edge.speed_km_h( dir );
    uint32_t virtual_cost = 0;
    virtual_cost += edge.is_a( ) * 1;
    virtual_cost += edge.is_b( ) * 2;
    virtual_cost += edge.is_c( ) * 1;
    virtual_cost += edge.is_d( ) * 3;
    virtual_cost += edge.is_e( dir ) * 4;
    virtual_cost += edge.is_f( dir ) * 1;
    virtual_cost += edge.is_g( dir ) * 2;

    return ( static_cast< uint64_t >( edge.length_m( ) ) * 36 * multiplier[ edge.frc( ) ]
             + divident / 2 )
               / divident
           + virtual_cost;
}

struct PqNode
{
    PqNode( uint32_t cost, uint32_t id )
        : cost( cost )
        , id( id )
    {
    }

    uint32_t cost;
    uint32_t id;  // node id + direction

    bool
    operator<( const PqNode& other ) const
    {
        // std::priority_queue is a max_heap, we need to invert operator<
        return cost > other.cost;
    }
};

/// Implements a simple Dijkstra shortest path finding algorithm, computing distance between edges
/// (not nodes). For big graphs this can mean a lot of random memory access.
template < typename Graph >
void
dijkstra( const Graph& graph, uint32_t num_iterations )
{
    std::mt19937 gen( 1337 );
    std::uniform_int_distribution< uint32_t > dis( 0, graph.edge_count( ) * 2 - 1 );
    for ( uint32_t i = 0; i < num_iterations; i++ )
    {
        // sue a very simple PQ to avoid testing good PQ implementations instead of flatdata
        std::vector< PqNode > heap;
        heap.reserve( graph.edge_count( ) * 2 );
        std::priority_queue< PqNode > pq( std::less< PqNode >( ), std::move( heap ) );
        std::vector< uint32_t > cost( graph.edge_count( ) * 2,
                                      std::numeric_limits< uint32_t >::max( ) );

        uint32_t start = dis( gen );
        uint64_t sum_cost = 0;
        uint32_t num_nodes = 0;
        pq.emplace( dis( gen ), 0 );
        while ( !pq.empty( ) )
        {
            PqNode node = pq.top( );
            pq.pop( );
            if ( node.cost != cost[ node.id ] )
            {
                // we are using a simple priority_queue without the ability to update cost of
                // items. Thus we have each item multiple times in the queue. Since we do only
                // want to handle the best cost we skip all other.
            }
            num_nodes++;
            sum_cost += node.cost;
            auto edge = graph.edge( InternalId{node.id >> 1} );
            auto edges
                = ( node.id & 1 ) == 0 ? edge.to_node( ).edges( ) : edge.from_node( ).edges( );
            for ( ; edges.valid( ); edges++ )
            {
                auto data = edges.data( );
                uint32_t target_id
                    = ( data.id( ).id << 1 ) | ( edges.dir( ) == Direction::POSITIVE ? 0 : 1 );
                uint32_t target_cost = cost_function( data, edges.dir( ) ) + node.cost;
                if ( target_cost < cost[ target_id ] )
                {
                    cost[ target_id ] = target_cost;
                    // This can either insert a new target, or another (better) copy for an already
                    // inserted target
                    pq.emplace( target_id, target_cost );
                }
            }
        }
        std::cout << "Start " << ( ( ( start & 1 ) == 0 ) ? "+" : "-" ) << ( start >> 1 )
                  << " SumCosts: " << sum_cost
                  << " Queue Size: " << num_nodes << std::endl;
    }
}
