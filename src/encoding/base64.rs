//! RFC4648 implemented, base64 encoding.
//!
//! # Examples
//!
//! Base64 encode basic usage:
//!
//! ```
//! use connx::encoding::base64;
//! let encoded_bytes: Vec<u8> = base64::encode_bytes(b"hello");
//! assert_eq!(encoded_bytes, b"aGVsbG8=");
//! ```
//!
//! Base64 decode basic usage:
//!
//! ```
//! use connx::encoding::base64;
//! assert_eq!(base64::decode_str("aGVsbG8="), Ok((b"hello").to_vec()));
//! ```

/// Standard encoding map from RFC4648
pub const ENCODE_STD: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// URL and filename encoding map from RFC4648
pub const ENCODE_URL: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Standard padding char from RFC4648
pub const PAD_CHAR: u8 = b'=';

/// Standard decoding map
pub const DECODE_STD_MAP: &[u8] = &[
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3e, 0xff, 0xff, 0xff, 0x3f,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

/// URL and filepath decoding map
pub const DECODE_URL_MAP: &[u8] = &[
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x3e, 0xff, 0xff,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0xff, 0xff, 0xff, 0xff, 0x3f,
    0xff, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

/// Errors when base64 encode and decode
#[derive(Debug, PartialEq)]
pub enum Base64Error {
    InvalidByte(InvalidByteError),
    InvalidLength,
}

impl std::fmt::Display for Base64Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidByte(e) => e.fmt(f),
            Self::InvalidLength => write!(f, "encoding/base64: invalid input length"),
        }
    }
}

impl std::error::Error for Base64Error {}

impl From<InvalidByteError> for Base64Error {
    fn from(e: InvalidByteError) -> Self {
        Self::InvalidByte(e)
    }
}

/// Error happens when pass invalid character to decode function
#[derive(Debug, PartialEq)]
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
        write!(f, "encoding/base64: invalid byte: {}", self.b)
    }
}

impl std::error::Error for InvalidByteError {}

/// Calculate base64 encoded string length
///
/// - @param src: raw string
/// - @return: base64 encoded string length
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// assert_eq!(base64::encode_str_len(""), 0);
/// assert_eq!(base64::encode_str_len("f"), 4); // will be encoded as 4 byte
/// assert_eq!(base64::encode_str_len("fo"), 4); // will be encoded as 4 byte
/// assert_eq!(base64::encode_str_len("foo"), 4); // will be encoded as 4 byte
/// assert_eq!(base64::encode_str_len("foob"), 8); // will be encoded as 8 byte
/// assert_eq!(base64::encode_str_len("fooba"), 8); // will be encoded as 8 byte
/// assert_eq!(base64::encode_str_len("foobar"), 8); // will be encoded as 8 byte
/// ```
#[inline]
pub fn encode_str_len(src: &str) -> usize {
    encode_len(src.len())
}

/// Calculate base64 encoded string length
///
/// - @param src_len: raw bytes length
/// - @return: base64 encoded string length
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// assert_eq!(base64::encode_len(0), 0);
/// assert_eq!(base64::encode_len(1), 4);
/// assert_eq!(base64::encode_len(2), 4);
/// assert_eq!(base64::encode_len(3), 4);
/// assert_eq!(base64::encode_len(4), 8);
/// assert_eq!(base64::encode_len(5), 8);
/// assert_eq!(base64::encode_len(6), 8);
/// ```
#[inline]
pub fn encode_len(src_len: usize) -> usize {
    (src_len + 2) / 3 * 4
}

/// Calculate base64 decoded data length. Padded base64 should always be
/// a multiple of 4 bytes in length.
///
/// - @param n: base64 string length
/// - @return: decoded raw bytes length
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// assert_eq!(base64::decode_len(0), 0);
/// assert_eq!(base64::decode_len(1), 0);
/// assert_eq!(base64::decode_len(2), 0);
/// assert_eq!(base64::decode_len(3), 0);
/// assert_eq!(base64::decode_len(4), 3);
/// assert_eq!(base64::decode_len(8), 6);
/// assert_eq!(base64::decode_len(12), 9);
/// ```
pub fn decode_len(n: usize) -> usize {
    n / 4 * 3
}

/// Encode bytes to base64 bytes
///
/// - @param dst: encoded base64 bytes
/// - @param src: raw bytes
/// - @param encode_map: base64 encoding map
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// let src = b"hello";
/// let dst_len = base64::encode_len(src.len());
/// let mut dst = vec![0; dst_len];
/// base64::encode(&mut dst, src, base64::ENCODE_STD);
/// assert_eq!(dst, b"aGVsbG8=");
/// ```
pub fn encode(dst: &mut [u8], src: &[u8], encode_map: &[u8]) {
    let src_len = src.len();

    // process every 3 bytes(src) to 4 bytes(dst)
    // 3x8bit => 4x6bit
    let mut src_idx = 0;
    let mut dst_idx = 0;
    let n = (src_len / 3) * 3;
    while src_idx < n {
        // Convert 3x 8bit source byte into 4 bytes
        let val: u32 = u32::from(src[src_idx + 0]) << 16
            | u32::from(src[src_idx + 1]) << 8
            | u32::from(src[src_idx + 2]);

        dst[dst_idx + 0] = encode_map[(val >> 18 & 0x3F) as usize];
        dst[dst_idx + 1] = encode_map[(val >> 12 & 0x3F) as usize];
        dst[dst_idx + 2] = encode_map[(val >> 6 & 0x3F) as usize];
        dst[dst_idx + 3] = encode_map[(val & 0x3F) as usize];

        src_idx += 3;
        dst_idx += 4;
    }

    let remain = src_len - src_idx;
    match remain {
        0 => return,
        1 => {
            let val: u32 = u32::from(src[src_idx + 0]) << 16;
            dst[dst_idx + 0] = encode_map[(val >> 18 & 0x3F) as usize];
            dst[dst_idx + 1] = encode_map[(val >> 12 & 0x3F) as usize];
            dst[dst_idx + 2] = PAD_CHAR;
            dst[dst_idx + 3] = PAD_CHAR;
        }
        2 => {
            let val: u32 = u32::from(src[src_idx + 0]) << 16 | u32::from(src[src_idx + 1]) << 8;
            dst[dst_idx + 0] = encode_map[(val >> 18 & 0x3F) as usize];
            dst[dst_idx + 1] = encode_map[(val >> 12 & 0x3F) as usize];
            dst[dst_idx + 2] = encode_map[(val >> 6 & 0x3F) as usize];
            dst[dst_idx + 3] = PAD_CHAR;
        }
        _ => unreachable!(),
    }
}

/// Encode bytes to base64 bytes, with custom map
///
/// - @param src: raw bytes
/// - @param encode_map: base64 encoding map
/// - @return: base64 encoded bytes
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// let encoded_bytes: Vec<u8> = base64::encode_bytes_with_map(b"hello", base64::ENCODE_STD);
/// assert_eq!(encoded_bytes, b"aGVsbG8=");
/// ```
pub fn encode_bytes_with_map(src: &[u8], encode_map: &[u8]) -> Vec<u8> {
    let dst_len = encode_len(src.len());
    let mut dst = vec![0; dst_len];
    encode(&mut dst, src, encode_map);
    dst
}

/// Encode string to base64 string, with custom map
///
/// - @param src: raw string
/// - @param encode_map: base64 encoding map
/// - @return: base64 encoded string
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// let encoded_string: String = base64::encode_str_with_map("hello", base64::ENCODE_STD);
/// assert_eq!(encoded_string, "aGVsbG8=".to_string());
/// ```
pub fn encode_str_with_map(src: &str, encode_map: &[u8]) -> String {
    let dst_len = encode_len(src.len());
    let mut dst = vec![0; dst_len];
    encode(&mut dst, src.as_bytes(), encode_map);
    String::from_utf8(dst).unwrap()
}

/// Encode bytes to base64 bytes
///
/// - @param src: raw bytes
/// - @return: base64 encoded bytes
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// let encoded_bytes: Vec<u8> = base64::encode_bytes(b"hello");
/// assert_eq!(encoded_bytes, b"aGVsbG8=");
/// ```
pub fn encode_bytes(src: &[u8]) -> Vec<u8> {
    let dst_len = encode_len(src.len());
    let mut dst = vec![0; dst_len];
    encode(&mut dst, src, ENCODE_STD);
    dst
}

/// Encode bytes to base64 string
///
/// - @param src: raw bytes
/// - @return: base64 encoded string
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// let encoded: String = base64::encode_to_str(b"hello");
/// assert_eq!(encoded, "aGVsbG8=".to_string());
/// ```
pub fn encode_to_str(src: &[u8]) -> String {
    let dst_len = encode_len(src.len());
    let mut dst = vec![0; dst_len];
    encode(&mut dst, src, ENCODE_STD);
    String::from_utf8(dst).unwrap()
}

/// Encode string to base64 string
///
/// - @param src: raw string
/// - @return: base64 encoded string
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// let encoded_string: String = base64::encode_str("hello");
/// assert_eq!(encoded_string, "aGVsbG8=".to_string());
/// ```
pub fn encode_str(src: &str) -> String {
    let dst_len = encode_len(src.len());
    let mut dst = vec![0; dst_len];
    encode(&mut dst, src.as_bytes(), ENCODE_STD);
    String::from_utf8(dst).unwrap()
}

/// Calculate decode map from encode map
///
/// - @param encode_map: encode map used for base64
/// - @return: decode map used for base64
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// let base64_std_decode_map = base64::decode_map(base64::ENCODE_STD);
/// let base64_url_decode_map = base64::decode_map(base64::ENCODE_URL);
/// assert_eq!(&base64_std_decode_map, base64::DECODE_STD_MAP);
/// assert_eq!(&base64_url_decode_map, base64::DECODE_URL_MAP);
/// ```
pub fn decode_map(encode_map: &[u8]) -> Vec<u8> {
    encode_map
        .iter()
        .enumerate()
        .fold(vec![0xFF; 256], |mut m, (i, &c)| {
            m[c as usize] = i as u8;
            m
        })
}

#[inline]
fn from_char(b: u8, decode_map: &[u8]) -> Result<u8, InvalidByteError> {
    let out = decode_map[b as usize];
    if out == 0xFF {
        return Err(InvalidByteError::new(b));
    }
    return Ok(out);
}

/// Decode base64 bytes to raw bytes
///
/// - @param dst: decoded raw bytes
/// - @param src: base64 bytes
/// - @param encode_map: base64 decoding map
/// - @return: raw byte size if successfully decoded.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// let src = b"aGVsbG8=";
/// let mut dst = vec![0; base64::decode_len(src.len())];
/// let dst_size = base64::decode(&mut dst, src, base64::DECODE_STD_MAP).unwrap();
/// dst.drain(dst_size..);
/// assert_eq!(dst, b"hello");
/// ```
pub fn decode(dst: &mut [u8], src: &[u8], decode_map: &[u8]) -> Result<usize, Base64Error> {
    let src_len = src.len();
    if src_len == 0 || src_len % 4 != 0 {
        return Err(Base64Error::InvalidLength);
    }

    // Process src[0..src_len-4]
    let mut src_idx = 0;
    let mut dst_idx = 0;
    while src_idx + 4 < src_len {
        let val: u32 = u32::from(from_char(src[src_idx + 0], decode_map)?) << 18
            | u32::from(from_char(src[src_idx + 1], decode_map)?) << 12
            | u32::from(from_char(src[src_idx + 2], decode_map)?) << 6
            | u32::from(from_char(src[src_idx + 3], decode_map)?);

        dst[dst_idx + 0] = ((val >> 16) & 0xFF) as u8;
        dst[dst_idx + 1] = ((val >> 8) & 0xFF) as u8;
        dst[dst_idx + 2] = (val & 0xFF) as u8;

        src_idx += 4;
        dst_idx += 3;
    }

    // Process last 4 bytes
    if src[src_idx + 2] == b'=' {
        // two padding char, [c c = =], convert to one dst byte
        let val: u32 = u32::from(from_char(src[src_idx + 0], decode_map)?) << 18
            | u32::from(from_char(src[src_idx + 1], decode_map)?) << 12;
        dst[dst_idx + 0] = ((val >> 16) & 0xFF) as u8;
        dst_idx += 1;
    } else if src[src_idx + 3] == b'=' {
        // one padding char, [c c c =], convert to two dst byte
        let val: u32 = u32::from(from_char(src[src_idx + 0], decode_map)?) << 18
            | u32::from(from_char(src[src_idx + 1], decode_map)?) << 12
            | u32::from(from_char(src[src_idx + 2], decode_map)?) << 6;
        dst[dst_idx + 0] = ((val >> 16) & 0xFF) as u8;
        dst[dst_idx + 1] = ((val >> 8) & 0xFF) as u8;
        dst_idx += 2;
    } else {
        // no padding char, [c c c c], convert to three dst byte
        let val: u32 = u32::from(from_char(src[src_idx + 0], decode_map)?) << 18
            | u32::from(from_char(src[src_idx + 1], decode_map)?) << 12
            | u32::from(from_char(src[src_idx + 2], decode_map)?) << 6
            | u32::from(from_char(src[src_idx + 3], decode_map)?);
        dst[dst_idx + 0] = ((val >> 16) & 0xFF) as u8;
        dst[dst_idx + 1] = ((val >> 8) & 0xFF) as u8;
        dst[dst_idx + 2] = (val & 0xFF) as u8;
        dst_idx += 3;
    }

    Ok(dst_idx)
}

/// Decode base64 string to raw bytes
///
/// - @param src: base64 string
/// - @return: decoded raw byte if successfully decoded.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use connx::encoding::base64;
/// let src = "aGVsbG8=";
/// let dst = base64::decode_str(src).unwrap();
/// assert_eq!(dst, b"hello");
/// ```
pub fn decode_str(src: &str) -> Result<Vec<u8>, Base64Error> {
    let mut dst = vec![0; decode_len(src.len())];
    let dst_size = decode(&mut dst, src.as_bytes(), DECODE_STD_MAP)?;
    dst.drain(dst_size..);
    Ok(dst)
}

#[cfg(test)]
mod tests_base64 {
    use super::*;

    #[test]
    fn test_encode_str_len_01() {
        assert_eq!(encode_str_len(""), 0);
        assert_eq!(encode_str_len("a"), 4);
        assert_eq!(encode_str_len("ab"), 4);
        assert_eq!(encode_str_len("abc"), 4);
        assert_eq!(encode_str_len("abcd"), 8);
    }

    #[test]
    fn test_encode_str_01() {
        assert_eq!(encode_str(""), "".to_string());
        assert_eq!(encode_str("f"), "Zg==".to_string());
        assert_eq!(encode_str("fo"), "Zm8=".to_string());
        assert_eq!(encode_str("foo"), "Zm9v".to_string());
        assert_eq!(encode_str("foob"), "Zm9vYg==".to_string());
        assert_eq!(encode_str("fooba"), "Zm9vYmE=".to_string());
        assert_eq!(encode_str("foobar"), "Zm9vYmFy".to_string());
    }

    #[test]
    fn test_encode_str_02() {
        assert_eq!(encode_str("sure."), "c3VyZS4=".to_string());
        assert_eq!(encode_str("sure"), "c3VyZQ==".to_string());
        assert_eq!(encode_str("sur"), "c3Vy".to_string());
        assert_eq!(encode_str("su"), "c3U=".to_string());
        assert_eq!(encode_str("leasure."), "bGVhc3VyZS4=".to_string());
        assert_eq!(encode_str("easure."), "ZWFzdXJlLg==".to_string());
        assert_eq!(encode_str("asure."), "YXN1cmUu".to_string());
        assert_eq!(encode_str("sure."), "c3VyZS4=".to_string());
    }

    #[test]
    fn test_encode_str_03() {
        assert_eq!(encode_str("hello"), "aGVsbG8=".to_string());
        assert_eq!(encode_bytes(b"hello"), b"aGVsbG8=");
    }

    #[test]
    fn test_decode_map_01() {
        let m = decode_map(ENCODE_STD);
        assert_eq!(m, DECODE_STD_MAP);
    }

    #[test]
    fn test_decode_str_01() {
        assert_eq!(decode_str(""), Err(Base64Error::InvalidLength));
        assert_eq!(decode_str("Zg=="), Ok((b"f").to_vec()));
        assert_eq!(decode_str("Zm8="), Ok((b"fo").to_vec()));
        assert_eq!(decode_str("Zm9v"), Ok((b"foo").to_vec()));
        assert_eq!(decode_str("Zm9vYg=="), Ok((b"foob").to_vec()));
        assert_eq!(decode_str("Zm9vYmE="), Ok((b"fooba").to_vec()));
        assert_eq!(decode_str("Zm9vYmFy"), Ok((b"foobar").to_vec()));
    }

    #[test]
    fn test_decode_str_02() {
        assert_eq!(decode_str("c3VyZS4="), Ok((b"sure.").to_vec()));
        assert_eq!(decode_str("c3VyZQ=="), Ok((b"sure").to_vec()));
        assert_eq!(decode_str("c3Vy"), Ok((b"sur").to_vec()));
        assert_eq!(decode_str("c3U="), Ok((b"su").to_vec()));
        assert_eq!(decode_str("bGVhc3VyZS4="), Ok((b"leasure.").to_vec()));
        assert_eq!(decode_str("ZWFzdXJlLg=="), Ok((b"easure.").to_vec()));
        assert_eq!(decode_str("YXN1cmUu"), Ok((b"asure.").to_vec()));
        assert_eq!(decode_str("c3VyZS4="), Ok((b"sure.").to_vec()));
    }
}
