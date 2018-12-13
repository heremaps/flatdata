/**
 * Copyright (c) 2018 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#include <flatdata/internal/functional/StaticList.h>

namespace flatdata
{
namespace static_list
{
namespace
{
using singleton = concat< char, emptylist >::type;
using ls = concat< int, singleton >::type;

static_assert( std::is_same< singleton, list< char > >::value,
               "concat of an element and an empty list is a singleton list" );
static_assert( std::is_same< ls, list< int, char > >::value,
               "concat of an element and a singleton list is a two element list" );
static_assert( std::is_same< singleton::head, char >::value,
               "head of a singleton list is its element" );
static_assert( std::is_same< singleton::tail, emptylist >::value,
               "tail of a singleton list is empty" );
static_assert( std::is_same< ls::head, int >::value,
               "head of a two element list is the first element" );
static_assert( std::is_same< ls::tail, singleton >::value,
               "tail of a two element list is a singleton list containing the second element" );
static_assert( std::is_same< ls::tail::tail, emptylist >::value,
               "tail(tail) of a two element list is empty" );
static_assert( contains< emptylist, char >::value == false,
               "empty list does not contain anything" );
static_assert( contains< singleton, char >::value == true,
               "a singleton list contains its element" );
static_assert( contains< singleton, int >::value == false,
               "a singleton list does not contain some other element" );
static_assert( contains< ls, char >::value == true, "a list contains its elements" );
static_assert( contains< ls, int >::value == true, "a list contains its elements" );
static_assert( contains< ls, void >::value == false, "a list does not contain some other element" );

template < typename X >
struct always_true
{
    using type = std::true_type;
    static const bool value = true;
};

static_assert( std::is_same< map< always_true, emptylist >::type, emptylist >::value,
               "map of an empty list is an empty list" );
static_assert(
    std::is_same< map< always_true, singleton >::type, list< std::true_type > >::value,
    "map of the contant true function over a singleton list is a singleton list containing true" );
static_assert(
    std::is_same< map< always_true, ls >::type, list< std::true_type, std::true_type > >::value,
    "map of the constant true function over a two element list is a constant true two element "
    "list" );

static_assert(
    std::is_same< reduce< conjuction, std::true_type, emptylist >::type, std::true_type >::value,
    "reduce of an empty list with true accumulator and conjuction is true" );
static_assert(
    std::is_same< reduce< conjuction, std::true_type, list< std::true_type > >::type,
                  std::true_type >::value,
    "reduce of a singleton list containg true with true accumulator and conjuction is true" );
static_assert(
    std::is_same< reduce< conjuction, std::true_type, list< std::false_type > >::type,
                  std::false_type >::value,
    "reduce of a singleton list contains false with true accumulator and conjuction is false" );
static_assert(
    std::is_same<
        reduce< conjuction, std::true_type, list< std::false_type, std::false_type > >::type,
        std::false_type >::value,
    "reduce of a constant false list with true accumulator and conjuction is false" );
static_assert(
    std::is_same<
        reduce< conjuction, std::true_type, list< std::true_type, std::false_type > >::type,
        std::false_type >::value,
    "reduce of a mixed true/false list with true accumulator and conjuction is false" );
static_assert(
    std::is_same<
        reduce< conjuction, std::true_type, list< std::true_type, std::true_type > >::type,
        std::true_type >::value,
    "reduce of a constant true list with true accumulator and conjuction is true" );
static_assert(
    std::is_same<
        reduce< conjuction, std::false_type, list< std::true_type, std::true_type > >::type,
        std::false_type >::value,
    "reduce of a constant true list with false accumulator and conjuction is false" );

static_assert( is_subset< emptylist, emptylist >::value,
               "empty list is a subset of the empty list" );
static_assert( is_subset< emptylist, singleton >::value,
               "empty list is a subset of a singleton list" );
static_assert( is_subset< emptylist, ls >::value, "empty list is a subset of a list" );
static_assert( is_subset< singleton, ls >::value,
               "singleton list is a subset a list which extends it" );
static_assert( is_subset< ls, ls >::value, "a list is its own subset" );
static_assert( !is_subset< ls, singleton >::value,
               "a two element list is not a subset of singleton list" );
static_assert( !is_subset< ls, emptylist >::value,
               "a two element list is not a subset of empty list" );
static_assert( !is_subset< singleton, emptylist >::value,
               "singleton list is not a subset of the empty list" );

}  // namespace
}  // namespace static_list
}  // namespace flatdata
