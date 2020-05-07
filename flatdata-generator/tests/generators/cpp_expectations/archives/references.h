inline bool
A::load_contents( )
{
    bool is_open = true;

    read_resource( is_open, m_list1, "list1", internal::A__list1__schema__ );
    is_open = is_open && ( !m_list1 | m_list1->size( ) <= 256 );
    read_resource( is_open, m_list2, "list2", internal::A__list2__schema__ );
    read_resource( is_open, m_multilist, "multilist", internal::A__multilist__schema__ );
    read_resource( is_open, m_refs, "refs", internal::A__refs__schema__ );
    read_resource( is_open, m_multirefs, "multirefs", internal::A__multirefs__schema__ );
    return is_open;
}

inline void
A::describe_resources( std::ostream& stream ) const
{
    describe_resource( stream, "list1", m_list1, m_list1 && m_list1->size( ) > 256 );
    describe_resource( stream, "list2", m_list2 );
    describe_resource( stream, "multilist", m_multilist );
    describe_resource( stream, "refs", m_refs );
    describe_resource( stream, "multirefs", m_multirefs );
}
