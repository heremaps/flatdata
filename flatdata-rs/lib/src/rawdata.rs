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
    /// Create a new object from raw memory reference
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    /// Read a \0 terminated substring starting at specified offset
    pub fn substring(&self, start: usize) -> Result<&'a str, std::str::Utf8Error> {
        let suffix = &self.data[start..];
        match suffix.iter().position(|&c| c == 0) {
            Some(idx) => std::str::from_utf8(&suffix[..idx]),
            None => std::str::from_utf8(suffix),
        }
    }

    /// Converts RawData back into bytes
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
    }

    #[test]
    fn last_without_terminator() {
        let data: &[u8] = b"abc";
        let raw_data = RawData::new(data);
        assert_eq!(raw_data.substring(1), Ok("bc"));
    }

    #[test]
    fn until_terminator() {
        let data: &[u8] = b"ab\0c";
        let raw_data = RawData::new(data);
        assert_eq!(raw_data.substring(1), Ok("b"));
    }
}
