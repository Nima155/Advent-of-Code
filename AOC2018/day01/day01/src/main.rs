use std::fs;
fn main() {
    let all_nums = fs::read_to_string("../input.txt").unwrap();

    println!("{}", all_nums.split("\r\n").map(|n| n.parse::<i64>().unwrap()).sum::<i64>());
}
