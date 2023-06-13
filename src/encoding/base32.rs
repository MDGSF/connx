/// Standard encoding map from RFC4648
pub const ENCODE_STD: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

/// Hex encoding map from RFC4648
pub const ENCODE_HEX: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";

/// Standard padding char from RFC4648
pub const PAD_CHAR: u8 = b'=';

#[inline]
pub fn encode_len(n: usize) -> usize {
    (n + 4) / 5 * 8
}

#[inline]
pub fn decode_len(n: usize) -> usize {
    n / 8 * 5
}

pub fn encode(dst: &mut [u8], src: &[u8], encode_map: &[u8]) {
    let src_len = src.len();

    // process every 5 bytes(src) to 8 bytes(dst)
    // 5x8bit => 8x5bit
    let mut src_idx = 0;
    let mut dst_idx = 0;
    let n = (src_len / 5) * 5;
    while src_idx < n {
        let val: u64 = u64::from(src[src_idx + 0]) << 32
            | u64::from(src[src_idx + 1]) << 24
            | u64::from(src[src_idx + 2]) << 16
            | u64::from(src[src_idx + 3]) << 8
            | u64::from(src[src_idx + 4]);

        dst[dst_idx + 0] = encode_map[(val >> 35 & 0x1F) as usize];
        dst[dst_idx + 1] = encode_map[(val >> 30 & 0x1F) as usize];
        dst[dst_idx + 2] = encode_map[(val >> 25 & 0x1F) as usize];
        dst[dst_idx + 3] = encode_map[(val >> 20 & 0x1F) as usize];
        dst[dst_idx + 4] = encode_map[(val >> 15 & 0x1F) as usize];
        dst[dst_idx + 5] = encode_map[(val >> 10 & 0x1F) as usize];
        dst[dst_idx + 6] = encode_map[(val >> 5 & 0x1F) as usize];
        dst[dst_idx + 7] = encode_map[(val & 0x1F) as usize];

        src_idx += 5;
        dst_idx += 8;
    }

    let remain = src_len - src_idx;
    match remain {
        0 => return,
        1 => {
            let val: u32 = u32::from(src[src_idx + 0]) << 8;
            dst[dst_idx + 0] = encode_map[(val >> 11 & 0x1F) as usize];
            dst[dst_idx + 1] = encode_map[(val >> 6 & 0x1F) as usize];
            for i in (dst_idx + 2)..=(dst_idx + 7) {
                dst[i] = PAD_CHAR;
            }
        }
        2 => {
            let val: u32 = u32::from(src[src_idx + 0]) << 16 | u32::from(src[src_idx + 1]) << 8;
            dst[dst_idx + 0] = encode_map[(val >> 19 & 0x1F) as usize];
            dst[dst_idx + 1] = encode_map[(val >> 14 & 0x1F) as usize];
            dst[dst_idx + 2] = encode_map[(val >> 9 & 0x1F) as usize];
            dst[dst_idx + 3] = encode_map[(val >> 4 & 0x1F) as usize];
            for i in (dst_idx + 4)..=(dst_idx + 7) {
                dst[i] = PAD_CHAR;
            }
        }
        3 | 4 => {
            let mut val: u64 = 0;
            for i in 0..remain {
                val |= u64::from(src[src_idx + i]) << ((4 - i) * 8);
            }
            let remain_bit: i32 = (remain * 8) as i32;
            let mut used_bit: i32 = 0;
            for dst_offset in 0..8 {
                dst[dst_idx + dst_offset] = if used_bit >= remain_bit {
                    PAD_CHAR
                } else {
                    used_bit += 5;
                    encode_map[(val >> (40 - used_bit) & 0x1F) as usize]
                };
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests_base32 {
    use super::*;

    fn test_encode_base(data: &[u8], encoded_expect: &[u8]) {
        let mut dst = vec![0; encode_len(data.len())];
        encode(&mut dst, data, ENCODE_STD);
        assert_eq!(dst, encoded_expect);
    }

    #[test]
    fn test_encode_01() {
        test_encode_base(b"", b"");
        test_encode_base(b"f", b"MY======");
        test_encode_base(b"fo", b"MZXQ====");
        test_encode_base(b"foo", b"MZXW6===");
        test_encode_base(b"foob", b"MZXW6YQ=");
        test_encode_base(b"fooba", b"MZXW6YTB");
        test_encode_base(b"foobar", b"MZXW6YTBOI======");
    }
}
