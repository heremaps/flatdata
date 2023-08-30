#pragma once

#include "GraphGenerator.h"

#include "graph.hpp"

#include <algorithm>
#include <iostream>

#define RETURN_FOR_DIR( variable_prefix, dir ) \
    if ( dir == Direction::POSITIVE )          \
        return variable_prefix##_pos;          \
    return variable_prefix##_neg;

/// This class implements a graph on top of a flatdata archive
/// Data structure design is identical StructGraph, to enable measuring flatdata's performance
/// impact
class FlatdataGraph
{
public:
    class NodeRange;

    class Edge
    {
    public:
        InternalId
        id( ) const
        {
            return InternalId{ m_id };
        }
        ExternalId
        external_id( ) const
        {
            return ExternalId{ m_data.id };
        }
        uint32_t
        length_m( ) const
        {
            return m_data.length;
        }
        bool
        is_a( ) const
        {
            return m_data.is_a;
        }
        bool
        is_b( ) const
        {
            return m_data.is_b;
        }
        bool
        is_c( ) const
        {
            return m_data.is_c;
        }
        bool
        is_d( ) const
        {
            return m_data.is_d;
        }
        bool
        is_e( Direction dir ) const
        {
            RETURN_FOR_DIR( m_data.is_e, dir );
        }
        bool
        is_f( Direction dir ) const
        {
            RETURN_FOR_DIR( m_data.is_f, dir );
        }
        bool
        is_g( Direction dir ) const
        {
            RETURN_FOR_DIR( m_data.is_g, dir );
        }
        uint8_t
        frc( ) const
        {
            return m_data.frc;
        }
        uint8_t
        speed_km_h( Direction dir ) const
        {
            RETURN_FOR_DIR( m_data.speed, dir );
        }
        NodeRange from_node( ) const;
        NodeRange to_node( ) const;

    private:
        friend class FlatdataGraph;
        Edge( benchmark::EdgeData data, uint32_t id, const FlatdataGraph* graph )
            : m_data( data )
            , m_id( id )
            , m_graph( graph )
        {
        }
        benchmark::EdgeData m_data;
        uint32_t m_id;
        const FlatdataGraph* m_graph;
    };

    class EdgeRange
    {
    public:
        EdgeRange( )
            : m_current( )
            , m_end( )
            , m_graph( )
        {
        }
        void
        operator++( )
        {
            m_current++;
        }
        void
        operator++( int )
        {
            m_current++;
        }
        bool
        valid( ) const
        {
            return m_current < m_end;
        }
        size_t
        size( ) const
        {
            return m_end - m_current;
        }
        Direction
        dir( ) const
        {
            return m_graph->m_edges[ m_current ].dir.as< Direction >( );
        }
        Edge
        data( ) const
        {
            uint32_t id = m_graph->m_edges[ m_current ].id;
            return Edge( m_graph->m_data[ id ], id, m_graph );
        }

    private:
        friend class FlatdataGraph;
        EdgeRange( std::pair< uint32_t, uint32_t > range, const FlatdataGraph* graph )
            : m_current( range.first )
            , m_end( range.second )
            , m_graph( graph )
        {
        }
        uint32_t m_current;
        uint32_t m_end;
        const FlatdataGraph* m_graph;
    };

    class NodeRange
    {
    public:
        NodeRange( )
            : m_current( )
            , m_end( )
            , m_graph( )
        {
        }
        void
        operator++( )
        {
            m_current++;
        }
        void
        operator++( int )
        {
            m_current++;
        }
        bool
        valid( ) const
        {
            return m_current < m_end;
        }
        size_t
        size( ) const
        {
            return m_end - m_current;
        }
        NodeId
        id( ) const
        {
            return NodeId{ m_current };
        }
        Coordinates
        coordinates( ) const
        {
            return Coordinates{ m_graph->m_nodes[ m_current ].x, m_graph->m_nodes[ m_current ].y };
        }
        EdgeRange
        edges( ) const
        {
            return EdgeRange( m_graph->m_nodes[ m_current ].adjacent_edges, m_graph );
        }

    private:
        friend class FlatdataGraph;
        NodeRange( uint32_t current, uint32_t end, const FlatdataGraph* graph )
            : m_current( current )
            , m_end( end )
            , m_graph( graph )
        {
        }
        uint32_t m_current;
        uint32_t m_end;
        const FlatdataGraph* m_graph;
    };

public:
    FlatdataGraph( benchmark::Graph archive )
    {
        m_nodes = archive.nodes( );
        m_edges = archive.adjacent_edges( );
        m_data = archive.edge_data( );
    }

    template < typename Graph >
    static void
    create( const Graph& graph, benchmark::GraphBuilder& archive )
    {
        auto nodes = archive.start_nodes( );
        auto edges = archive.start_adjacent_edges( );
        auto edge_data = archive.start_edge_data( );

        for ( auto node = graph.nodes( ); node.valid( ); node++ )
        {
            auto output_node = nodes.grow( );
            output_node.x = node.coordinates( ).x;
            output_node.y = node.coordinates( ).y;
            output_node.first_adjacent_edge = edges.size( );
            for ( auto e = node.edges( ); e.valid( ); e++ )
            {
                auto output_edge = edges.grow( );
                output_edge.id = e.data( ).id( ).id;
                output_edge.dir = static_cast< uint32_t >( e.dir( ) );
            }
        }
        nodes.grow( ).first_adjacent_edge = edges.size( );  // sentinel

        for ( uint32_t i = 0; i < graph.edge_count( ); i++ )
        {
            auto edge = graph.edge( InternalId{ i } );
            auto output_data = edge_data.grow( );

            output_data.id = edge.external_id( ).id;
            output_data.speed_pos = edge.speed_km_h( Direction::POSITIVE );
            output_data.speed_neg = edge.speed_km_h( Direction::NEGATIVE );
            output_data.is_a = edge.is_a( );
            output_data.is_b = edge.is_b( );
            output_data.is_c = edge.is_c( );
            output_data.is_d = edge.is_d( );
            output_data.is_e_pos = edge.is_e( Direction::POSITIVE );
            output_data.is_e_neg = edge.is_e( Direction::NEGATIVE );
            output_data.is_f_pos = edge.is_f( Direction::POSITIVE );
            output_data.is_f_neg = edge.is_f( Direction::NEGATIVE );
            output_data.is_g_pos = edge.is_g( Direction::POSITIVE );
            output_data.is_g_neg = edge.is_g( Direction::NEGATIVE );
            output_data.frc = edge.frc( );
            output_data.length = edge.length_m( );
            output_data.from = edge.from_node( ).id( ).id;
            output_data.to = edge.to_node( ).id( ).id;
        }
        edge_data.grow( ).id = std::numeric_limits< uint64_t >::max( );  // sentinel
        nodes.close( );
        edges.close( );
        edge_data.close( );
    }

    uint32_t
    node_count( ) const
    {
        return m_nodes.size( );
    }
    uint32_t
    edge_count( ) const
    {
        return m_data.size( );
    }

    NodeRange
    nodes( ) const
    {
        return NodeRange( 0, m_nodes.size( ), this );
    }
    Edge
    find( ExternalId id ) const
    {
        auto pos = std::lower_bound( m_data.begin( ), m_data.end( ), id,
                                     []( const benchmark::EdgeData& left,
                                         const ExternalId& right ) { return left.id < right.id; } );
        return Edge( *pos, pos - m_data.begin( ), this );
    }
    Edge
    edge( InternalId id ) const
    {
        return Edge( m_data[ id.id ], id.id, this );
    }

private:
    flatdata::ArrayView< benchmark::Node > m_nodes;
    flatdata::ArrayView< benchmark::AdjacentEdge > m_edges;
    flatdata::ArrayView< benchmark::EdgeData > m_data;
};

inline typename FlatdataGraph::NodeRange
FlatdataGraph::Edge::from_node( ) const
{
    return NodeRange( m_data.from, m_data.from + 1, m_graph );
}

inline typename FlatdataGraph::NodeRange
FlatdataGraph::Edge::to_node( ) const
{
    return NodeRange( m_data.to, m_data.to + 1, m_graph );
}
