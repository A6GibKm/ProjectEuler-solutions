// By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

// What is the 10 001st prime number?
package main

import "fmt"

func isPrime(n int) bool {
	if n == 1 {
		return false
	} else if n == 2 {
		return true
	} else {
		for i := 2; i < n; i++ {
			if n%i == 0 {
				return false
			}
		}
	}
	return true
}

var r int = 0

func main() {
	counter := 0
	for n := 1; counter < 10001; n++ {
		if isPrime(n) {
			counter++
			r = n
		}
	}
	fmt.Println(r)
}
