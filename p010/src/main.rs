fn is_prime(n: u64) -> bool {
    if n == 1 {
        return false
        }
    for m in 2..n {
        if n % m == 0 {
            return false
        } else if m * m > n {
            return true
        }
    }
    return true
}

fn main() {
    let mut s: u64 = 0;
    for k in 1..2000001 {
        if is_prime(k) {
            s += k;
        } else if k % 100000 == 0 {
            println!("At {}", k)
        }
    }
    println!("The sum is: {}", s)
}
