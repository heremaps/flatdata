pub mod n {

#[doc(hidden)]
pub mod schema {
pub mod structs {
}

}
pub const FOO: i8 = 0;

pub const FOO2: i8 = 10;
}

#[allow(missing_docs)]
pub mod m {

#[doc(hidden)]
pub mod schema {
pub mod structs {
pub const BAR: &str = r#"namespace m {
const i8 FOO = 1;
}

namespace n {
const i8 FOO = 0;
}

namespace m {
struct Bar
{
    @const( .m.FOO )
    foo1 : i8 : 8;
    @const( .n.FOO )
    foo2 : i8 : 8;
    @const( .m.FOO )
    foo3 : i8 : 8;
}
}

"#;
}

}
pub const FOO: i8 = 1;
