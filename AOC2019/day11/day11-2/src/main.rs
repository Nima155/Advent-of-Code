mod intcode;

use std::{collections::HashMap, fs};

use intcode::{run_the_program, build_grid};

fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let mut program = read
        .split(',')
        .enumerate()
        .map(|(i, f)| (i, f.parse::<i64>().unwrap()))
        .collect::<HashMap<_, _>>();

    build_grid(&run_the_program(&mut program));
}
