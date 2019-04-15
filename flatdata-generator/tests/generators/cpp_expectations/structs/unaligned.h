template< template < typename, int, int, int > class Member >
union U8Template
{
    using PaddingType = Member< uint64_t, 0, 3, 1 >;
    PaddingType padding;
    using FType = Member< uint8_t, 3, 5, 1 >;
    FType f;