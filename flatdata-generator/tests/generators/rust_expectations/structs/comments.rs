// This is a comment about Foo
define_struct!(
    Foo,
    RefFoo,
    RefMutFoo,
    schema::structs::FOO,
    16,
    (a, set_a, u64, u64, 0, 64),
    (b, set_b, u64, u64, 64, 64));

/// This is a comment about Bar
define_struct!(
    Bar,
    RefBar,
    RefMutBar,
    schema::structs::BAR,
    16,
    (a, set_a, u64, u64, 0, 64),
    (b, set_b, u64, u64, 64, 64));
}
