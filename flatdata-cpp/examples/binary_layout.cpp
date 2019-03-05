/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include "binary_layout.hpp"

#include <flatdata/MemoryResourceStorage.h>

int
main( int, const char** )
{
    std::shared_ptr< flatdata::MemoryResourceStorage > storage
        = flatdata::MemoryResourceStorage::create( );
    auto builder = binary_layout::BinaryLayoutBuilder::open( storage );

    flatdata::Vector< binary_layout::UnalignedStructure > alignment_example( 2 );
    auto struct_0 = alignment_example[ 0 ];
    struct_0.f0 = 0xF;
    struct_0.f1 = 0;
    struct_0.f2 = 0x07;

    auto struct_1 = alignment_example[ 1 ];
    struct_1.f0 = 0;
    struct_1.f1 = 0xFFFF;
    struct_1.f2 = 0x0;

    builder.set_alignment_example( alignment_example );

    std::cout << storage->hexdump( true );
    std::cout << storage->bindump( false );
    return 0;
}
