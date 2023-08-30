/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "StaticList.h"

#include <cstddef>
#include <type_traits>

namespace flatdata
{
/**
 * Retrieve arguments list of a function type F.
 */
template < typename F >
struct arguments_of;

template < typename F >
struct arguments_of : public arguments_of< decltype( &F::operator( ) ) >
{
};

// specialization for function pointer
template < typename R, typename... Args >
struct arguments_of< R ( * )( Args... ) > : public arguments_of< R( Args... ) >
{
};

// specialization for class function pointer
template < typename C, typename R, typename... Args >
struct arguments_of< R ( C::* )( Args... ) const > : public arguments_of< R( Args... ) >
{
};

// generic case
template < typename R, typename... Args >
struct arguments_of< R( Args... ) >
{
    using type = static_list::list< Args... >;
};

namespace internal
{
struct is_callable_with_impl
{
    template < typename F, typename... Args >
    static decltype( std::declval< F >( )( std::declval< Args >( )... ), std::true_type( ) ) test(
        int );

    template < typename F, typename... Args >
    static std::false_type test( ... );
};
}  // namespace internal

/**
 * Check if a function of type F can be called with Args....
 */
template < typename F, typename... Args >
struct is_callable_with
{
    using type = decltype( internal::is_callable_with_impl::test< F, Args... >( 0 ) );
    static const bool value = std::is_same< type, std::true_type >::value;
};

/**
 * Check if a function of type F can be called with all Args_i from List<Args_0, Args_1, ... >,
 * where Args_i = List<Arg_(i_1), ..., Arg_(i_n)>.
 */
template < typename F, typename ListArgs >
struct is_callable_with_for_all
{
private:
    template < typename List >
    struct F_is_callable_with;

    template < typename... Args >
    struct F_is_callable_with< static_list::list< Args... > >
    {
        using type = typename is_callable_with< F, Args... >::type;
        static const bool value = std::is_same< type, std::true_type >::value;
    };

public:
    using type = typename static_list::reduce<
        static_list::conjuction,
        std::true_type,
        typename static_list::map< F_is_callable_with, ListArgs >::type >::type;
    static const bool value = std::is_same< type, std::true_type >::value;
};

template < typename Functor, typename Arg >
typename std::result_of< Functor( Arg ) >::type
apply_if_accepts_argument( Functor&& f, Arg arg )
{
    return f( arg );
}

// SFINAE fallback to be used in case functor does not accept arguments
inline void
apply_if_accepts_argument( ... )
{
}
}  // namespace flatdata
