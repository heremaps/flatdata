#include "FlatdataGraph.h"
#include "GraphGenerator.h"
#include "StructGraph.h"

#include "BreadthFirstSearch.h"
#include "Dijkstra.h"
#include "Lookup.h"
#include "Printer.h"

#include <sys/resource.h>
#include <sys/time.h>
#include <sys/types.h>

#include <fstream>
#include <iomanip>

template < typename F >
void
measure( F&& f )
{
    timespec begin_time;
    clock_gettime( CLOCK_PROCESS_CPUTIME_ID, &begin_time );
    rusage usage_start;
    getrusage( RUSAGE_SELF, &usage_start );

    f( );

    timespec end_time;
    clock_gettime( CLOCK_PROCESS_CPUTIME_ID, &end_time );
    rusage usage_end;
    getrusage( RUSAGE_SELF, &usage_end );
    std::cout << "CPU time (ms) = " << std::fixed << std::dec << std::setprecision( 4 )
              << ( (double)end_time.tv_sec - begin_time.tv_sec ) * 1000
                     + ( (double)end_time.tv_nsec - begin_time.tv_nsec ) / 1000000
              << std::endl;
    std::cout << "CPU time rusage user (ms) = " << std::fixed << std::dec << std::setprecision( 4 )
              << ( (double)usage_end.ru_utime.tv_sec - usage_start.ru_utime.tv_sec ) * 1000
                     + ( (double)usage_end.ru_utime.tv_usec - usage_start.ru_utime.tv_usec ) / 1000
              << std::endl;
    std::cout << "CPU time rusage sys (ms) = " << std::fixed << std::dec << std::setprecision( 4 )
              << ( (double)usage_end.ru_stime.tv_sec - usage_start.ru_stime.tv_sec ) * 1000
                     + ( (double)usage_end.ru_stime.tv_usec - usage_start.ru_stime.tv_usec ) / 1000
              << std::endl;
    std::cout << "Memory rusage peak (kb) = " << std::fixed << std::dec << usage_end.ru_maxrss
              << std::endl;
}

std::vector< char >
read_file( const char* name )
{
    std::ifstream file( name, std::ios::binary | std::ios::ate );
    std::streamsize size = file.tellg( );
    file.seekg( 0, std::ios::beg );

    std::vector< char > buffer( size );
    if ( !file.read( buffer.data( ), size ) )
    {
        std::cerr << "Failed to read file " << name << std::endl;
        abort( );
    }
    return buffer;
}

void
write_file( const char* name, const std::vector< char >& data )
{
    {
        std::ofstream file( name, std::ios::binary );
        if ( !file.write( data.data( ), data.size( ) ) )
        {
            std::cerr << "Failed to write file " << name << std::endl;
            abort( );
        }
    }
}

template < typename F, typename... Args >
void
call_for_graph( const char* graph_name, uint32_t num_nodes, Args... args )
{
    if ( graph_name == std::string( ) || graph_name == std::string( "reference" ) )
    {
        GraphGenerator graph( num_nodes );
        return F::call( graph, args... );
    }
    if ( graph_name == std::string( "struct" ) )
    {
        auto buffer = read_file( "benchmark.struct" );
        StructGraph graph( buffer.data( ) );
        return F::call( graph, args... );
    }
    if ( graph_name == std::string( "flatdata" ) )
    {
        auto storage = flatdata::FileResourceStorage::create( "benchmark.flatdata" );
        auto archive = benchmark::Graph::open( std::move( storage ) );
        FlatdataGraph graph( archive );
        return F::call( graph, args... );
    }
}

struct DoPrint
{
    template < typename Graph >
    static void
    call( const Graph& graph )
    {
        measure( [&]( ) { print_graph( graph ); } );
    }
};

struct DoBFS
{
    template < typename Graph >
    static void
    call( const Graph& graph, uint32_t num_runs )
    {
        measure( [&]( ) { bfs( graph, num_runs ); } );
    }
};

struct DoDijkstra
{
    template < typename Graph >
    static void
    call( const Graph& graph, uint32_t num_runs )
    {
        measure( [&]( ) { dijkstra( graph, num_runs ); } );
    }
};

struct DoLookup
{
    template < typename Graph >
    static void
    call( const Graph& graph, uint32_t num_runs )
    {
        measure( [&]( ) { lookup( graph, num_runs ); } );
    }
};

void
create( const char* graph_name, uint32_t num_nodes )
{
    GraphGenerator graph( num_nodes );
    if ( graph_name == std::string( "struct" ) )
    {
        auto buffer = StructGraph::create( graph );
        write_file( "benchmark.struct", buffer );
        return;
    }
    if ( graph_name == std::string( "flatdata" ) )
    {
        auto storage = flatdata::FileResourceStorage::create( "benchmark.flatdata" );
        auto archive = benchmark::GraphBuilder::open( std::move( storage ) );
        FlatdataGraph::create( graph, archive );
        return;
    }
}

int
main( int argc, char** argv )
{
    std::cout << "Usage: flatdata_benchmark COMMAND GRAPH OPTIONS" << std::endl;
    std::cout << "Commands: print, create, bfs, dijkstra, lookup, all_tests" << std::endl;
    std::cout << "Graphs: reference, struct, flatdata" << std::endl;
    std::cout << "Options: --num_nodes XXX" << std::endl;
    std::cout << "Options: --num_runs XXX" << std::endl;

    if ( argc < 3 )
        return 1;

    uint32_t num_nodes = 4;
    uint32_t num_runs = 1;
    std::cout << "Called with: " << argv[ 0 ] << " " << argv[ 1 ] << " " << argv[ 2 ];
    for ( int i = 3; i < argc; i++ )
    {
        std::cout << " " << argv[ i ];
        if ( argv[ i ] == std::string( "--num_nodes" ) )
        {
            if ( i + 1 >= argc )
            {
                return 1;
            }
            num_nodes = atoi( argv[ ++i ] );
            std::cout << " " << num_nodes;
        }
        else if ( argv[ i ] == std::string( "--num_runs" ) )
        {
            if ( i + 1 >= argc )
            {
                return 1;
            }
            num_runs = atoi( argv[ ++i ] );
            std::cout << " " << num_runs;
        }
    }
    std::cout << std::endl;

    if ( argv[ 1 ] == std::string( "print" ) )
    {
        call_for_graph< DoPrint >( argv[ 2 ], num_nodes );
    }
    if ( argv[ 1 ] == std::string( "bfs" ) || argv[ 1 ] == std::string( "all_tests" ) )
    {
        call_for_graph< DoBFS >( argv[ 2 ], num_nodes, num_runs );
    }
    if ( argv[ 1 ] == std::string( "dijkstra" ) || argv[ 1 ] == std::string( "all_tests" ) )
    {
        call_for_graph< DoDijkstra >( argv[ 2 ], num_nodes, num_runs );
    }
    if ( argv[ 1 ] == std::string( "lookup" ) || argv[ 1 ] == std::string( "all_tests" ) )
    {
        call_for_graph< DoLookup >( argv[ 2 ], num_nodes, num_runs );
    }
    if ( argv[ 1 ] == std::string( "create" ) )
    {
        measure( [&]( ) { create( argv[ 2 ], num_nodes ); } );
    }

    return 0;
}
