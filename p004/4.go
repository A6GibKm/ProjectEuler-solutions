// A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.

// Find the largest palindrome made from the product of two 3-digit numbers.

package main

import "fmt"

func Power(a, n int) int {
	var i, result int
	result = 1
	for i = 0; i < n; i++ {
		result *= a
	}
	return result
}

func pal(n int) int {
	sum := 0
	var N int = 0
	for i := 0; !(n/Power(10, i) == 0); i++ {
		N = i
	}

	for i := 1; i <= N+1; i++ {
		sum += (n % Power(10, i) / Power(10, i-1)) * Power(10, N-i+1)
	}
	return sum
}

func isPal(n int) bool {
	return n == pal(n)
}
func main() {
	max := 1
	for i := 800; i <= 1000; i++ {
		for j := i; j <= 1000; j++ {
			if isPal(i*j) && i*j > max {
				max = i * j
				fmt.Println(i, j, i*j, max)
			}
		}
	}
}
