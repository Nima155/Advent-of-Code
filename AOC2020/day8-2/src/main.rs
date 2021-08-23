use std::fs;

use interpreter::interpreter;
mod interpreter;

// Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). 
// What is the value of the accumulator after the program terminates?
fn main() {
    let read = fs::read_to_string("input.txt").unwrap();
    
    let vec = read
        .split("\r\n")
        .map(|instruct| instruct.split(' ').collect())
        .collect::<Vec<Vec<_>>>();

    println!("{}", interpreter(vec));
   
}