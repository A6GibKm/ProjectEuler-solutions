fn triangle(n: u64) -> u64 {
    n * ( n + 1 ) /2
}

fn divisors(n: u64) -> u64 {
    let mut number: u64 = 0;
    for i in 1..n+1 {
        if n % i == 0 {
            number += 1;
        }
    }
    number
}

fn main() {
    let mut n: u64 = 1;
    let mut i: u64 = 0;
    while n < 500 {
        let a: u64;
        let b: u64;
        i += 1;
        if i % 2 == 0 {
            a = divisors(i/2);
            b = divisors(i+1);
        } else {
            a = divisors(i);
            b = divisors((i+1)/2);
        }
        n = a*b;
    }
    println!("result: {}, {}", i, triangle(i))
}
