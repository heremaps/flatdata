/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <iostream>

namespace flatdata
{
namespace internal
{
const char* const INDEX_SUFFIX = "_index";

inline std::string
multivector_index_schema( const char* original_schema )
{
    return "index(" + std::string( original_schema ) + ")";
}

template < typename T >
struct ValueCreator
{
    template < typename Loader >
    boost::optional< T >
    operator( )( const char* resource, const char* schema, Loader&& loader )
    {
        auto data = loader( resource, schema );
        if ( !data )
        {
            return boost::none;
        }
        return T{data.data( )};
    }
};

template < typename T >
struct ValueCreator< ArrayView< T > >
{
    template < typename Loader >
    boost::optional< ArrayView< T > >
    operator( )( const char* resource, const char* schema, Loader&& loader )
    {
        auto data = loader( resource, schema );
        if ( !data )
        {
            return boost::none;
        }
        return ArrayView< T >{data.data( ), data.data( ) + data.size_in_bytes( )};
    }
};

template < typename IndexType, typename... Args >
struct ValueCreator< MultiArrayView< IndexType, Args... > >
{
    template < typename Loader >
    boost::optional< MultiArrayView< IndexType, Args... > >
    operator( )( const char* resource, const char* schema, Loader&& loader )
    {
        auto index = ValueCreator< ArrayView< IndexType > >( )(
            ( std::string( resource ) + INDEX_SUFFIX ).c_str( ),
            multivector_index_schema( schema ).c_str( ), loader );
        auto data = loader( resource, schema );
        if ( !index || !data )
        {
            return boost::none;
        }
        return MultiArrayView< IndexType, Args... >{*index, data.data( )};
    }
};

template <>
struct ValueCreator< MemoryDescriptor >
{
    template < typename Loader >
    boost::optional< MemoryDescriptor >
    operator( )( const char* resource, const char* schema, Loader&& loader )
    {
        auto data = loader( resource, schema );
        if ( !data )
        {
            return boost::none;
        }
        return data;
    }
};

template <>
struct ValueCreator< BitsetView >
{
    template < typename Loader >
    boost::optional< BitsetView >
    operator( )( const char* resource, const char* schema, Loader&& loader )
    {
        auto data = loader( resource, schema );
        if ( !data )
        {
            return boost::none;
        }
        return BitsetView{data.data( ), data.data( ) + data.size_in_bytes( )};
    }
};

}  // namespace internal

template < typename T >
boost::optional< T >
ResourceStorage::read( const char* resource_name, const char* schema )
{
    auto loader = [this]( const char* name, const char* schema ) {
        return read_and_check_schema( name, schema );
    };
    return internal::ValueCreator< T >( )( resource_name, schema, loader );
}

inline MemoryDescriptor
ResourceStorage::read_schema( const char* key )
{
    return read_resource( ( std::string( key ) + ".schema" ).c_str( ) );
}

inline bool
ResourceStorage::write_schema( const char* resource_name, const char* schema )
{
    auto schema_stream
        = create_output_stream( ( std::string( resource_name ) + ".schema" ).c_str( ) );
    *schema_stream << schema;
    schema_stream->flush( );
    return static_cast< bool >( *schema_stream );
}

inline MemoryDescriptor
ResourceStorage::read_and_check_schema( const char* resource_name, const char* expected_schema )
{
    auto data = read_resource( resource_name );
    auto schema = read_resource( ( std::string( resource_name ) + ".schema" ).c_str( ) );

    if ( !data || !schema )
    {
        return MemoryDescriptor( );
    }

    if ( sizeof( resource_storage::size_type ) + PADDING_SIZE > data.size_in_bytes( ) )
    {
        return MemoryDescriptor( );
    }

    Reader< resource_storage::size_type > size_reader{data.data( )};
    auto size = size_reader;
    if ( size + sizeof( resource_storage::size_type ) + PADDING_SIZE != data.size_in_bytes( ) )
    {
        return MemoryDescriptor( );
    }

    std::string stored_schema( reinterpret_cast< const char* >( schema.data( ) ),
                               schema.size_in_bytes( ) );

    if ( stored_schema != expected_schema )
    {
        return MemoryDescriptor( );
    }

    return MemoryDescriptor{data.data( ) + sizeof( resource_storage::size_type ), size};
}

template < typename T >
bool
ResourceStorage::write( const char* resource_name, const char* schema, T data )
{
    return write_to_stream( resource_name, schema, data.data( ), data.size_in_bytes( ) );
}

template < typename T >
ExternalVector< T >
ResourceStorage::create_external_vector( const char* resource_name,
                                         const char* schema,
                                         size_t flush_size_bytes )
{
    write_schema( resource_name, schema );
    auto data_stream = create_output_stream( resource_name );
    return ExternalVector< T >( ResourceHandle::create( std::move( data_stream ) ),
                                flush_size_bytes );
}

inline ExternalBitset
ResourceStorage::create_external_bitset( const char* resource_name,
                                         const char* schema,
                                         size_t flush_size_bytes )
{
    write_schema( resource_name, schema );
    auto data_stream = create_output_stream( resource_name );
    return ExternalBitset( ResourceHandle::create( std::move( data_stream ) ), flush_size_bytes );
}

template < typename IndexType, typename... Args >
MultiVector< IndexType, Args... >
ResourceStorage::create_multi_vector( const char* resource_name,
                                      const char* schema,
                                      size_t flush_size_bytes )
{
    std::string index_name = std::string( resource_name ) + internal::INDEX_SUFFIX;
    auto index = create_external_vector< IndexType >(
        index_name.c_str( ), internal::multivector_index_schema( schema ).c_str( ),
        flush_size_bytes );

    write_schema( resource_name, schema );

    auto data_stream = create_output_stream( resource_name );
    return MultiVector< IndexType, Args... >(
        std::move( index ), ResourceHandle::create( std::move( data_stream ) ), flush_size_bytes );
}

}  // namespace flatdata
