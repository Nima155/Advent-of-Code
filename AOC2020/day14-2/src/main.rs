mod combinators;
use std::{collections::HashMap, fs};
// use std::ops::Add;
use lazy_static::lazy_static;
use regex::Regex;

use crate::combinators::all_combinations;
fn main() {
    let bits_and_bobs = fs::read_to_string("input.txt").unwrap();

    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\d+"#).unwrap();
    }

    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask = vec![];
    for line in bits_and_bobs.split("\r\n") {
        let rhs = line.split(" = ").collect::<Vec<_>>()[1];
        if let true = line.starts_with("mask") {
            mask = rhs
                .chars()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    '1' => Some((35 - i, 1)),
                    'X' => Some((35 - i, 2)),
                    _ => None,
                })
                .collect::<Vec<_>>()
        } else {
            let to_dec = RE.find(line).unwrap().as_str();

            all_combinations(
                &mask,
                rhs.parse().unwrap(),
                &mut memory,
                0,
                to_dec.parse().unwrap(),
            );
        }
    }

    println!("{}", memory.values().sum::<u64>());
}
