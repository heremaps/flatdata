use std::borrow::Cow;
use std::str;

/// Exposes blocks of raw data, providing auxiliary functionality like
/// extracting substrings.
#[derive(Debug, Clone, Copy)]
pub struct RawData<'a> {
    data: &'a [u8],
}

impl<'a> std::ops::Deref for RawData<'a> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &[u8] {
        self.data
    }
}

impl<'a> RawData<'a> {
    /// Creates a new object from raw memory reference.
    #[inline]
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Reads a \0 terminated substring starting at specified offset.
    #[inline]
    pub fn substring(&self, start: usize) -> Result<&'a str, str::Utf8Error> {
        self.substring_with(start, str::from_utf8)
    }

    /// Reads a \0 terminated substring starting at specified offset, including invalid characters.
    #[inline]
    pub fn substring_lossy(&self, start: usize) -> Cow<'a, str> {
        self.substring_with(start, String::from_utf8_lossy)
    }

    /// Reads a \0 terminated substring starting at specified offset as raw bytes.
    #[inline]
    pub fn substring_raw(&self, start: usize) -> &'a [u8] {
        self.substring_with(start, std::convert::identity)
    }

    /// Reads a \0 terminated substring starting at specified offset without checking that the
    /// string contains valid UTF-8.
    ///
    /// # Safety
    /// Same as str::from_utf8_unchecked
    #[inline]
    pub unsafe fn substring_unchecked(&self, start: usize) -> &'a str {
        self.substring_with(start, |bytes| str::from_utf8_unchecked(bytes))
    }

    fn substring_with<T>(&self, start: usize, f: impl FnOnce(&'a [u8]) -> T) -> T {
        let suffix = &self.data[start..];
        match suffix.iter().position(|&c| c == 0) {
            Some(idx) => f(&suffix[..idx]),
            None => f(suffix),
        }
    }

    /// Converts RawData back into bytes.
    #[inline]
    pub fn as_bytes(&self) -> &'a [u8] {
        self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let data: &[u8] = b"";
        let raw_data = RawData::new(data);
        assert_eq!(raw_data.substring(0), Ok(""));
        assert_eq!(raw_data.substring_lossy(0), "");
        assert_eq!(raw_data.substring_raw(0), b"");
        assert_eq!(unsafe { raw_data.substring_unchecked(0) }, "");
    }

    #[test]
    fn last_without_terminator() {
        let data: &[u8] = b"abc";
        let raw_data = RawData::new(data);
        assert_eq!(raw_data.substring(1), Ok("bc"));
        assert_eq!(raw_data.substring_lossy(1), "bc");
        assert_eq!(raw_data.substring_raw(1), b"bc");
        assert_eq!(unsafe { raw_data.substring_unchecked(1) }, "bc");
    }

    #[test]
    fn until_terminator() {
        let data: &[u8] = b"ab\0c";
        let raw_data = RawData::new(data);
        assert_eq!(raw_data.substring(1), Ok("b"));
        assert_eq!(raw_data.substring_lossy(1), "b");
        assert_eq!(raw_data.substring_raw(1), b"b");
        assert_eq!(unsafe { raw_data.substring_unchecked(1) }, "b");
    }

    #[test]
    fn invalid_utf8() {
        let data: &[u8] = b"ab\xF0\x90\x80\0c";
        let raw_data = RawData::new(data);
        assert!(raw_data.substring(1).is_err());
        assert_eq!(raw_data.substring_lossy(1), "b�");
        assert_eq!(raw_data.substring_raw(1), b"b\xF0\x90\x80");
    }
}
