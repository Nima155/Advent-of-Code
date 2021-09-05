mod parsers;
use std::fs;

use parsers::main_logic;
fn main() {
    // 349 too high!, 238 too low!
    let read = fs::read_to_string("mock-input.txt").unwrap();

    let lines = read.split("\r\n").collect::<Vec<_>>();

    println!("{}", main_logic(&lines));
}
