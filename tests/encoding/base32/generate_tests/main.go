package main

import (
	"encoding/base32"
	"fmt"
	"math/rand"
	"time"
)

func random_string(n int) string {
	const charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
	b := make([]byte, n)
	for i := range b {
		b[i] = charset[rand.Intn(len(charset))]
	}
	return string(b)
}

func test() {
	for i := 0; i < 100; i++ {
		str := random_string(100)
		encoded := base32.StdEncoding.EncodeToString([]byte(str))
		fmt.Printf(`test_encode_base(b"%v", b"%v");`, str, encoded)
		fmt.Println()
	}
}

func main() {
	rand.Seed(time.Now().UnixNano())
	test()
}
