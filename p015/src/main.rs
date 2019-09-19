// Not a very good answer as it took more than 2 minutes to compute
// Should be improved to F(n,n) = Comb(2n , n) = (2n)! / (n!)^2

fn ways(n: u64, m: u64)-> u64 {
    if m == 0 || n == 0 {
        return 1
    } else if n == 1 {
        return m + 1
    } else if m == 1 {
        return n + 1
    } else {
        return ways(n-1,m) + ways(n, m-1)
    }
}

fn main() {
    let result = ways(20, 20);
    println!("result: {}", result);
}
