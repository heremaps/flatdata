/**
 * Copyright (c) 2025 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

// Test that code generated from schemas with imports compiles and works correctly.
// The "simple" test case: main.flatdata imports types.flatdata
// main.h is generated with #include "types.h" and only defines the local archive.
// types.h defines the struct from the imported file.
#include "imports/simple/main.h"

// The "cross_namespace" test case: main.flatdata imports other.flatdata (different namespace)
#include "imports/cross_namespace/main.h"

#include <flatdata/MemoryResourceStorage.h>
#include "catch_amalgamated.hpp"

TEST_CASE( "imported_types_are_usable_in_archive", "[Import]" )
{
    auto storage = flatdata::MemoryResourceStorage::create( );
    auto builder = app::ABuilder::open( storage );
    REQUIRE( builder.is_open( ) );

    auto data = builder.start_data( );
    auto& item = data.grow( );
    item.x = 42;
    item.y = 100;
    data.close( );

    auto archive = app::A::open( storage );
    REQUIRE( archive.data( ).size( ) == 1 );
    REQUIRE( archive.data( )[ 0 ].x == 42 );
    REQUIRE( archive.data( )[ 0 ].y == 100 );
}

TEST_CASE( "cross_namespace_imported_enum_works", "[Import]" )
{
    ::app::EntryMutator::StreamType data( 5 );
    ::app::EntryMutator entry( data );
    entry.id = 7;
    entry.kind = ::defs::Kind::B;

    ::app::Entry reader( data );
    REQUIRE( reader.id == 7 );
    REQUIRE( reader.kind == ::defs::Kind::B );
}
