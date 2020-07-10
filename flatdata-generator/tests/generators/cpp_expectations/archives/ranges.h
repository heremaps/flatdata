template< template < typename, int, int, int > class Member >
union STemplate
{
    using XType = Member< uint64_t, 0, 64, 10 >;
    XType x;
    using FirstYType = Member< uint32_t, 64, 14, 10 >;
    FirstYType first_y;
    using YRangeType = Member< std::pair< uint32_t, uint32_t >, 64, 14, 10 >;
    YRangeType y_range;

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

    static constexpr bool IS_OVERLAPPING_WITH_NEXT = true;

    /**
    * Private data member, should not be directly used.
    * Cannot be made private.
    * Please refer to C++ Standard, Chapter 9.2, Paragraph 19.
    * This union has to be kept standard-layout, which different access control prevents.
    */
    Member< uint32_t, 0, 0, 0 > _data;
};
