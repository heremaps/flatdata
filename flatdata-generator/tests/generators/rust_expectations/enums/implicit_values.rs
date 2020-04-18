#[derive(Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Enum1 {
    // = 0
    Value1 = 0,
    // = 3
    Value2 = 3,
    // = 4
    Value3 = 4,
    // = 1
    Value4 = 1,
    // = 2
    Value5 = 2,
    #[doc(hidden)]
    UnknownValue5 = 5,
    #[doc(hidden)]
    UnknownValue6 = 6,
    #[doc(hidden)]
    UnknownValue7 = 7,
}

impl flatdata::helper::Int for Enum1 {
    const IS_SIGNED: bool = false;
}
}
