//! RFC4648 implemented, base16 encoding.
//!
//! Base 16 encoding is the standard case-insensitive hex encoding and
//! may be referred to as "base16" or "hex".
//!
//! # Examples
//!
//! Base16 encode basic usage:
//!
//! ```
//! use connx::encoding::base16;
//! let src = b"hello";
//! let dst = base16::encode_to_string(src);
//! assert_eq!(dst, "68656c6c6f".to_string());
//! ```
//!
//! Base16 decode basic usage:
//!
//! ```
//! use connx::encoding::base16;
//! let src = "68656c6c6f";
//! let dst = base16::decode_string(src).unwrap();
//! assert_eq!(dst, b"hello");
//! ```

/// Errors when base16 encode and decode
#[derive(Debug)]
pub enum Base16Error {
    InvalidByte(InvalidByteError),
    OddLength,
}

impl std::fmt::Display for Base16Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidByte(e) => e.fmt(f),
            Self::OddLength => write!(f, "encoding/base16: odd length hex string"),
        }
    }
}

impl std::error::Error for Base16Error {}

impl From<InvalidByteError> for Base16Error {
    fn from(e: InvalidByteError) -> Self {
        Self::InvalidByte(e)
    }
}

/// Error happens when pass invalid character to decode function
#[derive(Debug)]
pub struct InvalidByteError {
    b: u8,
}

impl InvalidByteError {
    pub fn new(b: u8) -> Self {
        Self { b }
    }
}

impl std::fmt::Display for InvalidByteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "encoding/base16: invalid byte: {}", self.b)
    }
}

impl std::error::Error for InvalidByteError {}

/// Base16 encoding map
pub const HEX_TABLE: &[u8] = b"0123456789abcdef";

/// Calculate base16 encoded string length
///
/// - @param n: raw bytes length
/// - @return: base16 encoded string length, (n*2)
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base16;
/// assert_eq!(base16::encode_len(0), 0);
/// assert_eq!(base16::encode_len(1), 2);
/// assert_eq!(base16::encode_len(2), 4);
/// assert_eq!(base16::encode_len(3), 6);
/// ```
#[inline]
pub fn encode_len(n: usize) -> usize {
    n * 2
}

/// Calculate base16 decoded data length
///
/// - @param n: base16 string length
/// - @return: decoded raw bytes length, (n/2)
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base16;
/// assert_eq!(base16::decode_len(0), 0);
/// assert_eq!(base16::decode_len(2), 1);
/// assert_eq!(base16::decode_len(4), 2);
/// assert_eq!(base16::decode_len(6), 3);
/// ```
#[inline]
pub fn decode_len(n: usize) -> usize {
    n / 2
}

/// Encode bytes to base16 bytes
///
/// - @param dst: encoded base16 bytes
/// - @param src: raw bytes
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base16;
/// let src = b"hello";
/// let dst_len = base16::encode_len(src.len());
/// let mut dst = vec![0; dst_len];
/// base16::encode(&mut dst, src);
/// assert_eq!(dst, b"68656c6c6f");
/// ```
pub fn encode(dst: &mut [u8], src: &[u8]) -> usize {
    src.iter().fold(0, |dst_idx, &src_byte| {
        dst[dst_idx] = HEX_TABLE[(src_byte >> 4) as usize];
        dst[dst_idx + 1] = HEX_TABLE[(src_byte & 0x0F) as usize];
        dst_idx + 2
    });
    src.len() * 2
}

fn from_hex_char(b: u8) -> Result<u8, InvalidByteError> {
    if b.is_ascii_digit() {
        Ok(b - b'0')
    } else if b.is_ascii_lowercase() {
        Ok(b - b'a' + 10)
    } else if b.is_ascii_uppercase() {
        Ok(b - b'A' + 10)
    } else {
        Err(InvalidByteError::new(b))
    }
}

/// Decode base16 bytes to raw bytes
///
/// - @param dst: decoded raw bytes
/// - @param src: base16 bytes
/// - @return: raw byte size if successfully decoded.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base16;
/// let src = b"68656c6c6f";
/// let mut dst = vec![0; base16::decode_len(src.len())];
/// let dst_size = base16::decode(&mut dst, src).unwrap();
/// assert_eq!(dst, b"hello");
/// ```
pub fn decode(dst: &mut [u8], src: &[u8]) -> Result<usize, Base16Error> {
    let mut src_idx = 1;
    let mut dst_idx = 0;
    while src_idx < src.len() {
        let a = from_hex_char(src[src_idx - 1])?;
        let b = from_hex_char(src[src_idx])?;
        dst[dst_idx] = (a << 4) | b;
        src_idx += 2;
        dst_idx += 1;
    }
    if src.len() % 2 == 1 {
        let _ = from_hex_char(src[src_idx - 1])?;
        return Err(Base16Error::OddLength);
    }
    Ok(dst_idx)
}

/// Encode bytes to base16 string
///
/// - @param src: raw bytes
/// - @return: encoded base16 string
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base16;
/// let src = b"hello";
/// let dst = base16::encode_to_string(src);
/// assert_eq!(dst, "68656c6c6f".to_string());
/// ```
pub fn encode_to_string(src: &[u8]) -> String {
    let mut dst = vec![0u8; encode_len(src.len())];
    encode(&mut dst, src);
    String::from_utf8(dst).unwrap()
}

/// Decode base16 string to raw bytes
///
/// - @param src: base16 bytes
/// - @return: decoded raw bytes
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base16;
/// let src = "68656c6c6f";
/// let dst = base16::decode_string(src).unwrap();
/// assert_eq!(dst, b"hello");
/// ```
pub fn decode_string(src: &str) -> Result<Vec<u8>, Base16Error> {
    let mut dst = vec![0u8; decode_len(src.len())];
    decode(&mut dst, src.as_bytes())?;
    Ok(dst)
}

#[cfg(test)]
mod tests_base16 {
    use super::*;

    fn test_base(data: &[u8], encoded_expect: &str) {
        let encoded = encode_to_string(data);
        assert_eq!(encoded, encoded_expect.to_string());
        let decoded = decode_string(&encoded).unwrap();
        assert_eq!(data, decoded);
    }

    #[test]
    fn test01() {
        test_base(&[], "");
        test_base(&[0, 1, 2, 3, 4, 5, 6, 7], "0001020304050607");
        test_base(&[8, 9, 10, 11, 12, 13, 14, 15], "08090a0b0c0d0e0f");
        test_base(
            &[0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7],
            "f0f1f2f3f4f5f6f7",
        );
        test_base(
            &[0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff],
            "f8f9fafbfcfdfeff",
        );
        test_base(&[b'g'], "67");
        test_base(&[0xe3, 0xa1], "e3a1");
    }

    #[test]
    fn test02() {
        test_base(b"", "");
        test_base(b"f", "66");
        test_base(b"fo", "666f");
        test_base(b"foo", "666f6f");
        test_base(b"foob", "666f6f62");
        test_base(b"fooba", "666f6f6261");
        test_base(b"foobar", "666f6f626172");
    }

    #[test]
    fn test03() {
        test_base(b"hello", "68656c6c6f");
    }
}
