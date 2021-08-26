use std::fs;
fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let to_vec = read.split("\r\n").collect::<Vec<_>>();

    let guesstimate: i64 = to_vec[0].parse().unwrap();

    let mut wait_time = std::i64::MAX;

    let mut id = std::i64::MAX;

    to_vec[1].split(',').for_each(|num| {
        if let Ok(v) = num.parse::<i64>() {
            let value = (guesstimate / v) * v + if guesstimate % v != 0 { v } else { 0 };

            if (value - guesstimate) < wait_time {
                wait_time = value - guesstimate;
                id = v;
            }
        }
    });
    println!("{} {} {}", id * wait_time, id, wait_time);
}
