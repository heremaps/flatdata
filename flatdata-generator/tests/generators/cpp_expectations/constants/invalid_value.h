template< template < typename, int, int, int > class Member >
union BarTemplate
{
    using InvalidZeroType = Member< flatdata::Tagged< int8_t, ::n::INVALID_ZERO >, 0, 8, 3 >;
    InvalidZeroType invalid_zero;
    using InvalidMinIntType = Member< flatdata::Tagged< int8_t, ::n::INVALID_MIN_INT >, 8, 8, 3 >;
    InvalidMinIntType invalid_min_int;
    using InvalidMaxIntType = Member< flatdata::Tagged< int8_t, ::n::INVALID_MAX_INT >, 16, 8, 3 >;
    InvalidMaxIntType invalid_max_int;
