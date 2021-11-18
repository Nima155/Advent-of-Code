mod logic;
use std::{collections::HashMap, fs};

use logic::simulate_the_process;

fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let rules = lines
        .split("\r\n")
        .map(|l| {
            let mappings = l.split(" => ").collect::<Vec<_>>();
            (mappings[0], mappings[1])
        })
        .collect::<HashMap<_, _>>();

    simulate_the_process(&rules, 5);
    simulate_the_process(&rules, 18);
}
