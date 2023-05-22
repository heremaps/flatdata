#include "FlatdataGraph.h"
#include "GraphGenerator.h"
#include "StructGraph.h"

#include "BreadthFirstSearch.h"
#include "Dijkstra.h"
#include "Lookup.h"
#include "Printer.h"

#define DOCOPT_HEADER_ONLY
#include <docopt/docopt.h>

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
        measure( [ & ]( ) { print_graph( graph ); } );
    }
};

struct DoBFS
{
    template < typename Graph >
    static void
    call( const Graph& graph, uint32_t num_runs )
    {
        measure( [ & ]( ) { bfs( graph, num_runs ); } );
    }
};

struct DoDijkstra
{
    template < typename Graph >
    static void
    call( const Graph& graph, uint32_t num_runs )
    {
        measure( [ & ]( ) { dijkstra( graph, num_runs ); } );
    }
};

struct DoLookup
{
    template < typename Graph >
    static void
    call( const Graph& graph, uint32_t num_runs )
    {
        measure( [ & ]( ) { lookup( graph, num_runs ); } );
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

const char* USAGE = 1 + R"_(
flatdata benchmark

Compares performance of different algorithms on a random graph using eithera a raw memory struct
or a flatdata archive.

Usage:
  flatdata_benchmark create (struct|flatdata) --num-nodes=NUM
  flatdata_benchmark print (reference|struct|flatdata) [--num-nodes=NUM]
  flatdata_benchmark lookup (reference|struct|flatdata) [--num-runs=NUM] [--num-nodes=NUM]
  flatdata_benchmark bfs (reference|struct|flatdata) [--num-runs=NUM] [--num-nodes=NUM]
  flatdata_benchmark dijkstra (reference|struct|flatdata) [--num-runs=NUM] [--num-nodes=NUM]
  flatdata_benchmark -h | --help

Data:
  reference  Create a random graph in memory and execute the corresponding command for it.
  struct     Read data from benchmark.struct, reinterpret it as a C++ struct and execute the
             corresponding command for it.
  flatdata   Read data from flatdata archive and execute the corresponding command for it.

Commands:
  create    Create a new random graph and write it to disk either as a reinterpret_casted C++
            struct (filename: benchmark.struct) or flatdata archive (directory:
            benchmark.flatdata). Note that the struct data is overwritten on disk, however for the
            flatdata archive if it already exists, this command fails.
  print     Print graph representation stored on disk to console.
  lookup    Run a lookup for each edge in the graph in uniform random fashion.
  bfs       Execute Breath-First-Search starting at a random node.
  dijkstra  Exectue Dijkstra's shortest path algorithm starting at a random edge.

Options:
  -h --help         Show this screen.
  --num-nodes=NUM   Number of nodes in the random graph [default: 2000].
  --num-runs=NUM    Number of runs the specified algorithm should be executed [default: 10].
)_";

int
main( int argc, char** argv )
{
    auto args = docopt::docopt( USAGE, { argv + 1, argv + argc } );
    try
    {
        size_t num_nodes = args.at( "--num-nodes" ).asLong( );
        size_t num_runs = args.at( "--num-runs" ).asLong( );

        if ( args.at( "create" ).asBool( ) )
        {
            measure( [ & ]( ) { create( argv[ 2 ], num_nodes ); } );
        }
        else if ( args.at( "print" ).asBool( ) )
        {
            call_for_graph< DoPrint >( argv[ 2 ], num_nodes );
        }
        else if ( args.at( "lookup" ).asBool( ) )
        {
            size_t num_runs = args.at( "--num-runs" ).asLong( );
            call_for_graph< DoLookup >( argv[ 2 ], num_nodes, num_runs );
        }
        else if ( args.at( "bfs" ).asBool( ) )
        {
            size_t num_runs = args.at( "--num-runs" ).asLong( );
            call_for_graph< DoBFS >( argv[ 2 ], num_nodes, num_runs );
        }
        else if ( args.at( "dijkstra" ).asBool( ) )
        {
            size_t num_runs = args.at( "--num-runs" ).asLong( );
            call_for_graph< DoDijkstra >( argv[ 2 ], num_nodes, num_runs );
        }
    }
    catch ( const std::runtime_error& err )
    {
        std::cerr << "Error: " << err.what( ) << std::endl;
    }

    return 0;
}
