/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include <type_traits>

//
// @note: template argument named Function always stands for a meta-function on types
//

namespace flatdata
{
namespace static_list
{
/**
 * none type for head of the empty list
 */
struct none
{
};

/**
 * List of types Ts... = T1, T2, ..., Tn
 */
template < typename... >
struct list;

/** Empty list */
template <>
struct list<>
{
    using head = none;
    using tail = list<>;
};
using emptylist = list<>;

/**
 * List (constructor)
 *
 * @tparam Head first element of the list
 * @tparam Tail... remaining elements of the list
 * @return a new list consisting of <Head, Tail...>
 */
template < typename Head, typename... Tail >
struct list< Head, Tail... >
{
    using head = Head;
    using tail = list< Tail... >;
    using type = list< Head, Tail... >;  // for making `list` a meta-function
};

/**
 * Construct a list from element T and List.
 *
 * @tparam T head of the list
 * @tparam List<Ts...> such that Ts... will become the tail of the list
 * @return the list <T, Ts...>
 */
template < typename T, typename List >
struct concat;
template < typename T, typename... Ts >
struct concat< T, list< Ts... > >
{
    using type = list< T, Ts... >;
};

/**
 * Check if List contains T.
 *
 * @tparam List<Ts...> list
 * @tparam T
 * @return true, if T is exactly one of the types Ts..., otherwise false.
 */
template < typename List, typename T >
struct contains
{
    static const bool value = std::is_same< typename List::head, T >::value
                              || contains< typename List::tail, T >::value;
    using type = typename std::conditional< value, std::true_type, std::false_type >::type;
};
template < typename T >
struct contains< emptylist, T >
{
    static const bool value = false;
    using type = std::false_type;
};

/**
 * Map meta-function Function over List
 *
 * @tparam Function<X> meta-function which maps element to element
 * @tparama List<Ts...> list of elements
 * @return map<Function, list<T1, T2, ...> = list<Function<T1>, Function<T2>, ...>
 */
template < template < typename... > class Function, typename List >
struct map
{
    using type = typename concat< typename Function< typename List::head >::type,
                                  typename map< Function, typename List::tail >::type >::type;
};
template < template < typename... > class Function >
struct map< Function, emptylist >
{
    using type = emptylist;
};

/**
 * Reduce List using meta-function Function<X, Y> starting with Accumulator from left
 *
 * @tparam Function<X, Y> meta-function which reduces element X, Y to another element
 * @tparam Accumulator accumulator to start reducing with
 * @tparam List<Ts...> list of elements to reduce
 * @return element Function<... Function<Function<Accumulator, X>, Y>, ...>
 */
template < template < typename... > class Function, typename Accumulator, typename List >
struct reduce
{
    using type = typename reduce< Function,
                                  typename Function< Accumulator, typename List::head >::type,
                                  typename List::tail >::type;
};
template < template < typename... > class Function, typename Accumulator >
struct reduce< Function, Accumulator, emptylist >
{
    using type = Accumulator;
};

/**
 * Conjuction of X and Y.
 *
 * @tparam X, Y elements of conjuction
 * @return boolean conjuction `X and Y`
 */
template < typename X, typename Y >
struct conjuction;
template <>
struct conjuction< std::true_type, std::true_type >
{
    using type = std::true_type;
};
template <>
struct conjuction< std::true_type, std::false_type >
{
    using type = std::false_type;
};
template <>
struct conjuction< std::false_type, std::true_type >
{
    using type = std::false_type;
};
template <>
struct conjuction< std::false_type, std::false_type >
{
    using type = std::false_type;
};

/**
 * check if ListA is a subset of ListB
 */
template < typename ListA, typename ListB >
struct is_subset
{
    template < typename X >
    struct is_contained_in_B
    {
        using type = typename contains< ListB, X >::type;
    };

    using type = typename reduce< conjuction,
                                  std::true_type,
                                  typename map< is_contained_in_B, ListA >::type >::type;
    static const bool value = std::is_same< type, std::true_type >::value;
};
}  // namespace static_list
}  // namespace flatdata
