use std::{collections, fs};
// Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). 
// What is the value of the accumulator after the program terminates?
fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let vec = read
        .split("\r\n")
        .map(|instruct| instruct.split(' ').collect())
        .collect::<Vec<Vec<_>>>();

    let mut acc = 0;
    let mut cur_indx: i32 = 0;

    let mut visited = collections::HashSet::new();
    loop {
        let instruction = &vec[cur_indx as usize];

        if visited.contains(&cur_indx) {
            break println!("{}", acc);
        }

        visited.insert(cur_indx);

        match instruction[0] {
            "acc" => {
                acc += instruction[1].parse::<i32>().unwrap();
                cur_indx += 1
            }
            "jmp" => cur_indx += instruction[1].parse::<i32>().unwrap(),
            "nop" => {
                cur_indx += 1;
            }
            _ => {}
        }
    }
}
