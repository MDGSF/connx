pub const ENCODE_STD: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
pub const ENCODE_HEX: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUV";

fn main() {
    let decode_std_map = decode_map(ENCODE_STD);
    let decode_hex_map = decode_map(ENCODE_HEX);

    print!("pub const DECODE_STD_MAP: &[u8] = &[");
    for i in 0..256 {
        if i != 0 {
            print!(", ");
        }
        print!("0x{:02x}", decode_std_map[i]);
    }
    println!("];");

    print!("pub const DECODE_HEX_MAP: &[u8] = &[");
    for i in 0..256 {
        if i != 0 {
            print!(", ");
        }
        print!("0x{:02x}", decode_hex_map[i]);
    }
    println!("];");
}

pub fn decode_map(encode_map: &[u8]) -> Vec<u8> {
    encode_map
        .iter()
        .enumerate()
        .fold(vec![0xFF; 256], |mut m, (i, &c)| {
            m[c as usize] = i as u8;
            m
        })
}
