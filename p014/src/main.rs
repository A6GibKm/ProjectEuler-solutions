fn collatz(n : u64) -> u64 {
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

fn main() {
    let mut biggest: u64 = 1;
    for n in 1..1000001 {
        let mut s: u64 = 2;
        let mut m: u64 = collatz(n);
        while m > 1 {
            s += 1;
            m = collatz(m);
        }
        if s >= biggest {
            biggest = s;
            println!("n:{}, s:{}", n, s);
        }
    }
}
