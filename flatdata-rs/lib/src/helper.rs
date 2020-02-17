//! Module which contains internal helper traits.

/// Helper trait which defines a constant for an integer type whether it is signed.
pub trait Int {
    /// `true` if the implementing type is signed, otherwise `false`.
    const IS_SIGNED: bool;
}

impl Int for bool {
    const IS_SIGNED: bool = false;
}

impl Int for i8 {
    const IS_SIGNED: bool = true;
}

impl Int for u8 {
    const IS_SIGNED: bool = false;
}

impl Int for i16 {
    const IS_SIGNED: bool = true;
}

impl Int for u16 {
    const IS_SIGNED: bool = false;
}

impl Int for i32 {
    const IS_SIGNED: bool = true;
}

impl Int for u32 {
    const IS_SIGNED: bool = false;
}

impl Int for i64 {
    const IS_SIGNED: bool = true;
}

impl Int for u64 {
    const IS_SIGNED: bool = false;
}
