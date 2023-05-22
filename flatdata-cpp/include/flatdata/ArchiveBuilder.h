/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "ResourceStorage.h"

#include <memory>

namespace flatdata
{
class ArchiveBuilder
{
public:
    virtual ~ArchiveBuilder( ) = default;

    /**
     * @brief Construct an uninitialized archive
     */
    ArchiveBuilder( ) = default;

    /**
     * @brief ArchiveBuilder
     * @param storage
     */
    explicit ArchiveBuilder( std::shared_ptr< flatdata::ResourceStorage > storage );
    /**
     * @brief Returns archive name. Is implemented by the concrete archive instances.
     */
    virtual const char* name( ) const = 0;

    /**
     * @brief Returns archive schema. Is implemented by the concrete archive instances.
     */
    virtual const char* schema( ) const = 0;

    /**
     * @brief Returns true if archive is correctly loaded
     */
    bool is_open( ) const;

    /**
     * @brief Returns true if archive is correctly loaded
     */
    explicit operator bool( ) const;

protected:
    bool initialize( );
    flatdata::ResourceStorage& storage( );
    const flatdata::ResourceStorage& storage( ) const;
    bool is_created( ) const;
    void check_created( ) const;

private:
    std::shared_ptr< flatdata::ResourceStorage > m_storage;
    bool m_created = false;
};

}  // namespace flatdata
