'''
 Copyright (c) 2017 HERE Europe B.V.
 See the LICENSE file in the root of this project for license details.
'''

from generator.generators.CppGenerator import CppGenerator
from .assertions import *


def test_constants_are_declared_correctly():
    generate_and_assert_in("""
        namespace n{
        const i8 foo = 17;
        const u16 bar = 0x42;
        }
    """, CppGenerator, """
namespace n {
enum : uint16_t
{
    bar = 0x42
};
} // namespace n

namespace n {
enum : int8_t
{
    foo = 17
};
} // namespace n""")


def test_folded_namespaces_are_represented_correctly():
    generate_and_assert_in("""
        namespace n.nn{
        const i8 foo = 17;
        }
    """, CppGenerator, """
namespace n { namespace nn {
enum : int8_t
{
    foo = 17
};
}} // namespace n.nn
""")


def test_structures_are_declared_correctly():
    generate_and_assert_in("""
        namespace n{
        struct S {
            f0 : u8 : 3;
            f1 : u16 : 15;
        }
        }
    """, CppGenerator, """
namespace n {
template< template < typename, int, int > class Member >
union STemplate
{
    using F0Type = Member< uint8_t, 0, 3 >;
    F0Type f0;
    using F1Type = Member< uint16_t, 3, 15 >;
    F1Type f1;

    /// Stream type accepted by the class
    using StreamType = typename Member< uint32_t, 0, 0 >::StreamType;
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
    static size_t size_in_bytes( );

    bool operator==( const STemplate& other ) const;
    bool operator!=( const STemplate& other ) const;
    bool operator<( const STemplate& other ) const;
    operator STemplate< flatdata::Reader >( ) const;
    explicit operator bool( ) const;

    std::string to_string( ) const;
    std::string describe( ) const;

    /**
    * Private data member, should not be directly used.
    * Cannot be made private.
    * Please refer to C++ Standard, Chapter 9.2, Paragraph 19.
    * This union has to be kept standard-layout, which different access control prevents.
    */
    Member< uint32_t, 0, 0 > _data;
};

typedef STemplate< flatdata::Reader > S;
typedef STemplate< flatdata::Writer > SMutator;

} // namespace n
""")


def test_archives_are_declared_correctly():
    generate_and_assert_in("""
    namespace n{
    struct S {
        f0 : u8 : 3;
    }
    archive A {
        r0 : S;
    }
    }""", CppGenerator, """
namespace n {

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

    using R0Type = ::n::S;
    const R0Type& r0( ) const;


    const char* name( ) const override;
    const char* schema( ) const override;

private:
    explicit A( std::shared_ptr< flatdata::ResourceStorage > storage );

    bool load_contents( ) override;
    void describe_resources( std::ostream& stream ) const override;

private:
    R0Type m_r0;
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
    using R0Type = ::n::S;
    using R0ReaderType = ::n::S;
    bool set_r0( R0ReaderType data );



private:
    ABuilder( std::shared_ptr< flatdata::ResourceStorage > storage );

};

} // namespace n
""")


def test_object_resource_is_represented_correctly():
    expected_lines = [
        """using ObjectResourceType = ::n::S;""",
        """using ObjectResourceReaderType = ::n::S;""",
        """const ObjectResourceType& object_resource( ) const;""",
        """bool set_object_resource( ObjectResourceReaderType data );"""
    ]

    generate_and_assert_in("""
    namespace n{
    struct S {
        f0 : u8 : 3;
    }
    archive A {
        object_resource : S;
    }
    }
""", CppGenerator, *expected_lines)


def test_vector_resource_is_declared_correctly():
    expected_lines = [
        """using VectorResourceType = flatdata::ArrayView< ::n::T >; """,
        """using VectorResourceType = flatdata::ExternalVector< ::n::T >;""",
        """using VectorResourceReaderType = flatdata::ArrayView< ::n::T >; """,
        """const VectorResourceType& vector_resource( ) const;""",
        """VectorResourceType start_vector_resource( );""",
        """bool set_vector_resource( VectorResourceReaderType data );"""
    ]
    generate_and_assert_in("""
    namespace n{
    struct T {
        f0 : u8 : 3;
    }
    archive A {
        vector_resource : vector< T >;
    }
    }""", CppGenerator, *expected_lines)


def test_multi_vector_resource_is_declared_correctly():
    expected_lines = [
        """using MultivectorResourceType = flatdata::MultiArrayView<
        ::_builtin::multivector::IndexType33, ::n::T, ::n::U >;""",
        """using MultivectorResourceType = flatdata::MultiVector<
        ::_builtin::multivector::IndexType33, ::n::T, ::n::U >; """,
        """const MultivectorResourceType& multivector_resource( ) const;""",
        """MultivectorResourceType start_multivector_resource( );""",
        """return storage( ).create_multi_vector< ::_builtin::multivector::IndexType33,
        ::n::T, ::n::U >( "multivector_resource", internal::A__multivector_resource__schema__ );"""
    ]
    generate_and_assert_in("""
    namespace n{
    struct U {
        f0 : u8 : 3;
    }
    struct T {
        f0 : u8 : 3;
    }
    archive A {
        multivector_resource : multivector< 33, T, U >;
    }
    }""", CppGenerator, *expected_lines)


def test_raw_data_resource_is_declared_correctly():
    expected_lines = [
        """using RawDataResourceType = flatdata::MemoryDescriptor;""",
        """using RawDataResourceReaderType = flatdata::MemoryDescriptor;""",
        """const RawDataResourceType& raw_data_resource( ) const;""",
        """RawDataResourceType m_raw_data_resource;""",
        """bool set_raw_data_resource( RawDataResourceReaderType data );"""
    ]
    generate_and_assert_in("""
    namespace n{
    archive A {
        raw_data_resource : raw_data;
    }
    }""", CppGenerator, *expected_lines)


def test_structures_are_defined_correctly():
    generate_and_assert_in("""
    namespace n{
    struct S {
        f0 : u8 : 7;
        f1 : i16 : 13;
    }
    }
    """, CppGenerator, """
namespace n {
namespace internal
{
    const char* const S__schema__ = R"schema(namespace n { struct S {
        f0 : u8 : 7;
        f1 : i16 : 13;
    } })schema";
}

template< template < typename, int, int > class Member >
inline
STemplate< Member >::STemplate( )
: _data( Member< uint32_t, 0, 0 >{nullptr} )
{
}

template< template < typename, int, int > class Member >
inline
STemplate< Member >::STemplate( StreamType data )
: _data( Member< uint32_t, 0, 0 >{data} )
{
}

template< template < typename, int, int > class Member >
inline
STemplate< Member >::operator bool( ) const
{
return _data.data != nullptr;
}

template< template < typename, int, int > class Member >
inline
typename STemplate< Member >::StreamType STemplate< Member >::data( ) const { return _data.data; }

template< template < typename, int, int > class Member >
inline
std::string STemplate< Member >::schema( ) { return internal::S__schema__; }

template< template < typename, int, int > class Member >
inline
std::string STemplate< Member >::name( ) { return "S"; }

template< template < typename, int, int > class Member >
inline
size_t STemplate< Member >::size_in_bytes( ) { return 3; }

template< template < typename, int, int > class Member >
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

template< template < typename, int, int > class Member >
inline
bool STemplate< Member >::operator!=( const STemplate& other ) const
{
    return !( *this == other );
}

template< template < typename, int, int > class Member >
inline
bool STemplate< Member >::operator<( const STemplate& other ) const
{
return
    f0 < other.f0 &&
    f1 < other.f1 ;
}

template< template < typename, int, int > class Member >
inline
STemplate< Member >::operator STemplate< flatdata::Reader >( ) const
{
    return STemplate< flatdata::Reader >( _data.data );
}

template< template < typename, int, int > class Member >
inline
std::string STemplate< Member >::to_string( ) const
{
    std::ostringstream ss;
    ss << "{ " << std::endl <<
    "f0 : " << static_cast< uint64_t >( f0 ) << ", " << std::endl
    <<
    "f1 : " << static_cast< uint64_t >( f1 ) << ", " << std::endl
    << "}"
;
    return ss.str( );
}

template< template < typename, int, int > class Member >
inline
std::string STemplate< Member >::describe( ) const
{
    std::ostringstream ss;
    ss << "Structure of size " << size_in_bytes( );
    return ss.str( );
}
} // namespace n
""")


def test_archives_are_defined_correctly():
    generate_and_assert_in("""
    namespace n{
    struct S {
        f0 : u8 : 7;
    }
    archive A {
        r : S;
    }
    }
    """, CppGenerator, """
namespace n {
namespace internal
{
const char* const A__schema__ =
"namespace n { struct S {\\n"
    "        f0 : u8 : 7;\\n"
    "    } }\\n"
    "namespace n { archive A {\\n"
    "        r : S;\\n"
    "    } }";
const char* const A__r__schema__ =
"namespace n { struct S {\\n"
    "        f0 : u8 : 7;\\n"
    "    } }\\n"
    "namespace n { r : S; }";
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

    read_resource( is_open, m_r, "r", internal::A__r__schema__ );
    return is_open;
}

inline void
A::describe_resources( std::ostream& stream ) const
{
    describe_resource( stream, "r", m_r );
}

inline auto A::r( ) const -> const RType&
{
    return m_r;
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
ABuilder::set_r( RReaderType data )
{
    check_created( );
    return storage( ).write< RReaderType >( "r", internal::A__r__schema__, data );

}
} // namespace n
""")


def test_optional_resource_is_declared_correctly():
    expected_lines = [
        "boost::optional< RawDataResourceType > m_raw_data_resource;",
        "const boost::optional< RawDataResourceType >& raw_data_resource( ) const;",
        """
        inline auto A::raw_data_resource( ) const -> const boost::optional< RawDataResourceType >&
        {
            return m_raw_data_resource;
        }
        """
    ]
    generate_and_assert_in("""
    namespace n{
    archive A {
        @optional
        raw_data_resource : raw_data;
    }
    }""", CppGenerator, *expected_lines)
