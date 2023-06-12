//! RFC4648 implemented, base64 encoding.

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

/// Calculate base64 encoded string length
///
/// - @param src: raw string
/// - @return: base64 encoded string length
#[inline]
pub fn encode_str_len(src: &str) -> usize {
    encode_len(src.len())
}

/// Calculate base64 encoded string length
///
/// - @param src_len: raw bytes length
/// - @return: base64 encoded string length
#[inline]
pub fn encode_len(src_len: usize) -> usize {
    (src_len + 2) / 3 * 4
}

/// Encode bytes to base64 bytes
///
/// - @param src: raw bytes
/// - @param encode_map: base64 encoding table
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
    let src_len = src.len();
    let dst_len = encode_len(src_len);
    let mut dst = vec![0; dst_len];

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
        0 => return dst,
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

    dst
}

/// Encode string to base64 string
///
/// - @param src: raw string
/// - @param encode_map: base64 encoding table
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
    let dst = encode_bytes_with_map(src.as_bytes(), encode_map);
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
    encode_bytes_with_map(src, ENCODE_STD)
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
    let dst = encode_bytes(src.as_bytes());
    String::from_utf8(dst).unwrap()
}

/// Calculate decode map from encode map
pub fn decode_map(encode_map: &[u8]) -> Vec<u8> {
    encode_map
        .iter()
        .enumerate()
        .fold(vec![0xFF; 256], |mut m, (i, &c)| {
            m[c as usize] = i as u8;
            m
        })
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
    fn test01() {
        assert_eq!(encode_str(""), "".to_string());
        assert_eq!(encode_str("f"), "Zg==".to_string());
        assert_eq!(encode_str("fo"), "Zm8=".to_string());
        assert_eq!(encode_str("foo"), "Zm9v".to_string());
        assert_eq!(encode_str("foob"), "Zm9vYg==".to_string());
        assert_eq!(encode_str("fooba"), "Zm9vYmE=".to_string());
        assert_eq!(encode_str("foobar"), "Zm9vYmFy".to_string());
    }

    #[test]
    fn test02() {
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
    fn test03() {
        assert_eq!(encode_str("hello"), "aGVsbG8=".to_string());
        assert_eq!(encode_bytes(b"hello"), b"aGVsbG8=");
    }

    #[test]
    fn test_decode_map_01() {
        let m = decode_map(ENCODE_STD);
        assert_eq!(m, DECODE_STD_MAP);
    }
}
