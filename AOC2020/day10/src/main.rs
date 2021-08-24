use std::collections::HashSet;
use std::fs;

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let ordered_nums = read
        .split("\r\n")
        .map(|e| e.parse().unwrap())
        .collect::<HashSet<u16>>();

    let max_num = ordered_nums.iter().max().unwrap();

    let mut start = 0;

    let mut ones = 0;
    let mut threes = 1;

    loop {
        for i in 1..=3 {
            if ordered_nums.contains(&(start + i)) {
                start += i;
                ones += (i == 1) as u32;
                threes += (i == 3) as u32;
                break;
            }
        }
        if start == *max_num {
            break;
        }
    }
    println!("{}, {}, {}", ones * threes, ones, threes);
}
