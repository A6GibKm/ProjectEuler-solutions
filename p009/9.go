// A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
// a2 + b2 = c2

// For example, 32 + 42 = 9 + 16 = 25 = 52.

// There exists exactly one Pythagorean triplet for which a + b + c = 1000.
// Find the product abc.
package main

func main() {

	for a := 1; a <= 1000; a++ {
		for b := a; b <= 1000; b++ {
			for c := b; c <= 1000; c++ {
				if a+b+c == 1000 && a*a+b*b == c*c {
					println(a, b, c, a*b*c)
				}
			}
		}
	}
}
