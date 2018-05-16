//! Module containing helper traits and macros.

/// Helper trait defining a constant for an integer type whether it is signed.
pub trait Int {
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

/// Intersperses a list of string literals with comma.
///
/// This macro takes any number of string literals, yielding an expression of type `&'static str`
/// which represents all literals concatenated with commas (,) in between.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate flatdata; fn main() {
/// let s = intersperse!("cat", "dog", "fish");
/// assert_eq!(s, "cat, dog, fish");
/// # }
/// ```
#[macro_export]
macro_rules! intersperse {
    () => ("");
    ($head:expr) => ($head);
    ($head:expr, $($tail:expr),+) => (concat!($head, ", ", intersperse!($($tail),*)));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_intersperse() {
        assert_eq!(intersperse!(), "");
        assert_eq!(intersperse!(""), "");
        assert_eq!(intersperse!("1", "2"), "1, 2");
        assert_eq!(intersperse!("1", "2", "3"), "1, 2, 3");
    }
}
