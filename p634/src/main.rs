// Define F(n) to be the number of integers x≤n that can be written in the form x=a2b3, where a and b.

// are integers not necessarily different and both greater than 1.

// For example, 32=22×23
// and 72=32×23 are the only two integers less than 100 that can be written in this form. Hence, F(100)=2

// Further you are given F(2×104)=130
// and F(3×106)=2014

// Find F(9×1018)

use std::str::FromStr;
use num_bigint::BigUint;
// use num_traits::cast::FromPrimitive;
// use num_traits::{One};

// let two = &BigUint::from_u64(2).unwrap();

fn x(a:u64, b:u64) -> u64 {
    a*a*b*b*b
}

fn main() {
    let a_max = &BigUint::from_str("1500000000").unwrap();
    let b_max = &BigUint::from_str("3000000").unwrap();
    let n: u64 = &BigUint::from_str("9000000000000000000").unwrap();;
    let mut s: u64 = 0;
    for a in 2..a_max {
        for b in 2..b_max {
            if x(a,b) <= n {
                s += 1;
            }
        }
    }
    println!("{}", s);
}
