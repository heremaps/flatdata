/**
 * Copyright (c) 2020 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

namespace flatdata
{
/// Tags a type with it's invalid value
template < typename T, T INVALID_VALUE >
class Tagged
{
};
}  // namespace flatdata
