use std::fs;
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    println!("{}", read.split("\r\n").map(|c| c.parse::<u64>().unwrap() / 3 - 2).sum::<u64>());
}
