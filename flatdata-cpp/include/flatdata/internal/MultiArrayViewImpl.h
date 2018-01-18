/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "functional/FuncTraits.h"
#include "functional/StaticList.h"
#include "functional/Utility.h"

namespace flatdata
{
namespace internal
{

/** convert flatdata's type to its corresponding accessor type */
template < typename X >
struct get_accessor_type
{
    using type = typename X::AccessorType;
};

/** lift the above function to static_list::list */
template < typename... Xs >
struct get_accessor_type< static_list::list< Xs... > >
{
    using type = typename static_list::map< ::flatdata::internal::get_accessor_type,
                                            typename static_list::list< Xs... > >::type;
};

template < typename F, typename AcceptedArgsList, typename... MultiArrayViewArgs >
struct ExplicitForEachAssert
{
    static const bool accepted_types_is_subset_of_multivector_types = static_list::is_subset<
        typename static_list::map< get_accessor_type, AcceptedArgsList >::type,
        typename static_list::map< get_accessor_type,
                                   static_list::list< MultiArrayViewArgs... > >::type >::value;

    static const bool F_is_callable_with_all_accepted_types
        = is_callable_with_for_all< F,
                                    typename static_list::map< static_list::list,
                                                               AcceptedArgsList >::type >::value;

    static const bool value
        = accepted_types_is_subset_of_multivector_types && F_is_callable_with_all_accepted_types;

    static void
    do_assert( )
    {
        static_assert( accepted_types_is_subset_of_multivector_types,
                       "Not all provided types are types supported by the container." );
        static_assert( F_is_callable_with_all_accepted_types,
                       "Function is not callable with all required types." );
    }
};

template < typename F, typename... MultiArrayViewArgs >
struct ImplicitForEachAssert
{
    using F_decay = typename decay< F >::type;

    using F_args_list = typename static_list::map< get_accessor_type,
                                                   typename get_args_list< F_decay >::type >::type;

    static const bool F_has_args_list = !std::is_same< F_args_list, static_list::emptylist >::value;

    using args_list = typename static_list::
        map< static_list::list,
             typename static_list::map< get_accessor_type,
                                        static_list::list< MultiArrayViewArgs... > >::type >::type;

    static const bool F_is_callable_only_with_types_from_container
        = static_list::is_subset< F_args_list, args_list >::value;

    static const bool value = F_has_args_list && F_is_callable_only_with_types_from_container;

    static void
    do_assert( )
    {
        static_assert( F_has_args_list,
                       "Function type does not specify accepted arguments types: consider to "
                       "use flatdata::make_overload or specify types explicitely in for_each." );
        static_assert( F_is_callable_only_with_types_from_container,
                       "Function is callable with arguments that are not types supported by "
                       "the container." );
    }
};
}  // namespace internal
}  // namespace flatdata
