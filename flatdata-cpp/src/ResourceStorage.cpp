/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/internal/Reader.h>
#include <flatdata/internal/ResourceHandle.h>
#include <flatdata/ResourceStorage.h>
#include <iostream>

namespace flatdata
{
bool
ResourceStorage::write_to_stream( const char* resource_name,
                                  const char* schema,
                                  const unsigned char* data,
                                  size_t size_in_bytes )
{
    auto stream = create_output_stream( resource_name );
    resource_storage::write_to_stream( *stream, size_in_bytes );
    stream->write( reinterpret_cast< const char* >( data ), size_in_bytes );
    resource_storage::write_padding( *stream );
    stream->flush( );

    return static_cast< bool >( *stream ) && write_schema( resource_name, schema );
}

}  // namespace flatdata
