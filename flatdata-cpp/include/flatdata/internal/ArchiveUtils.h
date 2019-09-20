/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <string>

namespace flatdata
{
namespace internal
{

inline std::string
signature_name( const std::string& archive_name )
{
    return archive_name + ".archive";
}

}  // namespace internal
}  // namespace flatdata
