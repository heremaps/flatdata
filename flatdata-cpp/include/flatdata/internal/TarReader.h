/**
 * Copyright (c) 2021 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <flatdata/MemoryDescriptor.h>

#include <stddef.h>
#include <string>
#include <vector>

namespace flatdata
{
namespace internal
{
struct TarFileEntry
{
    std::string name;
    size_t offset = 0;
    size_t size = 0;
};

std::vector< TarFileEntry > read_tar_file_entries( MemoryDescriptor data );
}  // namespace internal
}  // namespace flatdata
