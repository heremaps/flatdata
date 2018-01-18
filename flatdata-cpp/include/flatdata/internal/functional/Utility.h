/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "FuncTraits.h"
#include "StaticList.h"

#include <cstdint>
#include <type_traits>
#include <utility>

namespace flatdata
{
template < typename... Args >
struct index_of
{
    static_assert( sizeof...( Args ) != 256, "Did not find type" );
};

template < typename T, typename... Args >
struct index_of< T, T, Args... > : std::integral_constant< size_t, 0 >
{
};

template < typename T, typename Arg, typename... Args >
struct index_of< T, Arg, Args... >
    : std::integral_constant< size_t, 1 + index_of< T, Args... >::value >
{
};

/** Helper structure for overloading lambdas */
template < typename... Fn >
struct overload;

template < typename F, typename... Fn >
struct overload< F, Fn... > : F, public overload< Fn... >
{
    overload( F f, Fn... fn )
        : F( std::move( f ) )
        , overload< Fn... >( std::move( fn )... )
    {
    }

    using F::operator( );
    using overload< Fn... >::operator( );
    using args_list = typename static_list::concat< typename arguments_of< F >::type,
                                                    typename overload< Fn... >::args_list >::type;
};

template < typename F >
struct overload< F > : F
{
    overload( F f )
        : F( std::move( f ) )
    {
    }

    using F::operator( );
    using args_list = static_list::list< typename arguments_of< F >::type >;
};

/**
 * Create an overloaded lambda.
 *
 * @warning Do not use this function in the body of a lambda. gcc 4.8.4 crashes when such code is
 *          compiled with debug symbols.
 *          Cf. https://gcc.gnu.org/bugzilla/show_bug.cgi?id=72779
 */
template < typename... Fn >
overload< Fn... > make_overload( Fn... fn )
{
    return overload< Fn... >( fn... );
}

/** alias for std::decay */
template < typename X >
struct decay
{
    using type = typename std::decay< X >::type;
};

/** lift of std::decay to static_list::list */
template < typename... Xs >
struct decay< static_list::list< Xs... > >
{
    using type = typename static_list::map< std::decay, static_list::list< Xs... > >::type;
};

/** c++14 shortcut for enable_if */
template < bool value >
using enable_if_t = typename std::enable_if< value, void >::type;

/** constant meta-function which transform any type to void */
template < typename >
struct void_t
{
    using type = void;
};

/** get the subtype args_list from F, if defined, otherwise return emptylist */
template < typename F, typename = void >
struct get_args_list
{
    using type = static_list::emptylist;
};

template < typename F >
struct get_args_list< F, typename void_t< typename F::args_list >::type >
{
    using type = typename F::args_list;
};

/** check if F has args_list subtype defined */
template < typename F, typename = void >
struct has_args_list
{
    static const bool value
        = !std::is_same< typename get_args_list< F >::type, static_list::emptylist >::value;
};
}  // namespace flatdata
