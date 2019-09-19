use rayon::prelude::*;

// If we calculate a2 mod 6 for 0 ≤ a ≤ 5 we get: 0,1,4,3,4,1.

// The largest value of a such that a^2 ≡ a mod 6 is 4.
// Let's call M(n) the largest value of a < n such that a^2 ≡ a (mod n).
// So M(6) = 4.

// Find ∑M(n) for 1 ≤ n ≤ 10^7.

fn m(n: u64) -> u64 {
    // let prime_divs = prime_divisors(n, prime_list);
    // if prime_divs.len() == 1 {
    //     return 1
    // }
    //
    if n == 0 || n ==1 {
        return 0 as u64
    }
    let lower_bound: u64 = (n as f64).sqrt() as u64 + 1;
    let upper_bound: u64 = (n + 1)/2 + 1;
    for i in lower_bound..upper_bound {
        if (i*i - i) % n == 0 {
           return n + 1 - i
        }
    }
    return 1
}

// fn m(n: u64, prime_list: &Vec<bool>) -> u64 {
//     let mut min: u64 = n;
//     let prime_divs = prime_divisors(n, prime_list);
//     if prime_divs.len() == 1 {
//         1
//     } else {
//         for p in prime_divs.iter() {
//             min = match e(n, *p) < min {
//                 true => e(n, *p),
//                 false => min,
//             }
//         }
//         n + 1 - min
//     }
// }

// fn prime_list_gen(n: u64) -> Vec<bool> {
//     // let upper_limit = (n as f64).sqrt() as u64 + 1;
//     let upper_limit = n / 2 + 1;
//     let mut erast: Vec<bool> = vec![true; n as usize];
//     for i in 2..upper_limit {
//         if erast[i as usize] {
//             let mut j = i * i;
//             while j < n {
//                 erast[j as usize] = false;
//                 j += i
//             }
//         }
//     }
//     println!("primes below {} indexed", n);
//     erast
// }

// fn mod_inv(a: u64, module: u64) -> u64 {
//   let mut mn = (module, a);
//   let mut xy = (0, 1);

//   while mn.1 != 0 {
//     xy = (xy.1, xy.0 - (mn.0 / mn.1) * xy.1);
//     mn = (mn.1, mn.0 % mn.1);
//   }

//   while xy.0 < 0 {
//     xy.0 += module;
//   }
//   xy.0
// }

// fn extended_gcd(a: u64, b: u64) -> (u64, u64) {
//     let mut s: u64 = 0;
//     let mut r: u64 = b.try_into().unwrap();
//     let mut t: u64 = 1;
//     let mut old_s: u64 = 1;
//     let mut old_r: u64 = a.try_into().unwrap();
//     let mut old_t: u64 = 0;
//     let mut quotient: u64;
//     while r != 0 {
//         quotient = old_r / r;
//         old_r = r;
//         r = old_r - quotient * r;
//         old_s = s;
//         s = old_s - quotient * s;
//         old_t = t;
//         t = old_t - quotient * t;
//     }
//     (old_s, old_t)
// }

// fn prime_divisors(n: u64, prime_list: &Vec<bool>) -> Vec<u64> {
//     let mut list: Vec<u64> = Vec::new();
//     let upper_limit = n / 2 + 1;
//     for i in 2..upper_limit {
//         if prime_list[i as usize] && n % i == 0 {
//             list.push(i);
//         }
//     }
//     list
// }

// fn e(n: u64, p: u64) -> u64 {
//     assert!(n % p == 0, "p does not divide n");
//     assert!(p != 0, "div by zero");
//     let mut q: u64 = p;
//     while n % (p * q) == 0 {
//         q *= p;
//     }
//     let x = mod_inv(q,n/q);
//     let mut e = q * x % n;
//     e = match e < n + 1 - e {
//         true => e,
//         false => n + 1 - e,
//     };
//     e
// }

// fn e(n: u64, p: u64) -> u64 {
//     assert!(n % p == 0, "p does not divide n");
//     assert!(p != 0, "div by zero");
//     let mut q: u64 = p;
//     while n % (p * q) == 0 {
//         q *= p;
//     }
//     let mut x: u64 = 1;
//     for _ in 0..q - 1 {
//         x *= n / q;
//         x = x % n;
//     }
//     x
// }

fn compute(n: u64) -> u64 {
    let mut ints: Vec<u64> = vec![0; (n + 1) as usize];
    for i in 0..n + 1 {
        ints[i as usize] = i;
    }
    ints.par_iter()
        .map(|&i| m(i))
        .sum()
    // let mut s: u64 = 0;
    // for r in 2..n + 1 {
    //     s += m(r);
    //     // if r % (n / 100) == 0 {
    //     //     println!("at n = {}", r)
    //     // }
    // }
    // s
}

// fn compute(n: u64) -> u64 {
//     // let primes: Vec<bool> = prime_list_gen(n);
//     let mut s: u64 = 0;
//     for r in 2..n + 1 {
//         // s += m(r, &primes);
//         s += m(r);
//         if r % (n / 100) == 0 {
//             println!("at n = {}", r)
//         }
//     }
//     s
// }

fn main() {
    const N: u64 = 10000000;
    println!("Result: {}", compute(N));
}

#[cfg(test)]
mod tests {
    use super::{compute, m};

    const N: u64 = 2 * 3 * 5 * 7 * 11;

    #[test]
    fn tests1() {
        // let primes = prime_list_gen(N);
        let mut an = 1;
        for s in 2..N {
            if (s * s - s) % N == 0 {
                an = s;
            }
        }
        assert_eq!(4, m(6));
        assert_eq!(an, m(N));
        assert_eq!(1, m(4));
    }

    #[test]
    fn test2() {
        assert_eq!(17, compute(10));
    }

}
