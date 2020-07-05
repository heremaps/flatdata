/**
 * Copyright (c) 2017 HERE Europe B.V.
 * See the LICENSE file in the root of this project for license details.
 */

#pragma once

#include "ExternalVector.h"
#include "internal/MultiArrayViewImpl.h"

#include <cstdint>
#include <functional>
#include <sstream>
#include <string>
#include <tuple>

namespace flatdata
{
/*
 * Allows reading data stored with the MultiVector:
 * Each item i has a list of objects attached to it
 * To retrieve such a list use for_each with an appropriate functor:
 *
 *    MultiArrayView< A, B, C > view = ...;
 *    struct ABReader
 *    {
 *        void operator( )( A ) const { ... }
 *        void operator( )( B ) const { ... }
 *    }
 *
 *    view.for_each< A, B >( 10 ).call( ABReader( ... ) );  // explicit with functor
 *    view.for_each< C >( 10 ).call( [&]( C c ){ ... } );   // explicit with lambda
 *    view.for_each( 10 ).call( make_overload(              // implicit with overloaded lambda
 *        [&]( A ) { ... },
 *        [&]( B ) { ... }
 *    ) );
 */
template < typename IndexType, typename... Args >
class MultiArrayView
{
private:
    template < typename... AcceptedArgs >
    using AcceptedArgsList = typename std::conditional< ( sizeof...( AcceptedArgs ) > 0 ),
                                                        static_list::list< AcceptedArgs... >,
                                                        static_list::list< Args... > >::type;

public:
    using StreamType = const unsigned char*;

    MultiArrayView( );
    MultiArrayView( ArrayView< IndexType > index, StreamType data_begin );

    /**
     * Iterate through objects of specified types of the item specified by `index`.
     *
     * Usage example:
     *   MultiArrayView<A, B, C> view = ...;
     *   view.for_each<A, B>( index, callback );
     *   // callback must be callable with types A and B
     *
     * If the callback should be called with every type in the container, there is a shortcut
     * method (note that template arguments are not specified):
     *   view.for_each( index, callback );
     *   // callback must be callable with all types of the container, i.e. A, B and C
     *
     * It is also possible to overload a function partially and not to specify all accepted types
     * explicitly. For that use flatdata::make_overload. It stores the information, which types are
     * accepted by the callback in the callback's type:
     *   view.for_each( index, make_overload(
     *       [](A) { ... },
     *       [](B) { ... },
     *   );
     *   // callback will be called for objects of types A and B
     *
     * @tparam AcceptedArgs... Types of objects, for which the callback should be called
     * @param index Index of the item
     * @return Proxy object that calls a given function. In particular, the given function must be
     *         callable for each type in AcceptedArgs.
     */
    template < typename... AcceptedArgs,
               typename F,
               typename = enable_if_t< !has_args_list< F >::value >,
               typename
               = enable_if_t< internal::ExplicitForEachAssert< F,
                                                               AcceptedArgsList< AcceptedArgs... >,
                                                               Args... >::value > >
    void
    for_each( uint64_t index, F&& callback ) const
    {
        for_each_impl( index, std::forward< F >( callback ) );
    }

    /** Show static assert of failed explicit check */
    template < typename... AcceptedArgs,
               typename F,
               typename = enable_if_t< !has_args_list< F >::value >,
               typename
               = enable_if_t< !internal::ExplicitForEachAssert< F,
                                                                AcceptedArgsList< AcceptedArgs... >,
                                                                Args... >::value > >
    std::false_type
    for_each( uint64_t /*unused*/, F&& /*unused*/ ) const
    {
        internal::ExplicitForEachAssert< F, AcceptedArgsList< AcceptedArgs... >,
                                         Args... >::do_assert( );
        return {};
    }

    /**
     * Implicit version of `for_each` to use with lambda's constructed by flatdata::make_overload.
     */
    template < typename F,
               typename = enable_if_t< has_args_list< F >::value >,
               typename = enable_if_t< internal::ImplicitForEachAssert< F, Args... >::value > >
    void
    for_each( uint64_t index, F&& callback ) const
    {
        for_each_impl( index, std::forward< F >( callback ) );
    }

    /** Show static assert of failed implicit check */
    template < typename F,
               typename = enable_if_t< has_args_list< F >::value >,
               typename = enable_if_t< !internal::ImplicitForEachAssert< F, Args... >::value > >
    std::false_type
    for_each( uint64_t /*unused*/, F&& /*unused*/ ) const
    {
        internal::ImplicitForEachAssert< F, Args... >::do_assert( );
        return {};
    }

    size_t size( ) const;
    std::string describe( size_t unused = 0 ) const;
    explicit operator bool( ) const;

    template < typename ElementType >
    class Iterator
    {
    public:
        Iterator( ) = default;
        Iterator( StreamType data_current, StreamType data_end );

        bool valid( ) const;
        ElementType operator*( ) const;  // get current value
        void operator++( );              // go to the next element
        void operator++( int );          // go to the next element

    private:
        StreamType m_data_current = nullptr;
        StreamType m_data_end = nullptr;
        bool m_valid = false;
    };

    template < typename ElementType >
    Iterator< ElementType > iterator( uint64_t index ) const;

private:
    template < typename F >
    void for_each_impl( uint64_t index, F&& callback ) const;

    template < typename Functor, size_t index >
    static size_t get_impl( const unsigned char* /*unused*/,
                            size_t /*unused*/,
                            Functor&& /*unused*/ );

    template < typename Functor, size_t index, typename Arg, typename... PackedArgs >
    static size_t get_impl( const unsigned char* data, size_t type, Functor&& callback );

private:
    ArrayView< IndexType > m_index;
    StreamType m_data_begin;
};

// -------------------------------------------------------------------------------------------------

template < typename IndexType, typename... Args >
MultiArrayView< IndexType, Args... >::MultiArrayView( )
    : m_data_begin( nullptr )
{
}

template < typename IndexType, typename... Args >
MultiArrayView< IndexType, Args... >::MultiArrayView( ArrayView< IndexType > index,
                                                      StreamType data_begin )
    : m_index( index )
    , m_data_begin( data_begin )
{
}

template < typename IndexType, typename... Args >
size_t
MultiArrayView< IndexType, Args... >::size( ) const
{
    return m_index.size( );
}

template < typename IndexType, typename... Args >
std::string MultiArrayView< IndexType, Args... >::describe( size_t /*unused*/ ) const
{
    std::ostringstream ss;
    if ( this->operator bool( ) )
    {
        ss << "MultiArray of size " << size( ) << ", with index: " << m_index.describe( );
    }
    else
    {
        ss << "Uninitialized MultiArray";
    }
    return ss.str( );
}

template < typename IndexType, typename... Args >
MultiArrayView< IndexType, Args... >::operator bool( ) const
{
    return static_cast< bool >( m_index ) && m_data_begin != nullptr;
}

template < typename IndexType, typename... Args >
template < typename F >
void
MultiArrayView< IndexType, Args... >::for_each_impl( uint64_t index, F&& callback ) const
{
    std::pair< size_t, size_t > range = m_index[ index ].range;
    for ( auto data = m_data_begin + range.first, end = m_data_begin + range.second; data < end; )
    {
        unsigned char type = *data;
        data++;
        data += get_impl< F, 0, typename Args::AccessorType... >( data, type,
                                                                  std::forward< F >( callback ) );
    }
}

template < typename IndexType, typename... Args >
template < typename Functor, size_t index >
size_t
MultiArrayView< IndexType, Args... >::get_impl( const unsigned char* /*unused*/,
                                                size_t /*unused*/,
                                                Functor&& /*unused*/ )
{
    throw std::runtime_error( "Corrupted MultiArrayView data, unexpected type" );
}

template < typename IndexType, typename... Args >
template < typename Functor, size_t index, typename Arg, typename... PackedArgs >
size_t
MultiArrayView< IndexType, Args... >::get_impl( const unsigned char* data,
                                                size_t type,
                                                Functor&& callback )
{
    if ( type == index )
    {
        apply_if_accepts_argument( callback, Arg( data ) );
        return Arg::size_in_bytes( );
    }
    return get_impl< Functor&&, index + 1, PackedArgs... >( data, type,
                                                            std::forward< Functor >( callback ) );
}

// Needed to resolve typename issue correctly on different compilers
// See e.g.
// http://stackoverflow.com/questions/6232294/which-compiler-is-right-template-before-templated-return-type-needed
//  and http://stackoverflow.com/questions/8208203/nested-templates-workaround-for-msvc2010
// Unfortunately both of Boost macros are not working as expected, so have to define our own
#if defined( _MSC_VER )
#define TEMPLATE_WORKAROUND
#else
#define TEMPLATE_WORKAROUND template
#endif

template < typename IndexType, typename... Args >
template < typename ElementType >
typename MultiArrayView< IndexType, Args... >::TEMPLATE_WORKAROUND Iterator< ElementType >
MultiArrayView< IndexType, Args... >::iterator( uint64_t index ) const
{
    std::pair< size_t, size_t > range = m_index[ index ].range;
    return MultiArrayView< IndexType, Args... >::Iterator< ElementType >(
        m_data_begin + range.first, m_data_begin + range.second );
}

// -------------------------------------------------------------------------------------------------

template < typename IndexType, typename... Args >
template < typename ElementType >
MultiArrayView< IndexType, Args... >::Iterator< ElementType >::Iterator( StreamType data_current,
                                                                         StreamType data_end )
    : m_data_current( data_current )
    , m_data_end( data_end )
{
    if ( m_data_current < m_data_end )
    {
        ++( *this );
    }
}

template < typename IndexType, typename... Args >
template < typename ElementType >
void MultiArrayView< IndexType, Args... >::Iterator< ElementType >::operator++( )
{
    bool found = false;
    auto callback = [&]( ElementType ) { found = true; };
    while ( !found && m_data_current != m_data_end )
    {
        unsigned char type = *m_data_current;
        m_data_current++;
        m_data_current
            += MultiArrayView< IndexType, Args... >::get_impl< decltype( callback )&, 0,
                                                               typename Args::AccessorType... >(
                m_data_current, type, callback );
    }
    m_valid = found;
}

template < typename IndexType, typename... Args >
template < typename ElementType >
void MultiArrayView< IndexType, Args... >::Iterator< ElementType >::operator++( int )
{
    operator++( );
}

template < typename IndexType, typename... Args >
template < typename ElementType >
ElementType MultiArrayView< IndexType, Args... >::Iterator< ElementType >::operator*( ) const
{
    return ElementType{m_data_current - ElementType::size_in_bytes( )};
}

template < typename IndexType, typename... Args >
template < typename ElementType >
bool
MultiArrayView< IndexType, Args... >::Iterator< ElementType >::valid( ) const
{
    return m_valid;
}
}  // namespace flatdata
