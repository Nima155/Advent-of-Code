use std::fs;
fn main() {
    let instructions = fs::read_to_string("../input.txt").unwrap();
    let instructions = instructions.split("\r\n").collect::<Vec<_>>();

    println!("{}", execute_program(&instructions));
}

fn get_indx(num_in_str: &str) -> usize {
    (num_in_str.chars().next().unwrap() as u8 - b'a') as usize
}

fn execute_program(instructions: &[&str]) -> i64 {
    let mut registers = [0, 0, 0, 0]; // subsitute the zero at index 2 to 1 to get answer to part 2
    let mut i = 0_i64;
    while i >= 0 && i < instructions.len() as i64 {
        let instruction = instructions[i as usize];
        let split_up = instruction.split(' ').collect::<Vec<_>>();
        match split_up[0] {
            "cpy" => {
                if let Ok(i) = split_up[1].parse::<i64>() {
                    registers[get_indx(split_up[2])] = i;
                } else {
                    registers[get_indx(split_up[2])] = registers[get_indx(split_up[1])];
                }
            }
            "inc" | "dec" => {
                registers[get_indx(split_up[1])] += if split_up[0] == "inc" { 1 } else { -1 };
            }

            "jnz" => {
                let x;
                if let Ok(i) = split_up[1].parse::<i64>() {
                    x = i;
                } else {
                    x = registers[get_indx(split_up[1])];
                }
                if x != 0 {
                    i += split_up[2].parse::<i64>().unwrap();
                    continue;
                }
            }
            _ => {}
        }
        i += 1;
    }
    registers[0]
}
