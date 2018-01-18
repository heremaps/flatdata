/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "ArrayView.h"
#include "ExternalVector.h"
#include "internal/Writer.h"
#include "internal/ResourceStorageCommon.h"
#include "MemoryDescriptor.h"
#include "MultiVector.h"

#include <boost/noncopyable.hpp>
#include <boost/optional.hpp>

#include <fstream>
#include <memory>

namespace flatdata
{
class MemoryMappedFileStorage;
class ResourceHandle;

/**
 * @brief Hierarchical Resource Storage.
 *
 * Manages and returns resources corresponding to their keys. Keys can be slash-separated('/').
 * Manages schema for each resource and checks it on query.
 * Resource storage is expected to provide read-write access to resources
 */
class ResourceStorage : private boost::noncopyable
{
public:
    virtual ~ResourceStorage( ) = default;

    /**
     * @brief Read resource.
     * @param key resource key.
     * @param schema expected resource schema.
     * @return resource or empty object in case resource schema doesn't match one provided.
     */
    template < typename T >
    boost::optional< T > read( const char* resource_name, const char* schema );

    /**
     * @brief Write resource.
     * @param data resource data.
     * @param key resource key.
     * @param schema resource schema to store.
     * @return true if operation is successful, false - otherwise
     */
    template < typename T >
    bool write( const char* resource_name, const char* schema, T data );

    /**
     * @brief Creates managed external vector, allowing to conserve memory usage
     *        by flushing data to the resource storage from time to time, in case
     *        it is not memory-based.
     * @return growable array or nullptr on error
     */
    template < typename T >
    ExternalVector< T > create_external_vector( const char* resource_name, const char* schema );

    /**
     * @brief Creates managed multi-vector, allowing to conserve memory usage
     *        by flushing data to the resource storage from time to time, in case
     *        it is not memory-based.
     * @return MultiVector or nullptr on error
     */
    template < typename IndexType, typename... Args >
    MultiVector< IndexType, Args... > create_multi_vector( const char* resource_name,
                                                           const char* schema );

    /**
     * @brief Provides memory descriptor for a resource schema associated with the given key
     * @return memory descriptor associated with the key or nullptr on error
     */
    MemoryDescriptor read_schema( const char* key );

public:
    /**
     * @brief Get a directory by given key
     * @return ResourceStorage corresponding to the directory or nullptr on error
     */
    virtual std::unique_ptr< ResourceStorage > directory( const char* key ) = 0;

    /**
     * @brief Check if resource exists
     */
    virtual bool exists( const char* key ) = 0;

protected:
    /**
     * @brief Creates output stream for given key
     * @return Output stream associated with the key or nullptr on error
     */
    virtual std::shared_ptr< std::ostream > create_output_stream( const char* key ) = 0;

    /**
     * @brief Provides memory descriptor for a resource associated with the given key
     * @return memory descriptor associated with the key or nullptr on error
     */
    virtual MemoryDescriptor read_resource( const char* key ) = 0;

private:
    bool write_to_stream( const char* resource_name,
                          const char* schema,
                          const unsigned char* data,
                          size_t size_in_bytes );

    bool write_schema( const char* resource_name, const char* schema );

    MemoryDescriptor read_and_check_schema( const char* resource_name,
                                            const char* expected_schema );
};

}  // namespace flatdata

#include "internal/ResourceStorage.inl"
