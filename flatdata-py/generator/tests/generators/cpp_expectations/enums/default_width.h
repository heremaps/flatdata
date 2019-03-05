template< template < typename, int, int > class Member >
union StructEnumI8Template
{
    using FType = Member< ::n::EnumI8, 0, 8 >;
    FType f;