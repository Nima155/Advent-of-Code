mod combofinders;
use std::collections::HashSet;
use std::fs;
use combofinders::find_possible_arrangements;



fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let ordered_nums = read
        .split("\r\n")
        .map(|e| e.parse().unwrap())
        .collect::<HashSet<u16>>();

    let max_num = ordered_nums.iter().max().unwrap();

    println!("{}", find_possible_arrangements(&ordered_nums, 0, *max_num));
}
