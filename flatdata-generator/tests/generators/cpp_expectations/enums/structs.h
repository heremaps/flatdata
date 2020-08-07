
#pragma once

#include <flatdata/flatdata.h>
#include <cstdint>
#include <iostream>
#include <iomanip>

namespace n {

enum class EnumI8 : int8_t
{
    VALUE = 0,
    UNKNOWN_VALUE_MINUS_1 = -1
};

inline
const char* to_string( EnumI8 value );


} // namespace n

namespace n {


template< template < typename, int, int, int > class Member >
union StructEnumI8Template
{
    using FType = Member< ::n::EnumI8, 0, 1, 1 >;
    FType f;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = StructEnumI8Template< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = StructEnumI8Template< flatdata::Reader >;

    StructEnumI8Template( );
    explicit StructEnumI8Template( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const StructEnumI8Template& other ) const;
    bool operator!=( const StructEnumI8Template& other ) const;
    bool operator<( const StructEnumI8Template& other ) const;
    operator StructEnumI8Template< flatdata::Reader >( ) const;
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


typedef StructEnumI8Template< flatdata::Reader > StructEnumI8;
typedef StructEnumI8Template< flatdata::Writer > StructEnumI8Mutator;

} // namespace n

namespace n {

enum class EnumU8 : uint8_t
{
    VALUE = 0,
    UNKNOWN_VALUE_1 = 1
};

inline
const char* to_string( EnumU8 value );


} // namespace n

namespace n {


template< template < typename, int, int, int > class Member >
union StructEnumU8Template
{
    using FType = Member< ::n::EnumU8, 0, 1, 1 >;
    FType f;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = StructEnumU8Template< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = StructEnumU8Template< flatdata::Reader >;

    StructEnumU8Template( );
    explicit StructEnumU8Template( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const StructEnumU8Template& other ) const;
    bool operator!=( const StructEnumU8Template& other ) const;
    bool operator<( const StructEnumU8Template& other ) const;
    operator StructEnumU8Template< flatdata::Reader >( ) const;
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


typedef StructEnumU8Template< flatdata::Reader > StructEnumU8;
typedef StructEnumU8Template< flatdata::Writer > StructEnumU8Mutator;

} // namespace n

namespace n {

enum class EnumI16 : int16_t
{
    VALUE = 0,
    UNKNOWN_VALUE_MINUS_1 = -1
};

inline
const char* to_string( EnumI16 value );


} // namespace n

namespace n {


template< template < typename, int, int, int > class Member >
union StructEnumI16Template
{
    using FType = Member< ::n::EnumI16, 0, 1, 1 >;
    FType f;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = StructEnumI16Template< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = StructEnumI16Template< flatdata::Reader >;

    StructEnumI16Template( );
    explicit StructEnumI16Template( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const StructEnumI16Template& other ) const;
    bool operator!=( const StructEnumI16Template& other ) const;
    bool operator<( const StructEnumI16Template& other ) const;
    operator StructEnumI16Template< flatdata::Reader >( ) const;
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


typedef StructEnumI16Template< flatdata::Reader > StructEnumI16;
typedef StructEnumI16Template< flatdata::Writer > StructEnumI16Mutator;

} // namespace n

namespace n {

enum class EnumU16 : uint16_t
{
    VALUE = 0,
    UNKNOWN_VALUE_1 = 1
};

inline
const char* to_string( EnumU16 value );


} // namespace n

namespace n {


template< template < typename, int, int, int > class Member >
union StructEnumU16Template
{
    using FType = Member< ::n::EnumU16, 0, 1, 1 >;
    FType f;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = StructEnumU16Template< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = StructEnumU16Template< flatdata::Reader >;

    StructEnumU16Template( );
    explicit StructEnumU16Template( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const StructEnumU16Template& other ) const;
    bool operator!=( const StructEnumU16Template& other ) const;
    bool operator<( const StructEnumU16Template& other ) const;
    operator StructEnumU16Template< flatdata::Reader >( ) const;
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


typedef StructEnumU16Template< flatdata::Reader > StructEnumU16;
typedef StructEnumU16Template< flatdata::Writer > StructEnumU16Mutator;

} // namespace n

namespace n {

enum class EnumI32 : int32_t
{
    VALUE = 0,
    UNKNOWN_VALUE_MINUS_1 = -1
};

inline
const char* to_string( EnumI32 value );


} // namespace n

namespace n {


template< template < typename, int, int, int > class Member >
union StructEnumI32Template
{
    using FType = Member< ::n::EnumI32, 0, 1, 1 >;
    FType f;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = StructEnumI32Template< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = StructEnumI32Template< flatdata::Reader >;

    StructEnumI32Template( );
    explicit StructEnumI32Template( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const StructEnumI32Template& other ) const;
    bool operator!=( const StructEnumI32Template& other ) const;
    bool operator<( const StructEnumI32Template& other ) const;
    operator StructEnumI32Template< flatdata::Reader >( ) const;
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


typedef StructEnumI32Template< flatdata::Reader > StructEnumI32;
typedef StructEnumI32Template< flatdata::Writer > StructEnumI32Mutator;

} // namespace n

namespace n {

enum class EnumU32 : uint32_t
{
    VALUE = 0,
    UNKNOWN_VALUE_1 = 1
};

inline
const char* to_string( EnumU32 value );


} // namespace n

namespace n {


template< template < typename, int, int, int > class Member >
union StructEnumU32Template
{
    using FType = Member< ::n::EnumU32, 0, 1, 1 >;
    FType f;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = StructEnumU32Template< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = StructEnumU32Template< flatdata::Reader >;

    StructEnumU32Template( );
    explicit StructEnumU32Template( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const StructEnumU32Template& other ) const;
    bool operator!=( const StructEnumU32Template& other ) const;
    bool operator<( const StructEnumU32Template& other ) const;
    operator StructEnumU32Template< flatdata::Reader >( ) const;
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


typedef StructEnumU32Template< flatdata::Reader > StructEnumU32;
typedef StructEnumU32Template< flatdata::Writer > StructEnumU32Mutator;

} // namespace n

namespace n {

enum class EnumI64 : int64_t
{
    VALUE = 0,
    UNKNOWN_VALUE_MINUS_1 = -1
};

inline
const char* to_string( EnumI64 value );


} // namespace n

namespace n {


template< template < typename, int, int, int > class Member >
union StructEnumI64Template
{
    using FType = Member< ::n::EnumI64, 0, 1, 1 >;
    FType f;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = StructEnumI64Template< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = StructEnumI64Template< flatdata::Reader >;

    StructEnumI64Template( );
    explicit StructEnumI64Template( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const StructEnumI64Template& other ) const;
    bool operator!=( const StructEnumI64Template& other ) const;
    bool operator<( const StructEnumI64Template& other ) const;
    operator StructEnumI64Template< flatdata::Reader >( ) const;
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


typedef StructEnumI64Template< flatdata::Reader > StructEnumI64;
typedef StructEnumI64Template< flatdata::Writer > StructEnumI64Mutator;

} // namespace n

namespace n {

enum class EnumU64 : uint64_t
{
    VALUE = 0,
    UNKNOWN_VALUE_1 = 1
};

inline
const char* to_string( EnumU64 value );


} // namespace n

namespace n {


template< template < typename, int, int, int > class Member >
union StructEnumU64Template
{
    using FType = Member< ::n::EnumU64, 0, 1, 1 >;
    FType f;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0, 0 >::StreamType;
    /// Mutable structure type
    using MutatorType = StructEnumU64Template< flatdata::Writer >;
    /// Immutable structure type
    using AccessorType = StructEnumU64Template< flatdata::Reader >;

    StructEnumU64Template( );
    explicit StructEnumU64Template( StreamType data );

    /// Get raw data stream
    StreamType data( ) const;
    /// Get structure schema
    static std::string schema( );
    /// Get structure name
    static std::string name( );
    /// Get structure size in bytes
    static constexpr size_t size_in_bytes( );

    bool operator==( const StructEnumU64Template& other ) const;
    bool operator!=( const StructEnumU64Template& other ) const;
    bool operator<( const StructEnumU64Template& other ) const;
    operator StructEnumU64Template< flatdata::Reader >( ) const;
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


typedef StructEnumU64Template< flatdata::Reader > StructEnumU64;
typedef StructEnumU64Template< flatdata::Writer > StructEnumU64Mutator;

} // namespace n


// -------------------------------------------------------------------------------------------------
// -------------------------------------- Implementations ------------------------------------------
// -------------------------------------------------------------------------------------------------

namespace n {

inline
const char* to_string( EnumI8 value )
{
    switch( value )
    {
    case EnumI8::VALUE:
        return "EnumI8::VALUE";
    case EnumI8::UNKNOWN_VALUE_MINUS_1:
        return "EnumI8::UNKNOWN_VALUE_MINUS_1";
    default:
        // default needed since C++ allows storage of unknown values
        return "Unknown value of EnumI8";
    }
}

} // namespace n

namespace n {
namespace internal
{
    const char* const StructEnumI8__schema__ = R"schema(namespace n {
enum EnumI8 : i8 : 1
{
    VALUE = 0,
}
}

namespace n {
struct StructEnumI8
{
    f : .n.EnumI8 : 1;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI8Template< Member >::StructEnumI8Template( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI8Template< Member >::StructEnumI8Template( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI8Template< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename StructEnumI8Template< Member >::StreamType StructEnumI8Template< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI8Template< Member >::schema( ) { return internal::StructEnumI8__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI8Template< Member >::name( ) { return "StructEnumI8"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t StructEnumI8Template< Member >::size_in_bytes( ) { return 1; }

template< template < typename, int, int, int > class Member >
inline
bool StructEnumI8Template< Member >::operator==( const StructEnumI8Template& other ) const
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
bool StructEnumI8Template< Member >::operator!=( const StructEnumI8Template& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool StructEnumI8Template< Member >::operator<( const StructEnumI8Template& other ) const
{
return
    f < other.f;
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI8Template< Member >::operator StructEnumI8Template< flatdata::Reader >( ) const
{
    return StructEnumI8Template< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI8Template< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "StructEnumI8 {" << std::endl <<
    "    f : " << +f << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI8Template< Member >::describe( size_t /*unused*/ ) const
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

inline
const char* to_string( EnumU8 value )
{
    switch( value )
    {
    case EnumU8::VALUE:
        return "EnumU8::VALUE";
    case EnumU8::UNKNOWN_VALUE_1:
        return "EnumU8::UNKNOWN_VALUE_1";
    default:
        // default needed since C++ allows storage of unknown values
        return "Unknown value of EnumU8";
    }
}

} // namespace n

namespace n {
namespace internal
{
    const char* const StructEnumU8__schema__ = R"schema(namespace n {
enum EnumU8 : u8 : 1
{
    VALUE = 0,
}
}

namespace n {
struct StructEnumU8
{
    f : .n.EnumU8 : 1;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU8Template< Member >::StructEnumU8Template( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU8Template< Member >::StructEnumU8Template( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU8Template< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename StructEnumU8Template< Member >::StreamType StructEnumU8Template< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU8Template< Member >::schema( ) { return internal::StructEnumU8__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU8Template< Member >::name( ) { return "StructEnumU8"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t StructEnumU8Template< Member >::size_in_bytes( ) { return 1; }

template< template < typename, int, int, int > class Member >
inline
bool StructEnumU8Template< Member >::operator==( const StructEnumU8Template& other ) const
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
bool StructEnumU8Template< Member >::operator!=( const StructEnumU8Template& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool StructEnumU8Template< Member >::operator<( const StructEnumU8Template& other ) const
{
return
    f < other.f;
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU8Template< Member >::operator StructEnumU8Template< flatdata::Reader >( ) const
{
    return StructEnumU8Template< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU8Template< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "StructEnumU8 {" << std::endl <<
    "    f : " << +f << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU8Template< Member >::describe( size_t /*unused*/ ) const
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

inline
const char* to_string( EnumI16 value )
{
    switch( value )
    {
    case EnumI16::VALUE:
        return "EnumI16::VALUE";
    case EnumI16::UNKNOWN_VALUE_MINUS_1:
        return "EnumI16::UNKNOWN_VALUE_MINUS_1";
    default:
        // default needed since C++ allows storage of unknown values
        return "Unknown value of EnumI16";
    }
}

} // namespace n

namespace n {
namespace internal
{
    const char* const StructEnumI16__schema__ = R"schema(namespace n {
enum EnumI16 : i16 : 1
{
    VALUE = 0,
}
}

namespace n {
struct StructEnumI16
{
    f : .n.EnumI16 : 1;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI16Template< Member >::StructEnumI16Template( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI16Template< Member >::StructEnumI16Template( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI16Template< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename StructEnumI16Template< Member >::StreamType StructEnumI16Template< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI16Template< Member >::schema( ) { return internal::StructEnumI16__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI16Template< Member >::name( ) { return "StructEnumI16"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t StructEnumI16Template< Member >::size_in_bytes( ) { return 1; }

template< template < typename, int, int, int > class Member >
inline
bool StructEnumI16Template< Member >::operator==( const StructEnumI16Template& other ) const
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
bool StructEnumI16Template< Member >::operator!=( const StructEnumI16Template& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool StructEnumI16Template< Member >::operator<( const StructEnumI16Template& other ) const
{
return
    f < other.f;
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI16Template< Member >::operator StructEnumI16Template< flatdata::Reader >( ) const
{
    return StructEnumI16Template< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI16Template< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "StructEnumI16 {" << std::endl <<
    "    f : " << +f << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI16Template< Member >::describe( size_t /*unused*/ ) const
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

inline
const char* to_string( EnumU16 value )
{
    switch( value )
    {
    case EnumU16::VALUE:
        return "EnumU16::VALUE";
    case EnumU16::UNKNOWN_VALUE_1:
        return "EnumU16::UNKNOWN_VALUE_1";
    default:
        // default needed since C++ allows storage of unknown values
        return "Unknown value of EnumU16";
    }
}

} // namespace n

namespace n {
namespace internal
{
    const char* const StructEnumU16__schema__ = R"schema(namespace n {
enum EnumU16 : u16 : 1
{
    VALUE = 0,
}
}

namespace n {
struct StructEnumU16
{
    f : .n.EnumU16 : 1;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU16Template< Member >::StructEnumU16Template( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU16Template< Member >::StructEnumU16Template( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU16Template< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename StructEnumU16Template< Member >::StreamType StructEnumU16Template< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU16Template< Member >::schema( ) { return internal::StructEnumU16__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU16Template< Member >::name( ) { return "StructEnumU16"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t StructEnumU16Template< Member >::size_in_bytes( ) { return 1; }

template< template < typename, int, int, int > class Member >
inline
bool StructEnumU16Template< Member >::operator==( const StructEnumU16Template& other ) const
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
bool StructEnumU16Template< Member >::operator!=( const StructEnumU16Template& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool StructEnumU16Template< Member >::operator<( const StructEnumU16Template& other ) const
{
return
    f < other.f;
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU16Template< Member >::operator StructEnumU16Template< flatdata::Reader >( ) const
{
    return StructEnumU16Template< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU16Template< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "StructEnumU16 {" << std::endl <<
    "    f : " << +f << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU16Template< Member >::describe( size_t /*unused*/ ) const
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

inline
const char* to_string( EnumI32 value )
{
    switch( value )
    {
    case EnumI32::VALUE:
        return "EnumI32::VALUE";
    case EnumI32::UNKNOWN_VALUE_MINUS_1:
        return "EnumI32::UNKNOWN_VALUE_MINUS_1";
    default:
        // default needed since C++ allows storage of unknown values
        return "Unknown value of EnumI32";
    }
}

} // namespace n

namespace n {
namespace internal
{
    const char* const StructEnumI32__schema__ = R"schema(namespace n {
enum EnumI32 : i32 : 1
{
    VALUE = 0,
}
}

namespace n {
struct StructEnumI32
{
    f : .n.EnumI32 : 1;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI32Template< Member >::StructEnumI32Template( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI32Template< Member >::StructEnumI32Template( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI32Template< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename StructEnumI32Template< Member >::StreamType StructEnumI32Template< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI32Template< Member >::schema( ) { return internal::StructEnumI32__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI32Template< Member >::name( ) { return "StructEnumI32"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t StructEnumI32Template< Member >::size_in_bytes( ) { return 1; }

template< template < typename, int, int, int > class Member >
inline
bool StructEnumI32Template< Member >::operator==( const StructEnumI32Template& other ) const
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
bool StructEnumI32Template< Member >::operator!=( const StructEnumI32Template& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool StructEnumI32Template< Member >::operator<( const StructEnumI32Template& other ) const
{
return
    f < other.f;
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI32Template< Member >::operator StructEnumI32Template< flatdata::Reader >( ) const
{
    return StructEnumI32Template< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI32Template< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "StructEnumI32 {" << std::endl <<
    "    f : " << +f << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI32Template< Member >::describe( size_t /*unused*/ ) const
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

inline
const char* to_string( EnumU32 value )
{
    switch( value )
    {
    case EnumU32::VALUE:
        return "EnumU32::VALUE";
    case EnumU32::UNKNOWN_VALUE_1:
        return "EnumU32::UNKNOWN_VALUE_1";
    default:
        // default needed since C++ allows storage of unknown values
        return "Unknown value of EnumU32";
    }
}

} // namespace n

namespace n {
namespace internal
{
    const char* const StructEnumU32__schema__ = R"schema(namespace n {
enum EnumU32 : u32 : 1
{
    VALUE = 0,
}
}

namespace n {
struct StructEnumU32
{
    f : .n.EnumU32 : 1;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU32Template< Member >::StructEnumU32Template( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU32Template< Member >::StructEnumU32Template( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU32Template< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename StructEnumU32Template< Member >::StreamType StructEnumU32Template< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU32Template< Member >::schema( ) { return internal::StructEnumU32__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU32Template< Member >::name( ) { return "StructEnumU32"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t StructEnumU32Template< Member >::size_in_bytes( ) { return 1; }

template< template < typename, int, int, int > class Member >
inline
bool StructEnumU32Template< Member >::operator==( const StructEnumU32Template& other ) const
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
bool StructEnumU32Template< Member >::operator!=( const StructEnumU32Template& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool StructEnumU32Template< Member >::operator<( const StructEnumU32Template& other ) const
{
return
    f < other.f;
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU32Template< Member >::operator StructEnumU32Template< flatdata::Reader >( ) const
{
    return StructEnumU32Template< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU32Template< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "StructEnumU32 {" << std::endl <<
    "    f : " << +f << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU32Template< Member >::describe( size_t /*unused*/ ) const
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

inline
const char* to_string( EnumI64 value )
{
    switch( value )
    {
    case EnumI64::VALUE:
        return "EnumI64::VALUE";
    case EnumI64::UNKNOWN_VALUE_MINUS_1:
        return "EnumI64::UNKNOWN_VALUE_MINUS_1";
    default:
        // default needed since C++ allows storage of unknown values
        return "Unknown value of EnumI64";
    }
}

} // namespace n

namespace n {
namespace internal
{
    const char* const StructEnumI64__schema__ = R"schema(namespace n {
enum EnumI64 : i64 : 1
{
    VALUE = 0,
}
}

namespace n {
struct StructEnumI64
{
    f : .n.EnumI64 : 1;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI64Template< Member >::StructEnumI64Template( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI64Template< Member >::StructEnumI64Template( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI64Template< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename StructEnumI64Template< Member >::StreamType StructEnumI64Template< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI64Template< Member >::schema( ) { return internal::StructEnumI64__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI64Template< Member >::name( ) { return "StructEnumI64"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t StructEnumI64Template< Member >::size_in_bytes( ) { return 1; }

template< template < typename, int, int, int > class Member >
inline
bool StructEnumI64Template< Member >::operator==( const StructEnumI64Template& other ) const
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
bool StructEnumI64Template< Member >::operator!=( const StructEnumI64Template& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool StructEnumI64Template< Member >::operator<( const StructEnumI64Template& other ) const
{
return
    f < other.f;
}

template< template < typename, int, int, int > class Member >
inline
StructEnumI64Template< Member >::operator StructEnumI64Template< flatdata::Reader >( ) const
{
    return StructEnumI64Template< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI64Template< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "StructEnumI64 {" << std::endl <<
    "    f : " << +f << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumI64Template< Member >::describe( size_t /*unused*/ ) const
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

inline
const char* to_string( EnumU64 value )
{
    switch( value )
    {
    case EnumU64::VALUE:
        return "EnumU64::VALUE";
    case EnumU64::UNKNOWN_VALUE_1:
        return "EnumU64::UNKNOWN_VALUE_1";
    default:
        // default needed since C++ allows storage of unknown values
        return "Unknown value of EnumU64";
    }
}

} // namespace n

namespace n {
namespace internal
{
    const char* const StructEnumU64__schema__ = R"schema(namespace n {
enum EnumU64 : u64 : 1
{
    VALUE = 0,
}
}

namespace n {
struct StructEnumU64
{
    f : .n.EnumU64 : 1;
}
}

)schema";
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU64Template< Member >::StructEnumU64Template( )
: _data( Member< uint32_t, 0, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU64Template< Member >::StructEnumU64Template( StreamType data )
: _data( Member< uint32_t, 0, 0, 0 >{data} )
{
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU64Template< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int, int > class Member >
inline
typename StructEnumU64Template< Member >::StreamType StructEnumU64Template< Member >::data( ) const { return _data.data; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU64Template< Member >::schema( ) { return internal::StructEnumU64__schema__; }

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU64Template< Member >::name( ) { return "StructEnumU64"; }

template< template < typename, int, int, int > class Member >
inline
constexpr size_t StructEnumU64Template< Member >::size_in_bytes( ) { return 1; }

template< template < typename, int, int, int > class Member >
inline
bool StructEnumU64Template< Member >::operator==( const StructEnumU64Template& other ) const
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
bool StructEnumU64Template< Member >::operator!=( const StructEnumU64Template& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int, int > class Member >
inline
bool StructEnumU64Template< Member >::operator<( const StructEnumU64Template& other ) const
{
return
    f < other.f;
}

template< template < typename, int, int, int > class Member >
inline
StructEnumU64Template< Member >::operator StructEnumU64Template< flatdata::Reader >( ) const
{
    return StructEnumU64Template< flatdata::Reader >( _data.data );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU64Template< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "StructEnumU64 {" << std::endl <<
    "    f : " << +f << "," << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int, int > class Member >
inline
std::string StructEnumU64Template< Member >::describe( size_t /*unused*/ ) const
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

