#pragma once

#include "GraphGenerator.h"

#include <algorithm>
#include <vector>

class StructGraph
{
    struct InternalEdgeData
    {
        ExternalId id;
        uint32_t from;
        uint32_t to;
        uint32_t length;
        uint32_t speed_pos : 8;  // 8
        uint32_t speed_neg : 8;  // 16
        uint32_t is_a : 1;
        uint32_t is_b : 1;
        uint32_t is_c : 1;
        uint32_t is_d : 1;
        uint32_t is_e_pos : 1;
        uint32_t is_e_neg : 1;
        uint32_t is_f_pos : 1;
        uint32_t is_f_neg : 1;  // 24
        uint32_t is_g_pos : 1;
        uint32_t is_g_neg : 1;  // 26
        uint32_t frc : 3;       // 29
    };

    struct InternalEdge
    {
        uint32_t id : 31;
        uint32_t dir : 1;
    };

    struct InternalNode
    {
        Coordinates coords;
        uint32_t first_edge;
    };

public:
    class NodeRange;

    class Edge
    {
    public:
        InternalId
        id( ) const
        {
            return InternalId{m_id};
        }
        ExternalId
        external_id( ) const
        {
            return m_data->id;
        }
        uint32_t
        length_m( ) const
        {
            return m_data->length;
        }
        bool
        is_a( ) const
        {
            return m_data->is_a;
        }
        bool
        is_b( ) const
        {
            return m_data->is_b;
        }
        bool
        is_c( ) const
        {
            return m_data->is_c;
        }
        bool
        is_d( ) const
        {
            return m_data->is_d;
        }
        bool
        is_e( Direction dir ) const
        {
            return dir == Direction::POSITIVE ? m_data->is_e_pos : m_data->is_e_neg;
        }
        bool
        is_f( Direction dir ) const
        {
            return dir == Direction::POSITIVE ? m_data->is_f_pos : m_data->is_f_neg;
        }
        bool
        is_g( Direction dir ) const
        {
            return dir == Direction::POSITIVE ? m_data->is_g_pos : m_data->is_g_neg;
        }
        uint8_t
        frc( ) const
        {
            return m_data->frc;
        }
        uint8_t
        speed_km_h( Direction dir ) const
        {
            return dir == Direction::POSITIVE ? m_data->speed_pos : m_data->speed_neg;
        }
        NodeRange from_node( ) const;
        NodeRange to_node( ) const;

    private:
        friend class StructGraph;
        Edge( const InternalEdgeData* data, uint32_t id, const StructGraph* graph )
            : m_data( data )
            , m_id( id )
            , m_graph( graph )
        {
        }
        const InternalEdgeData* m_data;
        uint32_t m_id;
        const StructGraph* m_graph;
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
        void operator++( )
        {
            m_current++;
        }
        void operator++( int )
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
            return static_cast< Direction >( m_graph->m_edges[ m_current ].dir );
        }
        Edge
        data( ) const
        {
            uint32_t id = m_graph->m_edges[ m_current ].id;
            return Edge( m_graph->m_data + id, id, m_graph );
        }

    private:
        friend class StructGraph;
        EdgeRange( uint32_t current, uint32_t end, const StructGraph* graph )
            : m_current( current )
            , m_end( end )
            , m_graph( graph )
        {
        }
        uint32_t m_current;
        uint32_t m_end;
        const StructGraph* m_graph;
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
        void operator++( )
        {
            m_current++;
        }
        void operator++( int )
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
            return NodeId{m_current};
        }
        Coordinates
        coordinates( ) const
        {
            return m_graph->m_nodes[ m_current ].coords;
        }
        EdgeRange
        edges( ) const
        {
            return EdgeRange( m_graph->m_nodes[ m_current ].first_edge,
                              m_graph->m_nodes[ m_current + 1 ].first_edge, m_graph );
        }

    private:
        friend class StructGraph;
        NodeRange( uint32_t current, uint32_t end, const StructGraph* graph )
            : m_current( current )
            , m_end( end )
            , m_graph( graph )
        {
        }
        uint32_t m_current;
        uint32_t m_end;
        const StructGraph* m_graph;
    };

public:
    StructGraph( const char* data )
    {
        m_node_count = ( (const uint32_t*)data )[ 0 ];
        m_edge_count = ( (const uint32_t*)data )[ 1 ];
        data += 2 * sizeof( uint32_t );
        m_nodes = (const InternalNode*)data;
        data += sizeof( InternalNode ) * ( m_node_count + 1 );  // sentinel
        m_edges = (const InternalEdge*)data;
        data += sizeof( InternalEdge ) * m_edge_count * 2;
        m_data = (const InternalEdgeData*)data;
        data += sizeof( InternalEdgeData ) * m_edge_count;
    }

    template < typename Graph >
    static std::vector< char >
    create( const Graph& graph )
    {
        std::vector< char > result;
        size_t size = 2 * sizeof( uint32_t ) + sizeof( InternalNode ) * ( graph.node_count( ) + 1 )
                      + sizeof( InternalEdge ) * graph.edge_count( ) * 2
                      + sizeof( InternalEdgeData ) * graph.edge_count( );
        result.reserve( size );
        result.resize( size );
        char* data = result.data( );
        ( (uint32_t*)data )[ 0 ] = graph.node_count( );
        ( (uint32_t*)data )[ 1 ] = graph.edge_count( );
        data += 2 * sizeof( uint32_t );
        InternalNode* nodes = (InternalNode*)data;
        data += sizeof( InternalNode ) * ( graph.node_count( ) + 1 );  // sentinel
        InternalEdge* edges = (InternalEdge*)data;
        data += sizeof( InternalEdge ) * graph.edge_count( ) * 2;
        InternalEdgeData* edge_data = (InternalEdgeData*)data;

        nodes[ 0 ].first_edge = 0;
        size_t edge_pos = 0;
        for ( auto node = graph.nodes( ); node.valid( ); node++ )
        {
            nodes[ node.id( ).id ].coords = node.coordinates( );
            for ( auto e = node.edges( ); e.valid( ); e++ )
            {
                edges[ edge_pos ].id = e.data( ).id( ).id;
                edges[ edge_pos ].dir = static_cast< uint32_t >( e.dir( ) );
                edge_pos++;
            }
            nodes[ node.id( ).id + 1 ].first_edge = edge_pos;
        }

        for ( uint32_t i = 0; i < graph.edge_count( ); i++ )
        {
            auto edge = graph.edge( InternalId{i} );
            auto& target = edge_data[ i ];

            target.id = edge.external_id( );
            target.speed_pos = edge.speed_km_h( Direction::POSITIVE );
            target.speed_neg = edge.speed_km_h( Direction::NEGATIVE );
            target.is_a = edge.is_a( );
            target.is_b = edge.is_b( );
            target.is_c = edge.is_c( );
            target.is_d = edge.is_d( );
            target.is_e_pos = edge.is_e( Direction::POSITIVE );
            target.is_e_neg = edge.is_e( Direction::NEGATIVE );
            target.is_f_pos = edge.is_f( Direction::POSITIVE );
            target.is_f_neg = edge.is_f( Direction::NEGATIVE );
            target.is_g_pos = edge.is_g( Direction::POSITIVE );
            target.is_g_neg = edge.is_g( Direction::NEGATIVE );
            target.frc = edge.frc( );
            target.length = edge.length_m( );
            target.from = edge.from_node( ).id( ).id;
            target.to = edge.to_node( ).id( ).id;
        }
        return result;
    }

    uint32_t
    node_count( ) const
    {
        return m_node_count;
    }
    uint32_t
    edge_count( ) const
    {
        return m_edge_count;
    }

    NodeRange
    nodes( ) const
    {
        return NodeRange( 0, m_node_count, this );
    }
    Edge
    find( ExternalId id ) const
    {
        auto pos = std::lower_bound( m_data, m_data + m_edge_count, id,
                                     []( const InternalEdgeData& left, const ExternalId& right ) {
                                         return left.id.id < right.id;
                                     } );
        return Edge( pos, pos - m_data, this );
    }
    Edge
    edge( InternalId id ) const
    {
        return Edge( m_data + id.id, id.id, this );
    }

private:
    uint32_t m_node_count;
    uint32_t m_edge_count;
    const InternalNode* m_nodes;
    const InternalEdge* m_edges;
    const InternalEdgeData* m_data;
};

inline StructGraph::NodeRange
StructGraph::Edge::from_node( ) const
{
    return NodeRange( m_data->from, m_data->from + 1, m_graph );
}
inline StructGraph::NodeRange
StructGraph::Edge::to_node( ) const
{
    return NodeRange( m_data->to, m_data->to + 1, m_graph );
}
