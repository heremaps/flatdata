
#pragma once

#include <flatdata/flatdata.h>
#include <cstdint>
#include <iostream>
#include <iomanip>

namespace n {


template< template < typename, int, int, int > class Member >
union STemplate
{
    using XType = Member< uint64_t, 0, 64, 8 >;
    XType x;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = STemplate< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = STemplate< flatdata::Reader >;

    STemplate( );
    explicit STemplate( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const STemplate& other ) const;
    bool operator!=( const STemplate& other ) const;
    bool operator<( const STemplate& other ) const;
    operator STemplate< flatdata::Reader >( ) const;
    explicit operator bool( ) const;

    std::string to_string( ) const;
    std::string describe( size_t unused = 0 ) const;

    static constexpr bool IS_OVERLAPPING_WITH_NEXT = false;

    /**
    * Private data member, should not be directly used.
    * Cannot be made private.
    * Please refer to C++ Standard, Chapter 9.2, Paragraph 19.
    * This union has to be kept standard-layout, which different access control prevents.
    */
    Member< uint32_t, 0, 0, 0 > _data;
};


typedef STemplate< flatdata::Reader > S;
typedef STemplate< flatdata::Writer > SMutator;

} // namespace n

namespace n {

class X : public flatdata::Archive
{
public:
    /// Archive schema
    static const char* schema_definition( );
    /// Archive name
    static const char* name_definition( );

public:
    /**
    * Create and open archive at path.
    * In case opening fails, is_open() or operator bool() returns false.
    *
    * @sa is_open
    * @sa operator bool()
    */
    static X open( std::shared_ptr< flatdata::ResourceStorage > storage );
    X( ) = default;

    using PayloadType = flatdata::MemoryDescriptor;
    const PayloadType& payload( ) const;


    const char* name( ) const override;
    const char* schema( ) const override;

private:
    explicit X( std::shared_ptr< flatdata::ResourceStorage > storage );

    bool load_contents( ) override;
    void describe_resources( std::ostream& stream, size_t nest_level ) const override;

private:
    PayloadType m_payload;
};

class XBuilder : public flatdata::ArchiveBuilder
{
public:
    /// Creates Archive builder
    static XBuilder open( std::shared_ptr< flatdata::ResourceStorage > storage );
    /// Archive schema
    static const char* schema_definition( );

public:  /// Common methods
    XBuilder( ) = default;
    const char* name( ) const override;
    const char* schema( ) const override;

public:  /// Resources
    using PayloadType = flatdata::MemoryDescriptor;
    using PayloadReaderType = flatdata::MemoryDescriptor;
    bool set_payload( PayloadReaderType data );



private:
    XBuilder( std::shared_ptr< flatdata::ResourceStorage > storage );

};

} // namespace n

namespace m {


template< template < typename, int, int, int > class Member >
union STemplate
{
    using XType = Member< uint64_t, 0, 64, 8 >;
    XType x;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = STemplate< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = STemplate< flatdata::Reader >;

    STemplate( );
    explicit STemplate( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const STemplate& other ) const;
    bool operator!=( const STemplate& other ) const;
    bool operator<( const STemplate& other ) const;
    operator STemplate< flatdata::Reader >( ) const;
    explicit operator bool( ) const;

    std::string to_string( ) const;
    std::string describe( size_t unused = 0 ) const;

    static constexpr bool IS_OVERLAPPING_WITH_NEXT = false;

    /**
    * Private data member, should not be directly used.
    * Cannot be made private.
    * Please refer to C++ Standard, Chapter 9.2, Paragraph 19.
    * This union has to be kept standard-layout, which different access control prevents.
    */
    Member< uint32_t, 0, 0, 0 > _data;
};


typedef STemplate< flatdata::Reader > S;
typedef STemplate< flatdata::Writer > SMutator;

} // namespace m

namespace m {

class X : public flatdata::Archive
{
public:
    /// Archive schema
    static const char* schema_definition( );
    /// Archive name
    static const char* name_definition( );

public:
    /**
    * Create and open archive at path.
    * In case opening fails, is_open() or operator bool() returns false.
    *
    * @sa is_open
    * @sa operator bool()
    */
    static X open( std::shared_ptr< flatdata::ResourceStorage > storage );
    X( ) = default;

    using PayloadType = flatdata::MemoryDescriptor;
    const PayloadType& payload( ) const;


    const char* name( ) const override;
    const char* schema( ) const override;

private:
    explicit X( std::shared_ptr< flatdata::ResourceStorage > storage );

    bool load_contents( ) override;
    void describe_resources( std::ostream& stream, size_t nest_level ) const override;

private:
    PayloadType m_payload;
};

class XBuilder : public flatdata::ArchiveBuilder
{
public:
    /// Creates Archive builder
    static XBuilder open( std::shared_ptr< flatdata::ResourceStorage > storage );
    /// Archive schema
    static const char* schema_definition( );

public:  /// Common methods
    XBuilder( ) = default;
    const char* name( ) const override;
    const char* schema( ) const override;

public:  /// Resources
    using PayloadType = flatdata::MemoryDescriptor;
    using PayloadReaderType = flatdata::MemoryDescriptor;
    bool set_payload( PayloadReaderType data );



private:
    XBuilder( std::shared_ptr< flatdata::ResourceStorage > storage );

};

} // namespace m

namespace _builtin {
namespace multivector {

/** Builtin type to for MultiVector index */
template< template < typename, int, int, int > class Member >
union IndexType32Template
{
    using ValueType = Member< uint64_t, 0, 32, 4 >;
    ValueType value;
    using RangeType = Member< std::pair< uint64_t, uint64_t >, 0, 32, 4 >;
    RangeType range;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = IndexType32Template< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = IndexType32Template< flatdata::Reader >;

    IndexType32Template( );
    explicit IndexType32Template( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const IndexType32Template& other ) const;
    bool operator!=( const IndexType32Template& other ) const;
    bool operator<( const IndexType32Template& other ) const;
    operator IndexType32Template< flatdata::Reader >( ) const;
    explicit operator bool( ) const;

    std::string to_string( ) const;
    std::string describe( size_t unused = 0 ) const;

    static constexpr bool IS_OVERLAPPING_WITH_NEXT = true;

    /**
    * Private data member, should not be directly used.
    * Cannot be made private.
    * Please refer to C++ Standard, Chapter 9.2, Paragraph 19.
    * This union has to be kept standard-layout, which different access control prevents.
    */
    Member< uint32_t, 0, 0, 0 > _data;
};

/** Builtin type to for MultiVector index */
typedef IndexType32Template< flatdata::Reader > IndexType32;
typedef IndexType32Template< flatdata::Writer > IndexType32Mutator;

}} // namespace _builtin.multivector

namespace a {

class A : public flatdata::Archive
{
public:
    /// Archive schema
    static const char* schema_definition( );
    /// Archive name
    static const char* name_definition( );

public:
    /**
    * Create and open archive at path.
    * In case opening fails, is_open() or operator bool() returns false.
    *
    * @sa is_open
    * @sa operator bool()
    */
    static A open( std::shared_ptr< flatdata::ResourceStorage > storage );
    A( ) = default;

    using SingleType = ::n::S;
    const SingleType& single( ) const;

    using ListType = flatdata::ArrayView< ::m::S >;
    const ListType& list( ) const;

    using MultiType = flatdata::MultiArrayView< ::_builtin::multivector::IndexType32, ::n::S >;
    const MultiType& multi( ) const;

    using InnerType = ::n::X;
    const InnerType& inner( ) const;


    const char* name( ) const override;
    const char* schema( ) const override;

private:
    explicit A( std::shared_ptr< flatdata::ResourceStorage > storage );

    bool load_contents( ) override;
    void describe_resources( std::ostream& stream, size_t nest_level ) const override;

private:
    SingleType m_single;
    ListType m_list;
    MultiType m_multi;
    InnerType m_inner;
};

class ABuilder : public flatdata::ArchiveBuilder
{
public:
    /// Creates Archive builder
    static ABuilder open( std::shared_ptr< flatdata::ResourceStorage > storage );
    /// Archive schema
    static const char* schema_definition( );

public:  /// Common methods
    ABuilder( ) = default;
    const char* name( ) const override;
    const char* schema( ) const override;

public:  /// Resources
    using SingleType = ::n::S;
    using SingleReaderType = ::n::S;
    bool set_single( SingleReaderType data );

    using ListType = flatdata::ExternalVector< ::m::S >;
    using ListReaderType = flatdata::ArrayView< ::m::S >;
    ListType start_list( );
    bool set_list( ListReaderType data );

    using MultiType = flatdata::MultiVector< ::_builtin::multivector::IndexType32, ::n::S >;
    using MultiReaderType = flatdata::MultiArrayView< ::_builtin::multivector::IndexType32, ::n::S >;
    MultiType start_multi( );


    using InnerType = ::n::XBuilder;
    InnerType& inner( );

private:
    ABuilder( std::shared_ptr< flatdata::ResourceStorage > storage );

    InnerType m_inner;
};

} // namespace a


// -------------------------------------------------------------------------------------------------
// -------------------------------------- Implementations ------------------------------------------
// -------------------------------------------------------------------------------------------------

namespace n {
namespace internal
{
    const char* const S__schema__ = R"schema(namespace n {
struct S
{
    x : u64 : 64;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
STemplate< Member >::STemplate( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
STemplate< Member >::STemplate( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
STemplate< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename STemplate< Member >::StreamType STemplate< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string STemplate< Member >::schema( ) { return internal::S__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string STemplate< Member >::name( ) { return "S"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t STemplate< Member >::size_in_bytes( ) { return 8; }

template< template < typename, int, int, int > class Member >
inline
bool STemplate< Member >::operator==( const STemplate& other ) const
{
    for ( size_t i = 0; i < size_in_bytes( ); i++ )
    {
        if ( _data.data[ i ] != other._data.data[ i ] )
        {
            return false;
        }
    }
    return true;
}

template< template < typename, int, int, int > class Member >
inline
bool STemplate< Member >::operator!=( const STemplate& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool STemplate< Member >::operator<( const STemplate& other ) const
{
return
    x < other.x;
}

template< template < typename, int, int, int > class Member >
inline
STemplate< Member >::operator STemplate< flatdata::Reader >( ) const
{
    return STemplate< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string STemplate< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "S {" << std::endl <<
    "    x : " << +x << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string STemplate< Member >::describe( size_t /*unused*/ ) const
{
    std::ostringstream ss;
    if( this->operator bool( ) )
    {
        ss << "Structure of size " << size_in_bytes( );
    }
    else
    {
        ss << "Uninitialized Structure " << name();
    }
    return ss.str( );
}
} // namespace n

namespace n {
namespace internal
{
const char* const X__schema__ =
"namespace n {\n"
    "archive X\n"
    "{\n"
    "    payload : raw_data;\n"
    "}\n"
    "}\n"
    "\n"
    "";
const char* const X__payload__schema__ =
"namespace n {\n"
    "archive X\n"
    "{\n"
    "    payload : raw_data;\n"
    "}\n"
    "}\n"
    "\n"
    "";
}
// -------------------------------------------------------------------------------------------------

inline const char*
X::schema_definition( )
{
    return internal::X__schema__;
}

inline const char*
X::name_definition( )
{
    return "X";
}

inline const char*
X::name( ) const
{
    return X::name_definition( );
}

inline const char*
X::schema( ) const
{
    return X::schema_definition( );
}

inline
X
X::open( std::shared_ptr< flatdata::ResourceStorage > storage )
{
    X result( storage );
    result.initialize( );
    return result;
}

inline
X::X( std::shared_ptr< flatdata::ResourceStorage > storage )
    : flatdata::Archive( storage )
{
}

inline bool
X::load_contents( )
{
    bool is_open = true;

    read_resource( is_open, m_payload, "payload", internal::X__payload__schema__ );
    return is_open;
}

inline void
X::describe_resources( std::ostream& stream, size_t nest_level ) const
{
    describe_resource( nest_level, stream, "payload", m_payload );
}

inline auto X::payload( ) const -> const PayloadType&
{
    return m_payload;
}


// -------------------------------------------------------------------------------------------------

inline const char*
XBuilder::schema_definition( )
{
    return internal::X__schema__;
}

inline const char*
XBuilder::name( ) const
{
    return "X";
}

inline const char*
XBuilder::schema( ) const
{
    return X::schema_definition( );
}

inline
XBuilder::XBuilder( std::shared_ptr< flatdata::ResourceStorage > storage )
    : flatdata::ArchiveBuilder( storage )
{
}


inline XBuilder
XBuilder::open(std::shared_ptr< flatdata::ResourceStorage > storage )
{
    XBuilder result( storage );
    if ( !result.initialize( ) )
    {
        return XBuilder( );
    }
    return result;
}

inline bool
XBuilder::set_payload( PayloadReaderType data )
{
    check_created( );
    return storage( ).write< PayloadReaderType >( "payload", internal::X__payload__schema__, data );
}

} // namespace n

namespace m {
namespace internal
{
    const char* const S__schema__ = R"schema(namespace m {
struct S
{
    x : u64 : 64;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
STemplate< Member >::STemplate( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
STemplate< Member >::STemplate( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
STemplate< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename STemplate< Member >::StreamType STemplate< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string STemplate< Member >::schema( ) { return internal::S__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string STemplate< Member >::name( ) { return "S"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t STemplate< Member >::size_in_bytes( ) { return 8; }

template< template < typename, int, int, int > class Member >
inline
bool STemplate< Member >::operator==( const STemplate& other ) const
{
    for ( size_t i = 0; i < size_in_bytes( ); i++ )
    {
        if ( _data.data[ i ] != other._data.data[ i ] )
        {
            return false;
        }
    }
    return true;
}

template< template < typename, int, int, int > class Member >
inline
bool STemplate< Member >::operator!=( const STemplate& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool STemplate< Member >::operator<( const STemplate& other ) const
{
return
    x < other.x;
}

template< template < typename, int, int, int > class Member >
inline
STemplate< Member >::operator STemplate< flatdata::Reader >( ) const
{
    return STemplate< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string STemplate< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "S {" << std::endl <<
    "    x : " << +x << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string STemplate< Member >::describe( size_t /*unused*/ ) const
{
    std::ostringstream ss;
    if( this->operator bool( ) )
    {
        ss << "Structure of size " << size_in_bytes( );
    }
    else
    {
        ss << "Uninitialized Structure " << name();
    }
    return ss.str( );
}
} // namespace m

namespace m {
namespace internal
{
const char* const X__schema__ =
"namespace m {\n"
    "archive X\n"
    "{\n"
    "    payload : raw_data;\n"
    "}\n"
    "}\n"
    "\n"
    "";
const char* const X__payload__schema__ =
"namespace m {\n"
    "archive X\n"
    "{\n"
    "    payload : raw_data;\n"
    "}\n"
    "}\n"
    "\n"
    "";
}
// -------------------------------------------------------------------------------------------------

inline const char*
X::schema_definition( )
{
    return internal::X__schema__;
}

inline const char*
X::name_definition( )
{
    return "X";
}

inline const char*
X::name( ) const
{
    return X::name_definition( );
}

inline const char*
X::schema( ) const
{
    return X::schema_definition( );
}

inline
X
X::open( std::shared_ptr< flatdata::ResourceStorage > storage )
{
    X result( storage );
    result.initialize( );
    return result;
}

inline
X::X( std::shared_ptr< flatdata::ResourceStorage > storage )
    : flatdata::Archive( storage )
{
}

inline bool
X::load_contents( )
{
    bool is_open = true;

    read_resource( is_open, m_payload, "payload", internal::X__payload__schema__ );
    return is_open;
}

inline void
X::describe_resources( std::ostream& stream, size_t nest_level ) const
{
    describe_resource( nest_level, stream, "payload", m_payload );
}

inline auto X::payload( ) const -> const PayloadType&
{
    return m_payload;
}


// -------------------------------------------------------------------------------------------------

inline const char*
XBuilder::schema_definition( )
{
    return internal::X__schema__;
}

inline const char*
XBuilder::name( ) const
{
    return "X";
}

inline const char*
XBuilder::schema( ) const
{
    return X::schema_definition( );
}

inline
XBuilder::XBuilder( std::shared_ptr< flatdata::ResourceStorage > storage )
    : flatdata::ArchiveBuilder( storage )
{
}


inline XBuilder
XBuilder::open(std::shared_ptr< flatdata::ResourceStorage > storage )
{
    XBuilder result( storage );
    if ( !result.initialize( ) )
    {
        return XBuilder( );
    }
    return result;
}

inline bool
XBuilder::set_payload( PayloadReaderType data )
{
    check_created( );
    return storage( ).write< PayloadReaderType >( "payload", internal::X__payload__schema__, data );
}

} // namespace m

namespace _builtin {
namespace multivector {
namespace internal
{
    const char* const IndexType32__schema__ = R"schema()schema";
}

template< template < typename, int, int, int > class Member >
inline
IndexType32Template< Member >::IndexType32Template( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
IndexType32Template< Member >::IndexType32Template( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
IndexType32Template< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename IndexType32Template< Member >::StreamType IndexType32Template< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string IndexType32Template< Member >::schema( ) { return internal::IndexType32__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string IndexType32Template< Member >::name( ) { return "IndexType32"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t IndexType32Template< Member >::size_in_bytes( ) { return 4; }

template< template < typename, int, int, int > class Member >
inline
bool IndexType32Template< Member >::operator==( const IndexType32Template& other ) const
{
    for ( size_t i = 0; i < size_in_bytes( ); i++ )
    {
        if ( _data.data[ i ] != other._data.data[ i ] )
        {
            return false;
        }
    }
    return true;
}

template< template < typename, int, int, int > class Member >
inline
bool IndexType32Template< Member >::operator!=( const IndexType32Template& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool IndexType32Template< Member >::operator<( const IndexType32Template& other ) const
{
return
    value < other.value;
}

template< template < typename, int, int, int > class Member >
inline
IndexType32Template< Member >::operator IndexType32Template< flatdata::Reader >( ) const
{
    return IndexType32Template< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string IndexType32Template< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "IndexType32 {" << std::endl <<
    "    value : " << +value << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string IndexType32Template< Member >::describe( size_t /*unused*/ ) const
{
    std::ostringstream ss;
    if( this->operator bool( ) )
    {
        ss << "Structure of size " << size_in_bytes( );
    }
    else
    {
        ss << "Uninitialized Structure " << name();
    }
    return ss.str( );
}
}} // namespace _builtin.multivector

namespace a {
namespace internal
{
const char* const A__schema__ =
"namespace n {\n"
    "struct S\n"
    "{\n"
    "    x : u64 : 64;\n"
    "}\n"
    "}\n"
    "\n"
    "namespace m {\n"
    "struct S\n"
    "{\n"
    "    x : u64 : 64;\n"
    "}\n"
    "}\n"
    "\n"
    "namespace n {\n"
    "archive X\n"
    "{\n"
    "    payload : raw_data;\n"
    "}\n"
    "}\n"
    "\n"
    "namespace a {\n"
    "archive A\n"
    "{\n"
    "    single : .n.S;\n"
    "    list : vector< .m.S >;\n"
    "    multi : multivector< 32, .n.S >;\n"
    "    inner : archive .n.X;\n"
    "}\n"
    "}\n"
    "\n"
    "";
const char* const A__single__schema__ =
"namespace n {\n"
    "struct S\n"
    "{\n"
    "    x : u64 : 64;\n"
    "}\n"
    "}\n"
    "\n"
    "namespace a {\n"
    "archive A\n"
    "{\n"
    "    single : .n.S;\n"
    "}\n"
    "}\n"
    "\n"
    "";
const char* const A__list__schema__ =
"namespace m {\n"
    "struct S\n"
    "{\n"
    "    x : u64 : 64;\n"
    "}\n"
    "}\n"
    "\n"
    "namespace a {\n"
    "archive A\n"
    "{\n"
    "    list : vector< .m.S >;\n"
    "}\n"
    "}\n"
    "\n"
    "";
const char* const A__multi__schema__ =
"namespace n {\n"
    "struct S\n"
    "{\n"
    "    x : u64 : 64;\n"
    "}\n"
    "}\n"
    "\n"
    "namespace a {\n"
    "archive A\n"
    "{\n"
    "    multi : multivector< 32, .n.S >;\n"
    "}\n"
    "}\n"
    "\n"
    "";
const char* const A__inner__schema__ =
"namespace n {\n"
    "archive X\n"
    "{\n"
    "    payload : raw_data;\n"
    "}\n"
    "}\n"
    "\n"
    "namespace a {\n"
    "archive A\n"
    "{\n"
    "    inner : archive .n.X;\n"
    "}\n"
    "}\n"
    "\n"
    "";
}
// -------------------------------------------------------------------------------------------------

inline const char*
A::schema_definition( )
{
    return internal::A__schema__;
}

inline const char*
A::name_definition( )
{
    return "A";
}

inline const char*
A::name( ) const
{
    return A::name_definition( );
}

inline const char*
A::schema( ) const
{
    return A::schema_definition( );
}

inline
A
A::open( std::shared_ptr< flatdata::ResourceStorage > storage )
{
    A result( storage );
    result.initialize( );
    return result;
}

inline
A::A( std::shared_ptr< flatdata::ResourceStorage > storage )
    : flatdata::Archive( storage )
{
}

inline bool
A::load_contents( )
{
    bool is_open = true;

    read_resource( is_open, m_single, "single", internal::A__single__schema__ );
    read_resource( is_open, m_list, "list", internal::A__list__schema__ );
    read_resource( is_open, m_multi, "multi", internal::A__multi__schema__ );
    load_archive( is_open, m_inner, "inner" );
    return is_open;
}

inline void
A::describe_resources( std::ostream& stream, size_t nest_level ) const
{
    describe_resource( nest_level, stream, "single", m_single );
    describe_resource( nest_level, stream, "list", m_list );
    describe_resource( nest_level, stream, "multi", m_multi );
    describe_resource( nest_level, stream, "inner", m_inner );
}

inline auto A::single( ) const -> const SingleType&
{
    return m_single;
}

inline auto A::list( ) const -> const ListType&
{
    return m_list;
}

inline auto A::multi( ) const -> const MultiType&
{
    return m_multi;
}

inline auto A::inner( ) const -> const InnerType&
{
    return m_inner;
}


// -------------------------------------------------------------------------------------------------

inline const char*
ABuilder::schema_definition( )
{
    return internal::A__schema__;
}

inline const char*
ABuilder::name( ) const
{
    return "A";
}

inline const char*
ABuilder::schema( ) const
{
    return A::schema_definition( );
}

inline
ABuilder::ABuilder( std::shared_ptr< flatdata::ResourceStorage > storage )
    : flatdata::ArchiveBuilder( storage )
{
}


inline ABuilder
ABuilder::open(std::shared_ptr< flatdata::ResourceStorage > storage )
{
    ABuilder result( storage );
    if ( !result.initialize( ) )
    {
        return ABuilder( );
    }
    return result;
}

inline bool
ABuilder::set_single( SingleReaderType data )
{
    check_created( );
    return storage( ).write< SingleReaderType >( "single", internal::A__single__schema__, data );
}
inline auto ABuilder::start_list( ) -> ListType
{
    check_created( );
    return storage( ).create_external_vector< ::m::S >( "list", internal::A__list__schema__ );
}
inline bool
ABuilder::set_list( ListReaderType data )
{
    check_created( );
    return storage( ).write< ListReaderType >( "list", internal::A__list__schema__, data );
}
inline auto ABuilder::start_multi( ) -> MultiType
{
    check_created( );
    return storage( ).create_multi_vector< ::_builtin::multivector::IndexType32, ::n::S >( "multi", internal::A__multi__schema__ );
}

inline auto
ABuilder::inner( ) -> InnerType&
{
    if ( !m_inner.is_open( ) )
    {
        m_inner = InnerType::open( this->storage( ).create_directory( "inner" ) );
    }
    return m_inner;
}
} // namespace a

