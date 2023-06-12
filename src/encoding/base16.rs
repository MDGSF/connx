//! RFC4648 implemented, base16 encoding.
//! Base 16 encoding is the standard case-insensitive hex encoding and
//! may be referred to as "base16" or "hex".

#[derive(Debug)]
pub enum Base16Error {
    InvalidByte(InvalidByteError),
    Length,
}

impl std::fmt::Display for Base16Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidByte(e) => e.fmt(f),
            Self::Length => write!(f, "encoding/hex: odd length hex string"),
        }
    }
}

impl std::error::Error for Base16Error {}

impl From<InvalidByteError> for Base16Error {
    fn from(e: InvalidByteError) -> Self {
        Self::InvalidByte(e)
    }
}

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
        write!(f, "encoding/hex: invalid byte: {}", self.b)
    }
}

impl std::error::Error for InvalidByteError {}

pub const HEX_TABLE: &[u8] = b"0123456789abcdef";

/// Calculate the length of an encoding of n source bytes.
/// Specifically, it returns n * 2.
#[inline]
pub fn encode_len(n: usize) -> usize {
    n * 2
}

#[inline]
pub fn decode_len(n: usize) -> usize {
    n / 2
}

/// Encode `src` into `dst`.
pub fn encode(dst: &mut [u8], src: &[u8]) -> usize {
    src.iter().fold(0, |dst_idx, &src_byte| {
        dst[dst_idx] = HEX_TABLE[(src_byte >> 4) as usize];
        dst[dst_idx + 1] = HEX_TABLE[(src_byte & 0x0F) as usize];
        dst_idx + 2
    });
    src.len() * 2
}

fn from_hex_char(b: u8) -> Result<u8, InvalidByteError> {
    if b >= b'0' && b <= b'9' {
        Ok(b - b'0')
    } else if b >= b'a' && b <= b'f' {
        Ok(b - b'a' + 10)
    } else if b >= b'A' && b <= b'F' {
        Ok(b - b'A' + 10)
    } else {
        Err(InvalidByteError::new(b))
    }
}

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
        return Err(Base16Error::Length);
    }
    Ok(dst_idx)
}

pub fn encode_to_string(src: &[u8]) -> String {
    let mut dst = vec![0u8; encode_len(src.len())];
    encode(&mut dst, src);
    String::from_utf8(dst).unwrap()
}

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
}
