use std::{collections::HashMap, fs};
// use std::ops::Add;
use lazy_static::lazy_static;
use regex::Regex;
fn main() {
    let bits_and_bobs = fs::read_to_string("input.txt").unwrap();

    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\d+"#).unwrap();
    }

    let mut memory: HashMap<&str, u64> = HashMap::new();

    let mut mask = vec![];
    for line in bits_and_bobs.split("\r\n") {
        let rhs = line.split(" = ").collect::<Vec<_>>()[1];
        if let true = line.starts_with("mask") {
            mask = rhs
                .chars()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    '0'..='1' => Some((35 - i, c.to_digit(10).unwrap())),
                    _ => None,
                })
                .collect::<Vec<_>>()
        } else {
            let mut to_number = rhs.parse::<u64>().unwrap();
            for (shift, bit) in mask.iter() {
                if let 1 = bit {
                    to_number |= 1u64 << (*shift as u64);
                } else {
                    to_number &= !(1u64 << shift);
                }
            }

            memory.insert(RE.find(line).unwrap().as_str(), to_number);
        }
    }
    println!("{}", memory.values().sum::<u64>());
}
