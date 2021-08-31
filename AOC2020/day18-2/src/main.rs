mod evaluator;
use std::fs;

use evaluator::iterative_evaluator;

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    println!(
        "{:?}",
        read.split("\r\n")
            .map(|expr| iterative_evaluator(&expr.chars().collect::<Vec<_>>()))
            .collect::<Vec<_>>().iter().sum::<u64>()
    );
}
