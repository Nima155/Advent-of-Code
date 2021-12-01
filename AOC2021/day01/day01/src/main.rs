use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
    let (mut prev, mut count) = (-1, 0);
    for n in &lines {
        if *n > prev {
            count += 1;
        }
        prev = *n;
    }
    println!("{}", count - 1);
}
