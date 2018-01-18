/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/internal/functional/FuncTraits.h>

namespace flatdata
{
namespace functraits_test
{
void free_function( std::true_type ){};
void zero_arity_function( ){};
struct Functor
{
    void operator( )( std::true_type, std::true_type ){};
};

static_assert( is_callable_with< decltype( free_function ), std::true_type >::value,
               "Free function is callable with its arguments" );
static_assert( !is_callable_with< decltype( free_function ), std::false_type >::value,
               "Free function is not callable with non-implicitly "
               "convertible arguments" );
static_assert( is_callable_with< decltype( zero_arity_function ) >::value,
               "Free function with zero arity is callable without arguments" );
static_assert( !is_callable_with< decltype( zero_arity_function ), std::true_type >::value,
               "Free function with zero arity is not callable with wrong arity" );
static_assert( is_callable_with< Functor, std::true_type, std::true_type >::value,
               "Functor is callable with its arguments" );
static_assert( !is_callable_with< Functor, std::true_type, std::false_type >::value,
               "Functor is not callable with non-implicitly convertible arguments" );
static_assert( !is_callable_with< Functor >::value, "Functor is not callable with wrong arity" );
}  // namespace functraits_test
}  // namespace flatdata
