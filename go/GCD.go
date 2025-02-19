package main

import "fmt"

func GCD(m int, n int) int {
	if m < n {
		return GCD(n, m)
	}
	if m%n == 0 {
		return n
	}

	return GCD(n, m%n)
}

func main() {
	fmt.Println("GCD(17,5):", GCD(17, 5))
}
