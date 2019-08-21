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

    fn deref(&self) -> &[u8] {
        self.data
    }
}

impl<'a> RawData<'a> {
    /// Creates a new object from raw memory reference.
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Reads a \0 terminated substring starting at specified offset.
    pub fn substring(&self, start: usize) -> Result<&'a str, str::Utf8Error> {
        let suffix = &self.data[start..];
        match suffix.iter().position(|&c| c == 0) {
            Some(idx) => str::from_utf8(&suffix[..idx]),
            None => str::from_utf8(suffix),
        }
    }

    /// Reads a \0 terminated substring starting at specified offset, including invalid characters.
    pub fn substring_lossy(&self, start: usize) -> Cow<'a, str> {
        let suffix = &self.data[start..];
        match suffix.iter().position(|&c| c == 0) {
            Some(idx) => String::from_utf8_lossy(&suffix[..idx]),
            None => String::from_utf8_lossy(suffix),
        }
    }

    /// Reads a \0 terminated substring starting at specified offset without checking that the
    /// string contains valid UTF-8.
    pub unsafe fn substring_unchecked(&self, start: usize) -> &'a str {
        let suffix = &self.data[start..];
        match suffix.iter().position(|&c| c == 0) {
            Some(idx) => str::from_utf8_unchecked(&suffix[..idx]),
            None => str::from_utf8_unchecked(suffix),
        }
    }

    /// Converts RawData back into bytes.
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
        assert_eq!(unsafe { raw_data.substring_unchecked(0) }, "");
    }

    #[test]
    fn last_without_terminator() {
        let data: &[u8] = b"abc";
        let raw_data = RawData::new(data);
        assert_eq!(raw_data.substring(1), Ok("bc"));
        assert_eq!(raw_data.substring_lossy(1), "bc");
        assert_eq!(unsafe { raw_data.substring_unchecked(1) }, "bc");
    }

    #[test]
    fn until_terminator() {
        let data: &[u8] = b"ab\0c";
        let raw_data = RawData::new(data);
        assert_eq!(raw_data.substring(1), Ok("b"));
        assert_eq!(raw_data.substring_lossy(1), "b");
        assert_eq!(unsafe { raw_data.substring_unchecked(1) }, "b");
    }

    #[test]
    fn invalid_utf8() {
        let data: &[u8] = b"ab\xF0\x90\x80\0c";
        let raw_data = RawData::new(data);
        assert!(raw_data.substring(1).is_err());
        assert_eq!(raw_data.substring_lossy(1), "b�");
    }
}
