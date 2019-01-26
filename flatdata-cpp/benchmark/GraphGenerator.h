#pragma once

#include <stdint.h>
#include <cmath>
#include <cstdlib>
#include <utility>

struct ExternalId
{
    explicit ExternalId( uint64_t id )
        : id( id )
    {
    }
    uint64_t id;
};
struct InternalId
{
    explicit InternalId( uint32_t id )
        : id( id )
    {
    }
    uint32_t id;  // 0 to edge_count - 1
};
struct NodeId
{
    explicit NodeId( uint32_t id )
        : id( id )
    {
    }
    uint32_t id;  // 0 to node_count - 1
};
struct Coordinates
{
    Coordinates( uint32_t x, uint32_t y )
        : x( x )
        , y( y )
    {
    }
    uint32_t x;
    uint32_t y;
};

enum class Direction : uint8_t
{
    POSITIVE = 0,
    NEGATIVE = 1
};

/// Generates a simple grid graph with random attributes on edges
class GraphGenerator
{
public:
    class NodeRange;

    class Edge
    {
    public:
        InternalId id( ) const;
        ExternalId external_id( ) const;
        uint32_t length_m( ) const;  // 0 - 1.000.000
        bool is_a( ) const;
        bool is_b( ) const;
        bool is_c( ) const;
        bool is_d( ) const;
        bool is_e( Direction ) const;
        bool is_f( Direction ) const;
        bool is_g( Direction ) const;
        uint8_t frc( ) const;  // 0-4
        uint8_t speed_km_h( Direction ) const;
        NodeRange from_node( ) const;
        NodeRange to_node( ) const;

    private:
        friend class GraphGenerator;
        Edge( uint32_t id, uint32_t diameter );
        uint32_t m_id;
        uint32_t m_diameter;
    };

    class EdgeRange
    {
    public:
        EdgeRange( );
        void operator++( );
        void operator++( int );
        bool valid( ) const;
        size_t size( ) const;
        Direction dir( ) const;
        Edge data( ) const;

    private:
        friend class GraphGenerator;
        EdgeRange( uint32_t node, uint32_t current, uint32_t diameter );
        uint32_t m_node;
        uint32_t m_current;
        uint32_t m_diameter;
    };

    class NodeRange
    {
    public:
        NodeRange( );
        void operator++( );
        void operator++( int );
        bool valid( ) const;
        size_t size( ) const;
        NodeId id( ) const;
        Coordinates coordinates( ) const;
        EdgeRange edges( ) const;

    private:
        friend class GraphGenerator;
        NodeRange( uint32_t current, uint32_t end, uint32_t diameter );
        uint32_t m_current;
        uint32_t m_end;
        uint32_t m_diameter;
    };

public:
    explicit GraphGenerator( uint32_t num_nodes );

    uint32_t node_count( ) const;
    uint32_t edge_count( ) const;

    NodeRange nodes( ) const;
    Edge find( ExternalId ) const;
    Edge edge( InternalId ) const;

private:
    uint32_t m_num_nodes;
    uint32_t m_diameter;
};

inline GraphGenerator::GraphGenerator( uint32_t num_nodes )
{  // round up
    m_diameter = ceil( sqrt( num_nodes ) );
    m_num_nodes = m_diameter * m_diameter;
}

inline GraphGenerator::NodeRange
GraphGenerator::nodes( ) const
{
    return NodeRange( 0, m_num_nodes, m_diameter );
}

inline uint32_t
GraphGenerator::node_count( ) const
{
    return m_num_nodes;
}

inline uint32_t
GraphGenerator::edge_count( ) const
{
    return m_num_nodes * 2;
}

inline GraphGenerator::Edge
GraphGenerator::find( ExternalId id ) const
{
    return Edge( id.id >> 32, m_diameter );
}

inline GraphGenerator::Edge
GraphGenerator::edge( InternalId id ) const
{
    return Edge( id.id, m_diameter );
}

inline GraphGenerator::NodeRange::NodeRange( )
    : m_current( 0 )
    , m_end( 0 )
{
}
inline GraphGenerator::NodeRange::NodeRange( uint32_t current, uint32_t end, uint32_t diameter )
    : m_current( current )
    , m_end( end )
    , m_diameter( diameter )
{
}
inline void GraphGenerator::NodeRange::operator++( )
{
    m_current++;
}
inline void GraphGenerator::NodeRange::operator++( int )
{
    m_current++;
}
inline bool
GraphGenerator::NodeRange::valid( ) const
{
    return m_current < m_end;
}
inline size_t
GraphGenerator::NodeRange::size( ) const
{
    return m_end - m_current;
}
inline NodeId
GraphGenerator::NodeRange::id( ) const
{
    return NodeId{m_current};
}
inline Coordinates
GraphGenerator::NodeRange::coordinates( ) const
{
    uint32_t x = m_current % m_diameter;
    uint32_t y = m_current / m_diameter;
    x <<= 16;
    x |= y;
    y <<= 16;
    y |= x;
    return Coordinates{x, y};
}
inline GraphGenerator::EdgeRange
GraphGenerator::NodeRange::edges( ) const
{
    return EdgeRange( m_current, 0, m_diameter );
}

GraphGenerator::EdgeRange::EdgeRange( )
    : m_node( 0 )
    , m_current( 4 )
{
}
GraphGenerator::EdgeRange::EdgeRange( uint32_t node, uint32_t current, uint32_t diameter )
    : m_node( node )
    , m_current( current )
    , m_diameter( diameter )
{
}
void GraphGenerator::EdgeRange::operator++( )
{
    m_current++;
}
void GraphGenerator::EdgeRange::operator++( int )
{
    m_current++;
}
bool
GraphGenerator::EdgeRange::valid( ) const
{
    return m_current < 4;
}
size_t
GraphGenerator::EdgeRange::size( ) const
{
    return 4 - m_current;
}
Direction
GraphGenerator::EdgeRange::dir( ) const
{
    return m_current < 2 ? Direction::POSITIVE : Direction::NEGATIVE;
}
GraphGenerator::Edge
GraphGenerator::EdgeRange::data( ) const
{
    uint32_t x = m_node % m_diameter;
    uint32_t y = m_node / m_diameter;
    uint32_t previous_x = ( ( x + m_diameter - 1 ) % m_diameter );
    uint32_t previous_y = ( ( y + m_diameter - 1 ) % m_diameter );
    if ( m_current == 0 )
        return Edge( x + 2 * y * m_diameter, m_diameter );
    if ( m_current == 1 )
        return Edge( x + ( 2 * y + 1 ) * m_diameter, m_diameter );
    if ( m_current == 2 )
        return Edge( previous_x + 2 * y * m_diameter, m_diameter );
    return Edge( x + ( 2 * previous_y + 1 ) * m_diameter, m_diameter );
}

GraphGenerator::Edge::Edge( uint32_t id, uint32_t diameter )
    : m_id( id )
    , m_diameter( diameter )
{
}
InternalId
GraphGenerator::Edge::id( ) const
{
    return InternalId{m_id};
}
ExternalId
GraphGenerator::Edge::external_id( ) const
{
    return ExternalId{( static_cast< uint64_t >( m_id ) << 32 ) | ( m_id * 13 )};
}
uint32_t
GraphGenerator::Edge::length_m( ) const
{
    return 100 + ( ( m_id % 37 ) % 30 ) - 15;
}
bool
GraphGenerator::Edge::is_a( ) const
{
    return ( ( m_id % 37 ) & 2 ) != 0;
}
bool
GraphGenerator::Edge::is_b( ) const
{
    return ( ( m_id % 1013 ) & 2 ) != 0;
}
bool
GraphGenerator::Edge::is_c( ) const
{
    return ( ( m_id % 587 ) & 2 ) != 0;
}
bool
GraphGenerator::Edge::is_d( ) const
{
    return ( ( m_id % 7853 ) & 2 ) != 0;
}
bool GraphGenerator::Edge::is_e( Direction ) const
{
    return ( ( m_id % 6967 ) & 2 ) != 0;
}
bool GraphGenerator::Edge::is_f( Direction ) const
{
    return ( ( m_id % 5741 ) & 2 ) != 0;
}
bool GraphGenerator::Edge::is_g( Direction ) const
{
    return ( ( m_id % 5881 ) & 2 ) != 0;
}
uint8_t
GraphGenerator::Edge::frc( ) const
{
    uint32_t x = m_id % m_diameter;
    uint32_t y = m_id / m_diameter / 2;
    bool horizontal = ( ( m_id / m_diameter ) & 1 ) == 0;
    uint32_t value = y;
    if ( horizontal )
        value = x;
    if ( ( value & 1 ) == 0 )
        return 4;
    if ( ( value & 2 ) == 0 )
        return 3;
    if ( ( value & 4 ) == 0 )
        return 2;
    if ( ( value & 8 ) == 0 )
        return 1;
    return 0;
}
uint8_t
GraphGenerator::Edge::speed_km_h( Direction dir ) const
{
    uint8_t table[ 5 ] = {30, 40, 50, 60, 70};
    if ( dir == Direction::POSITIVE )
        return table[ frc( ) ];
    return table[ frc( ) ] - 1;
}
GraphGenerator::NodeRange
GraphGenerator::Edge::from_node( ) const
{
    uint32_t x = m_id % m_diameter;
    uint32_t y = m_id / m_diameter / 2;
    return NodeRange( x + y * m_diameter, x + y * m_diameter + 1, m_diameter );
}
GraphGenerator::NodeRange
GraphGenerator::Edge::to_node( ) const
{
    uint32_t x = m_id % m_diameter;
    uint32_t y = m_id / m_diameter / 2;
    bool horizontal = ( ( m_id / m_diameter ) & 1 ) == 0;
    if ( horizontal )
        x = ( x + 1 ) % m_diameter;
    else
        y = ( y + 1 ) % m_diameter;
    return NodeRange( x + y * m_diameter, x + y * m_diameter + 1, m_diameter );
}