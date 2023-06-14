//! Little Endian and Big Endian implemented
//!
//! # Examples
//!
//! Little Endian encode u16:
//!
//! ```
//! use connx::encoding::binary::{ByteOrder, LittleEndian};
//! let n: u16 = 0x1234;
//! let mut b = vec![0; 2];
//! LittleEndian::encode_u16(n, &mut b);
//! assert_eq!(b, vec![0x34, 0x12]);
//! assert_eq!(n, LittleEndian::decode_u16(&b));
//! ```
//!
//! Big Endian encode u16:
//!
//! ```
//! use connx::encoding::binary::{ByteOrder, BigEndian};
//! let n: u16 = 0x1234;
//! let mut b = vec![0; 2];
//! BigEndian::encode_u16(n, &mut b);
//! assert_eq!(b, vec![0x12, 0x34]);
//! assert_eq!(n, BigEndian::decode_u16(&b));
//! ```
//!
//! Little Endian encode u32:
//!
//! ```
//! use connx::encoding::binary::{ByteOrder, LittleEndian};
//! let n: u32 = 0x12345678;
//! let mut b = vec![0; 4];
//! LittleEndian::encode_u32(n, &mut b);
//! assert_eq!(b, vec![0x78, 0x56, 0x34, 0x12]);
//! assert_eq!(n, LittleEndian::decode_u32(&b));
//! ```
//!
//! Big Endian encode u32:
//!
//! ```
//! use connx::encoding::binary::{ByteOrder, BigEndian};
//! let n: u32 = 0x12345678;
//! let mut b = vec![0; 4];
//! BigEndian::encode_u32(n, &mut b);
//! assert_eq!(b, vec![0x12, 0x34, 0x56, 0x78]);
//! assert_eq!(n, BigEndian::decode_u32(&b));
//! ```

pub trait ByteOrder {
    fn encode_u16(n: u16, b: &mut [u8]);
    fn encode_u32(n: u32, b: &mut [u8]);
    fn encode_u64(n: u64, b: &mut [u8]);
    fn decode_u16(b: &[u8]) -> u16;
    fn decode_u32(b: &[u8]) -> u32;
    fn decode_u64(b: &[u8]) -> u64;
}

/// Little Endian implemented
pub struct LittleEndian;

impl ByteOrder for LittleEndian {
    fn encode_u16(n: u16, b: &mut [u8]) {
        b[0] = n as u8;
        b[1] = (n >> 8) as u8;
    }

    fn encode_u32(n: u32, b: &mut [u8]) {
        b[0] = n as u8;
        b[1] = (n >> 8) as u8;
        b[2] = (n >> 16) as u8;
        b[3] = (n >> 24) as u8;
    }

    fn encode_u64(n: u64, b: &mut [u8]) {
        b[0] = n as u8;
        b[1] = (n >> 8) as u8;
        b[2] = (n >> 16) as u8;
        b[3] = (n >> 24) as u8;
        b[4] = (n >> 32) as u8;
        b[5] = (n >> 40) as u8;
        b[6] = (n >> 48) as u8;
        b[7] = (n >> 56) as u8;
    }

    fn decode_u16(b: &[u8]) -> u16 {
        (b[0] as u16) | ((b[1] as u16) << 8)
    }

    fn decode_u32(b: &[u8]) -> u32 {
        (b[0] as u32) | ((b[1] as u32) << 8) | ((b[2] as u32) << 16) | ((b[3] as u32) << 24)
    }

    fn decode_u64(b: &[u8]) -> u64 {
        (b[0] as u64)
            | ((b[1] as u64) << 8)
            | ((b[2] as u64) << 16)
            | ((b[3] as u64) << 24)
            | ((b[4] as u64) << 32)
            | ((b[5] as u64) << 40)
            | ((b[6] as u64) << 48)
            | ((b[7] as u64) << 56)
    }
}

/// Big Endian implemented
pub struct BigEndian;

impl ByteOrder for BigEndian {
    fn encode_u16(n: u16, b: &mut [u8]) {
        b[0] = (n >> 8) as u8;
        b[1] = n as u8;
    }

    fn encode_u32(n: u32, b: &mut [u8]) {
        b[0] = (n >> 24) as u8;
        b[1] = (n >> 16) as u8;
        b[2] = (n >> 8) as u8;
        b[3] = n as u8;
    }

    fn encode_u64(n: u64, b: &mut [u8]) {
        b[0] = (n >> 56) as u8;
        b[1] = (n >> 48) as u8;
        b[2] = (n >> 40) as u8;
        b[3] = (n >> 32) as u8;
        b[4] = (n >> 24) as u8;
        b[5] = (n >> 16) as u8;
        b[6] = (n >> 8) as u8;
        b[7] = n as u8;
    }

    fn decode_u16(b: &[u8]) -> u16 {
        (b[1] as u16) | ((b[0] as u16) << 8)
    }

    fn decode_u32(b: &[u8]) -> u32 {
        (b[3] as u32) | ((b[2] as u32) << 8) | ((b[1] as u32) << 16) | ((b[0] as u32) << 24)
    }

    fn decode_u64(b: &[u8]) -> u64 {
        (b[7] as u64)
            | ((b[6] as u64) << 8)
            | ((b[5] as u64) << 16)
            | ((b[4] as u64) << 24)
            | ((b[3] as u64) << 32)
            | ((b[2] as u64) << 40)
            | ((b[1] as u64) << 48)
            | ((b[0] as u64) << 56)
    }
}

#[cfg(test)]
mod tests_binary {
    use super::*;

    #[test]
    fn test() {
        let n = 0x1234;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x34, 0x12]);
    }

    #[test]
    fn test_little_u16_00() {
        let n = 0xcf86;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x86, 0xcf]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u16_01() {
        let n = 0x1272;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x72, 0x12]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u16_02() {
        let n = 0xa2d7;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0xd7, 0xa2]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u16_03() {
        let n = 0x3cc2;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0xc2, 0x3c]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u16_04() {
        let n = 0xbbdb;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0xdb, 0xbb]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u16_05() {
        let n = 0x2970;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x70, 0x29]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u16_06() {
        let n = 0xd39f;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x9f, 0xd3]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u16_07() {
        let n = 0xe6b4;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0xb4, 0xe6]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u16_08() {
        let n = 0x8bb1;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0xb1, 0x8b]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u16_09() {
        let n = 0x6e28;
        let mut b = vec![0; 2];
        LittleEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x28, 0x6e]);
        let n2 = LittleEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_00() {
        let n = 0x83e4f98d;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x8d, 0xf9, 0xe4, 0x83]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_01() {
        let n = 0xd04ab55f;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x5f, 0xb5, 0x4a, 0xd0]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_02() {
        let n = 0x36d9ff45;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x45, 0xff, 0xd9, 0x36]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_03() {
        let n = 0x6172bfe3;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0xe3, 0xbf, 0x72, 0x61]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_04() {
        let n = 0x516c42b0;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0xb0, 0x42, 0x6c, 0x51]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_05() {
        let n = 0x78092a35;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x35, 0x2a, 0x09, 0x78]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_06() {
        let n = 0x4874ed16;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x16, 0xed, 0x74, 0x48]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_07() {
        let n = 0x4b08b92b;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x2b, 0xb9, 0x08, 0x4b]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_08() {
        let n = 0xadd87e4a;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x4a, 0x7e, 0xd8, 0xad]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u32_09() {
        let n = 0x37f317c5;
        let mut b = vec![0; 4];
        LittleEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0xc5, 0x17, 0xf3, 0x37]);
        let n2 = LittleEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_00() {
        let n = 0x1a02070f169c1121;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x21, 0x11, 0x9c, 0x16, 0x0f, 0x07, 0x02, 0x1a]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_01() {
        let n = 0x2e3108dabb158644;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x44, 0x86, 0x15, 0xbb, 0xda, 0x08, 0x31, 0x2e]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_02() {
        let n = 0xc90bd268b68e6a3f;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x3f, 0x6a, 0x8e, 0xb6, 0x68, 0xd2, 0x0b, 0xc9]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_03() {
        let n = 0x6e661e92759805f5;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0xf5, 0x05, 0x98, 0x75, 0x92, 0x1e, 0x66, 0x6e]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_04() {
        let n = 0xa584c47f2cdf5b8a;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x8a, 0x5b, 0xdf, 0x2c, 0x7f, 0xc4, 0x84, 0xa5]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_05() {
        let n = 0x2606cd2b57d29245;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x45, 0x92, 0xd2, 0x57, 0x2b, 0xcd, 0x06, 0x26]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_06() {
        let n = 0x6054502fc5d6d268;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x68, 0xd2, 0xd6, 0xc5, 0x2f, 0x50, 0x54, 0x60]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_07() {
        let n = 0x1a714cf86b83d0e2;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0xe2, 0xd0, 0x83, 0x6b, 0xf8, 0x4c, 0x71, 0x1a]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_08() {
        let n = 0xeec34c367674cb74;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x74, 0xcb, 0x74, 0x76, 0x36, 0x4c, 0xc3, 0xee]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_little_u64_09() {
        let n = 0xd92e17f7b068d9db;
        let mut b = vec![0; 8];
        LittleEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0xdb, 0xd9, 0x68, 0xb0, 0xf7, 0x17, 0x2e, 0xd9]);
        let n2 = LittleEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_00() {
        let n = 0xce41;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0xce, 0x41]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_01() {
        let n = 0x7317;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x73, 0x17]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_02() {
        let n = 0x2e1f;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x2e, 0x1f]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_03() {
        let n = 0xc8c9;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0xc8, 0xc9]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_04() {
        let n = 0x347a;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x34, 0x7a]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_05() {
        let n = 0x9377;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x93, 0x77]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_06() {
        let n = 0xfc4b;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0xfc, 0x4b]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_07() {
        let n = 0x4b88;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x4b, 0x88]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_08() {
        let n = 0x9b91;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x9b, 0x91]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u16_09() {
        let n = 0x722a;
        let mut b = vec![0; 2];
        BigEndian::encode_u16(n, &mut b);
        assert_eq!(b, vec![0x72, 0x2a]);
        let n2 = BigEndian::decode_u16(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_00() {
        let n = 0x2c5b2d1a;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x2c, 0x5b, 0x2d, 0x1a]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_01() {
        let n = 0x8a858525;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x8a, 0x85, 0x85, 0x25]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_02() {
        let n = 0x8b4dc795;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x8b, 0x4d, 0xc7, 0x95]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_03() {
        let n = 0x474c4687;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x47, 0x4c, 0x46, 0x87]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_04() {
        let n = 0x6c53b3e2;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x6c, 0x53, 0xb3, 0xe2]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_05() {
        let n = 0x87d47727;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x87, 0xd4, 0x77, 0x27]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_06() {
        let n = 0x40e807bd;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x40, 0xe8, 0x07, 0xbd]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_07() {
        let n = 0x483675c8;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x48, 0x36, 0x75, 0xc8]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_08() {
        let n = 0xc9e202ff;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0xc9, 0xe2, 0x02, 0xff]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u32_09() {
        let n = 0x5c9f48b2;
        let mut b = vec![0; 4];
        BigEndian::encode_u32(n, &mut b);
        assert_eq!(b, vec![0x5c, 0x9f, 0x48, 0xb2]);
        let n2 = BigEndian::decode_u32(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_00() {
        let n = 0xf0b5a315724c7af1;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0xf0, 0xb5, 0xa3, 0x15, 0x72, 0x4c, 0x7a, 0xf1]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_01() {
        let n = 0xa607c649581eeb39;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0xa6, 0x07, 0xc6, 0x49, 0x58, 0x1e, 0xeb, 0x39]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_02() {
        let n = 0x727a71f52257bb7d;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x72, 0x7a, 0x71, 0xf5, 0x22, 0x57, 0xbb, 0x7d]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_03() {
        let n = 0xc7964976f269a28;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x0c, 0x79, 0x64, 0x97, 0x6f, 0x26, 0x9a, 0x28]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_04() {
        let n = 0x7d0b9ca8be8e9981;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x7d, 0x0b, 0x9c, 0xa8, 0xbe, 0x8e, 0x99, 0x81]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_05() {
        let n = 0x89825e117039374b;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x89, 0x82, 0x5e, 0x11, 0x70, 0x39, 0x37, 0x4b]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_06() {
        let n = 0x9c73fac825416fed;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x9c, 0x73, 0xfa, 0xc8, 0x25, 0x41, 0x6f, 0xed]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_07() {
        let n = 0xd72d92faded7e411;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0xd7, 0x2d, 0x92, 0xfa, 0xde, 0xd7, 0xe4, 0x11]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_08() {
        let n = 0x1ee9f7676678e7aa;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0x1e, 0xe9, 0xf7, 0x67, 0x66, 0x78, 0xe7, 0xaa]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_big_u64_09() {
        let n = 0xa7dff7ab244fcd36;
        let mut b = vec![0; 8];
        BigEndian::encode_u64(n, &mut b);
        assert_eq!(b, vec![0xa7, 0xdf, 0xf7, 0xab, 0x24, 0x4f, 0xcd, 0x36]);
        let n2 = BigEndian::decode_u64(&b);
        assert_eq!(n, n2);
    }
}
