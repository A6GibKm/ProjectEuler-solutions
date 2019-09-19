use num_bigint::BigUint;
use num_traits::cast::FromPrimitive;
use num_traits::One;

fn main() {
    let two = &BigUint::from_u64(2).unwrap();
    let mut a: BigUint = One::one();
    for _ in 0..1000 {
        a *= two;
    }
    let s: String = a.to_string();
    let ss: &str = &s;
    let mut result: u32 = 0;
    for x in ss.chars() {
        result += x.to_digit(10).unwrap();
    }
    println!("2^n =  {}", a);
    println!("Solution: {}", result);
}
