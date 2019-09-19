// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

package main

import "fmt"

var s int = 20

func main() {
	bool = true
	for n := 1; bool; n++ {
		a := 0
		for i := 1; i <= s; i++ {
			a += n % i
		}

		if a == 0 {
			bool = false
			fmt.Println(n)
		}

	}
}
