/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "test_structures.hpp"

#include <flatdata/flatdata.h>
#include <gtest/gtest.h>

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

TEST( MultiVectorTest, TestVariousDataFunctor )
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
        ASSERT_TRUE( reader.has_a );
        ASSERT_TRUE( reader.has_b );
        ASSERT_TRUE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each< AStructMutator, BStruct, CStruct >( 0, reader );
        ASSERT_TRUE( reader.has_a );
        ASSERT_TRUE( reader.has_b );
        ASSERT_TRUE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each( 1, reader );
        ASSERT_FALSE( reader.has_a );
        ASSERT_FALSE( reader.has_b );
        ASSERT_TRUE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each< AStructMutator, BStruct, CStruct >( 1, reader );
        ASSERT_FALSE( reader.has_a );
        ASSERT_FALSE( reader.has_b );
        ASSERT_TRUE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each( 2, reader );
        ASSERT_TRUE( reader.has_a );
        ASSERT_FALSE( reader.has_b );
        ASSERT_FALSE( reader.has_c );
    }

    {
        Reader reader;
        view.second.for_each< AStructMutator, BStruct, CStruct >( 2, reader );
        ASSERT_TRUE( reader.has_a );
        ASSERT_FALSE( reader.has_b );
        ASSERT_FALSE( reader.has_c );
    }
}

TEST( MultiVectorTest, TestVariousDataOverloadedLambda )
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
    ASSERT_TRUE( has_a );
    ASSERT_TRUE( has_b );
    ASSERT_TRUE( has_c );

    reset( );
    view.second.for_each( 1, reader );
    ASSERT_FALSE( has_a );
    ASSERT_FALSE( has_b );
    ASSERT_TRUE( has_c );

    reset( );
    view.second.for_each( 2, reader );
    ASSERT_TRUE( has_a );
    ASSERT_FALSE( has_b );
    ASSERT_FALSE( has_c );
}

TEST( MultiVectorTest, ConstFunctor )
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

TEST( MultiVectorTest, LambdaExplicit )
{
    auto view = create_view_with_3_items( );

    // also check that lambda works with explicit for_each
    bool has_b = false;
    view.second.for_each< BStruct >( 0, [&]( BStruct x ) { has_b = x.value == 1000; } );
    ASSERT_TRUE( has_b );
}

TEST( MultiVectorTest, LambdaImplicit )
{
    auto view = create_view_with_3_items( );

    // also check that lambda works with implicit for_each
    bool has_b = false;
    view.second.for_each( 0, make_overload( [&]( BStruct x ) { has_b = x.value == 1000; } ) );
    ASSERT_TRUE( has_b );
}

TEST( MultiVectorTest, IterateOneTypeElementsContinuouslyPlaced )
{
    auto view = create_view_with_3_items( );

    uint64_t index = 0;
    auto it = view.second.iterator< AStruct >( index );
    ASSERT_TRUE( it.valid( ) );
    bool has_a1 = ( *it ).value == 7;
    ++it;
    ASSERT_TRUE( it.valid( ) );
    bool has_a2 = ( *it ).value == 8;
    ++it;
    ASSERT_FALSE( it.valid( ) );

    ASSERT_TRUE( has_a1 );
    ASSERT_TRUE( has_a2 );
}

TEST( MultiVectorTest, IterateOneTypeElementsRandomlyPlaced )
{
    auto view = create_view_with_3_items( );

    uint64_t index = 0;
    auto it = view.second.iterator< CStruct >( index );
    ASSERT_TRUE( it.valid( ) );
    bool has_c1 = ( *it ).value == 1230000;
    ++it;
    ASSERT_TRUE( it.valid( ) );
    bool has_c2 = ( *it ).value == 1000000;
    ++it;
    ASSERT_FALSE( it.valid( ) );

    ASSERT_TRUE( has_c1 );
    ASSERT_TRUE( has_c2 );
}

TEST( MultiVectorTest, FlushingWhileBuilding )
{
    auto view = create_view_with_enough_items_to_flush( );

    for ( size_t i = 0; i < NUM_ITEMS_TO_CAUSE_FLUSH; i++ )
    {
        auto iter = view.second.iterator< CStruct >( i );
        ASSERT_TRUE( iter.valid( ) ) << "Expected CStruct at index " << i;
        ASSERT_EQ( i, ( *iter ).value ) << "Wrong data at index " << i;
        iter++;
        ASSERT_FALSE( iter.valid( ) ) << "Expected at most one CStruct at index  " << i;
    }
}

TEST( MultiVectorTest, StaticTest )
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

TEST( MultiVectorTest, CloseViewIsSameAsStorageView )
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

    ASSERT_EQ( view_from_close.size( ), view_from_storage.size( ) );
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

    ASSERT_EQ( values_view_from_close.size( ), values_view_from_storage.size( ) );
    for ( size_t i = 0; i < values_view_from_close.size( ); ++i )
    {
        ASSERT_EQ( values_view_from_close[ i ], values_view_from_storage[ i ] );
    }
}
