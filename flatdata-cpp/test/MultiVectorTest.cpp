/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include "catch_amalgamated.hpp"

using namespace flatdata;
using namespace test_structures;

static auto create_view_with_3_items = []( ) {
    auto storage = MemoryResourceStorage::create( );
    auto vector = storage->create_multi_vector< TestIndexType48, AStruct, BStructMutator, CStruct >(
        "data", "foo" );
    {
        auto list = vector.grow( );
        auto a1 = list.add< AStruct >( );
        a1.value = 7;
        auto a2 = list.add< AStructMutator >( );
        a2.value = 8;
        auto c1 = list.add< CStruct >( );
        c1.value = 1230000;
        auto b = list.add< BStruct >( );
        b.value = 1000;
        auto c2 = list.add< CStruct >( );
        c2.value = 1000000;
    }
    {
        auto list = vector.grow( );
        auto c = list.add< CStruct >( );
        c.value = 1000000;
    }
    {
        auto list = vector.grow( );
        auto a = list.add< AStructMutator >( );
        a.value = 8;
    }
    auto view_from_close = vector.close( );
    return std::make_pair( std::move( storage ), std::move( view_from_close ) );
};

static const size_t NUM_ITEMS_TO_CAUSE_FLUSH
    = 32 * 1024 * 1024 / 4 + 1024;  // enough to flush, and them some

static auto create_view_with_enough_items_to_flush = []( ) {
    auto storage = MemoryResourceStorage::create( );
    auto vector = storage->create_multi_vector< TestIndexType48, AStruct, BStruct, CStruct >(
        "data", "foo" );
    for ( size_t i = 0; i < NUM_ITEMS_TO_CAUSE_FLUSH; i++ )
    {
        auto list = vector.grow( );
        list.add< CStruct >( ).value = i;
    }
    auto view_from_close = vector.close( );
    return std::make_pair( std::move( storage ), std::move( view_from_close ) );
};

TEST_CASE( "Test various data functor", "[MultiVector]" )
{
    auto view = create_view_with_3_items( );
    struct Reader
    {
        void
        operator( )( AStruct x )
        {
            has_a |= x.value == 8;
        }
        void
        operator( )( BStruct x )
        {
            has_b |= x.value == 1000;
        }
        void
        operator( )( CStruct x )
        {
            has_c |= x.value == 1000000;
        }
        bool has_a = false;
        bool has_b = false;
        bool has_c = false;
    };

    {
        Reader reader;
        view.second.for_each( 0, reader );
        REQUIRE( reader.has_a );
        REQUIRE( reader.has_b );
        REQUIRE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each< AStructMutator, BStruct, CStruct >( 0, reader );
        REQUIRE( reader.has_a );
        REQUIRE( reader.has_b );
        REQUIRE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each( 1, reader );
        REQUIRE_FALSE( reader.has_a );
        REQUIRE_FALSE( reader.has_b );
        REQUIRE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each< AStructMutator, BStruct, CStruct >( 1, reader );
        REQUIRE_FALSE( reader.has_a );
        REQUIRE_FALSE( reader.has_b );
        REQUIRE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each( 2, reader );
        REQUIRE( reader.has_a );
        REQUIRE_FALSE( reader.has_b );
        REQUIRE_FALSE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each< AStructMutator, BStruct, CStruct >( 2, reader );
        REQUIRE( reader.has_a );
        REQUIRE_FALSE( reader.has_b );
        REQUIRE_FALSE( reader.has_c );
    }
}

TEST_CASE( "Test various data overloaded lambda", "[MultiVector]" )
{
    auto view = create_view_with_3_items( );

    bool has_a = false;
    bool has_b = false;
    bool has_c = false;

    auto reset = [&]( ) {
        has_a = false;
        has_b = false;
        has_c = false;
    };

    auto reader = flatdata::make_overload( [&]( AStruct x ) { has_a |= x.value == 8; },
                                           [&]( BStruct x ) { has_b |= x.value == 1000; },
                                           [&]( CStruct x ) { has_c |= x.value == 1000000; } );

    reset( );
    view.second.for_each( 0, reader );
    REQUIRE( has_a );
    REQUIRE( has_b );
    REQUIRE( has_c );

    reset( );
    view.second.for_each( 1, reader );
    REQUIRE_FALSE( has_a );
    REQUIRE_FALSE( has_b );
    REQUIRE( has_c );

    reset( );
    view.second.for_each( 2, reader );
    REQUIRE( has_a );
    REQUIRE_FALSE( has_b );
    REQUIRE_FALSE( has_c );
}

TEST_CASE( "for_each with const functor", "[MultiVector]" )
{
    auto view = create_view_with_3_items( );

    // check that it compiles if we pass in const functors
    struct NoReader
    {
        // required for const initialization
        NoReader( ) = default;
        void
        operator( )( const AStruct& ) const
        {
        }
    };
    const NoReader no_reader;
    view.second.for_each< AStruct >( 0, no_reader );
}

TEST_CASE( "for_each with explicit lambda", "[MultiVector]" )
{
    auto view = create_view_with_3_items( );

    // also check that lambda works with explicit for_each
    bool has_b = false;
    view.second.for_each< BStruct >( 0, [&]( BStruct x ) { has_b = x.value == 1000; } );
    REQUIRE( has_b );
}

TEST_CASE( "for_each with implicit lambda", "[MultiVector]" )
{
    auto view = create_view_with_3_items( );

    // also check that lambda works with implicit for_each
    bool has_b = false;
    view.second.for_each( 0, make_overload( [&]( BStruct x ) { has_b = x.value == 1000; } ) );
    REQUIRE( has_b );
}

TEST_CASE( "Iterate one type elements continuously placed", "[MultiVector]" )
{
    auto view = create_view_with_3_items( );

    uint64_t index = 0;
    auto it = view.second.iterator< AStruct >( index );
    REQUIRE( it.valid( ) );
    bool has_a1 = ( *it ).value == 7;
    ++it;
    REQUIRE( it.valid( ) );
    bool has_a2 = ( *it ).value == 8;
    ++it;
    REQUIRE_FALSE( it.valid( ) );

    REQUIRE( has_a1 );
    REQUIRE( has_a2 );
}

TEST_CASE( "Iterate one type elements randomly placed", "[MultiVector]" )
{
    auto view = create_view_with_3_items( );

    uint64_t index = 0;
    auto it = view.second.iterator< CStruct >( index );
    REQUIRE( it.valid( ) );
    bool has_c1 = ( *it ).value == 1230000;
    ++it;
    REQUIRE( it.valid( ) );
    bool has_c2 = ( *it ).value == 1000000;
    ++it;
    REQUIRE_FALSE( it.valid( ) );

    REQUIRE( has_c1 );
    REQUIRE( has_c2 );
}

TEST_CASE( "Flushing while building", "[MultiVector]" )
{
    auto view = create_view_with_enough_items_to_flush( );

    for ( size_t i = 0; i < NUM_ITEMS_TO_CAUSE_FLUSH; i++ )
    {
        auto iter = view.second.iterator< CStruct >( i );
        REQUIRE( iter.valid( ) );
        REQUIRE( ( *iter ).value == i );
        iter++;
        REQUIRE_FALSE( iter.valid( ) );
    }
}

TEST_CASE( "Static test", "[MultiVector]" )
{
    auto view = create_view_with_3_items( );

    auto accepted_lambda = []( const BStruct& ) {};
    auto not_accepted_lambda = []( int ) {};
    auto overloded_lambda = make_overload( accepted_lambda, not_accepted_lambda );

    static_assert(
        std::is_same< decltype( view.second.for_each< SimpleStruct >( 0, accepted_lambda ) ),
                      std::false_type >::value,
        "int is not in the types of the container" );
    static_assert(
        std::is_same< decltype( view.second.for_each< BStruct >( 0, not_accepted_lambda ) ),
                      std::false_type >::value,
        "lambda does not accept type declared as accepted" );
    static_assert( std::is_same< decltype( view.second.for_each<>( 0, not_accepted_lambda ) ),
                                 std::false_type >::value,
                   "lambda does not accept any type in the container" );
    static_assert( std::is_same< decltype( view.second.for_each( 0, not_accepted_lambda ) ),
                                 std::false_type >::value,
                   "lambda does not accept any type in the container" );
    static_assert( std::is_same< decltype( view.second.for_each( 0, accepted_lambda ) ),
                                 std::false_type >::value,
                   "lambda does not define accepted arguments implicitely and is not callable with "
                   "all types of the container" );
    static_assert( std::is_same< decltype( view.second.for_each( 0, overloded_lambda ) ),
                                 std::false_type >::value,
                   "lambda accepts a type not in the types of the containers" );
}

TEST_CASE( "Close view is same as storage view", "[MultiVector]" )
{
    auto storage = MemoryResourceStorage::create( );
    auto vector = storage->create_multi_vector< TestIndexType48, AStruct, BStructMutator, CStruct >(
        "data", "foo" );
    {
        auto list = vector.grow( );
        auto a1 = list.add< AStruct >( );
        a1.value = 7;
        auto a2 = list.add< AStructMutator >( );
        a2.value = 8;
        auto c1 = list.add< CStruct >( );
        c1.value = 1230000;
        auto b = list.add< BStruct >( );
        b.value = 1000;
        auto c2 = list.add< CStruct >( );
        c2.value = 1000000;
    }
    {
        auto list = vector.grow( );
        auto c = list.add< CStruct >( );
        c.value = 1000000;
    }
    {
        auto list = vector.grow( );
        auto a = list.add< AStructMutator >( );
        a.value = 8;
    }

    auto view_from_close = vector.close( );
    auto view_from_storage
        = *storage->read< MultiArrayView< TestIndexType48, AStructMutator, BStruct, CStruct > >(
            "data", "foo" );

    std::vector< size_t > values_view_from_close;
    std::vector< size_t > values_view_from_storage;

    REQUIRE( view_from_storage.size( ) == view_from_close.size( ) );
    for ( size_t i = 0; i < view_from_close.size( ); ++i )
    {
        view_from_close.for_each(
            i, flatdata::make_overload(
                   [&]( AStruct x ) { values_view_from_close.push_back( x.value ); },
                   [&]( BStruct x ) { values_view_from_close.push_back( x.value ); },
                   [&]( CStruct x ) { values_view_from_close.push_back( x.value ); } ) );
        view_from_storage.for_each(
            i, flatdata::make_overload(
                   [&]( AStruct x ) { values_view_from_storage.push_back( x.value ); },
                   [&]( BStruct x ) { values_view_from_storage.push_back( x.value ); },
                   [&]( CStruct x ) { values_view_from_storage.push_back( x.value ); } ) );
    };

    REQUIRE( values_view_from_storage.size( ) == values_view_from_close.size( ) );
    for ( size_t i = 0; i < values_view_from_close.size( ); ++i )
    {
        REQUIRE( values_view_from_storage[ i ] == values_view_from_close[ i ] );
    }
}
