mod evaluator;
use std::fs;

use evaluator::recursive_evaluator;

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    println!(
        "{:?}",
        read.split("\r\n")
            .map(|expr| recursive_evaluator(&expr.chars().collect::<Vec<_>>(), 0))
            .collect::<Vec<_>>().iter().fold(0, |acc, cur| acc + cur.0)
    );
}
