//! Module containing helper traits and macros.

/// Helper trait defining a constant for an integer type whether it is signed.
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

/// Intersperses a list of string literals with comma.
///
/// This macro takes any number of string literals, yielding an expression of
/// type `&'static str` which represents all literals concatenated with commas
/// (,) in between.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate flatdata; fn main() {
/// let s = intersperse!("cat", "dog", "fish");
/// assert_eq!(s, "cat, dog, fish");
/// # }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! intersperse {
    () => ("");
    ($head:expr) => ($head);
    ($head:expr, $($tail:expr),+) => (concat!($head, ", ", intersperse!($($tail),*)));
}

#[doc(hidden)]
#[macro_export]
macro_rules! masked {
    ($value:expr, $num_bits:expr) => {
        1u64.checked_shl($num_bits as u32)
            .map(|mask| $value & (mask - 1))
            .unwrap_or($value)
    };
}

// TODO: Ideally, this macro would compile to a const expression. Then we could
// use it for definition of constants.
#[doc(hidden)]
#[macro_export]
macro_rules! num_bytes {
    ($offset:expr, $num_bits:expr) => {
        if $num_bits + $offset % 8 < 64 {
            ($num_bits + $offset % 8 + 7) / 8
        } else {
            ($num_bits + 7) / 8
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! extend_sign {
    ($T:tt, $value:expr, $num_bits:expr) => {
        if <$T as $crate::helper::Int>::IS_SIGNED {
            let num_otherbits = (::std::mem::size_of::<$T>() * 8 - $num_bits) as u32;
            ($value as $T)
                .wrapping_shl(num_otherbits)
                .wrapping_shr(num_otherbits)
        } else {
            $value as $T
        }
    };
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
