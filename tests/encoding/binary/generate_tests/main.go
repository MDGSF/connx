/*
go run main.go > log.txt
*/

package main

import (
	"encoding/binary"
	"fmt"
	"math/rand"
)

func test_little_u16() {
	for i := 0; i < 10; i++ {
		num := uint16(rand.Int31n(65535))
		b := make([]byte, 2)
		binary.LittleEndian.PutUint16(b, num)
		// fmt.Println(num, b)
		fmt.Printf(`
#[test]
fn test_little_u16_%02d() {
	let n = 0x%x;
	let mut b = vec![0; 2];
	LittleEndian::encode_u16(n, &mut b);
	assert_eq!(b, vec![0x%02x, 0x%02x]);
	let n2 = LittleEndian::decode_u16(&b);
	assert_eq!(n, n2);
}
`, i, num, b[0], b[1])
	}
}

func test_little_u32() {
	for i := 0; i < 10; i++ {
		num := rand.Uint32()
		b := make([]byte, 4)
		binary.LittleEndian.PutUint32(b, num)
		fmt.Printf(`
#[test]
fn test_little_u32_%02d() {
	let n = 0x%x;
	let mut b = vec![0; 4];
	LittleEndian::encode_u32(n, &mut b);
	assert_eq!(b, vec![0x%02x, 0x%02x, 0x%02x, 0x%02x]);
	let n2 = LittleEndian::decode_u32(&b);
	assert_eq!(n, n2);
}
`, i, num, b[0], b[1], b[2], b[3])
	}
}

func test_little_u64() {
	for i := 0; i < 10; i++ {
		num := rand.Uint64()
		b := make([]byte, 8)
		binary.LittleEndian.PutUint64(b, num)
		fmt.Printf(`
#[test]
fn test_little_u64_%02d() {
	let n = 0x%x;
	let mut b = vec![0; 8];
	LittleEndian::encode_u64(n, &mut b);
	assert_eq!(b, vec![0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x]);
	let n2 = LittleEndian::decode_u64(&b);
	assert_eq!(n, n2);
}
`, i, num, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7])
	}
}

func test_big_u16() {
	for i := 0; i < 10; i++ {
		num := uint16(rand.Int31n(65535))
		b := make([]byte, 2)
		binary.BigEndian.PutUint16(b, num)
		// fmt.Println(num, b)
		fmt.Printf(`
#[test]
fn test_big_u16_%02d() {
	let n = 0x%x;
	let mut b = vec![0; 2];
	BigEndian::encode_u16(n, &mut b);
	assert_eq!(b, vec![0x%02x, 0x%02x]);
	let n2 = BigEndian::decode_u16(&b);
	assert_eq!(n, n2);
}
`, i, num, b[0], b[1])
	}
}

func test_big_u32() {
	for i := 0; i < 10; i++ {
		num := rand.Uint32()
		b := make([]byte, 4)
		binary.BigEndian.PutUint32(b, num)
		fmt.Printf(`
#[test]
fn test_big_u32_%02d() {
	let n = 0x%x;
	let mut b = vec![0; 4];
	BigEndian::encode_u32(n, &mut b);
	assert_eq!(b, vec![0x%02x, 0x%02x, 0x%02x, 0x%02x]);
	let n2 = BigEndian::decode_u32(&b);
	assert_eq!(n, n2);
}
`, i, num, b[0], b[1], b[2], b[3])
	}
}

func test_big_u64() {
	for i := 0; i < 10; i++ {
		num := rand.Uint64()
		b := make([]byte, 8)
		binary.BigEndian.PutUint64(b, num)
		fmt.Printf(`
#[test]
fn test_big_u64_%02d() {
	let n = 0x%x;
	let mut b = vec![0; 8];
	BigEndian::encode_u64(n, &mut b);
	assert_eq!(b, vec![0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x, 0x%02x]);
	let n2 = BigEndian::decode_u64(&b);
	assert_eq!(n, n2);
}
`, i, num, b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7])
	}
}

func main() {
	test_little_u16()
	test_little_u32()
	test_little_u64()
	test_big_u16()
	test_big_u32()
	test_big_u64()
}
