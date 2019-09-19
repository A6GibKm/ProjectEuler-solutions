use std::io::{BufReader,BufRead};
use std::fs::File;

fn main() {
    let file = File::open("numbers").unwrap();
    let mut s: u64 = 0;
    for line in BufReader::new(file).lines() {

        let n: u64 =  line.unwrap()[0..14].parse::<u64>().unwrap();
        s += n
    };
    let string = s.to_string();
    println!("{}", &string[0..10]);
}
