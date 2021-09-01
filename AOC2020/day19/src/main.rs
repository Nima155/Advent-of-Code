mod parsers;
use std::fs;

use parsers::main_logic;
fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let lines = read.split("\r\n").collect::<Vec<_>>();

    println!("{}", main_logic(&lines));
}
