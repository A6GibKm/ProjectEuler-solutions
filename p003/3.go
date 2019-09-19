// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?

package main

import "fmt"

var N int = 600851475143

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

func main() {
	for i := 1; i < N; i++ {
		if isPrime(i) && N%i == 0 {
			fmt.Println(i)
		}

	}

}
