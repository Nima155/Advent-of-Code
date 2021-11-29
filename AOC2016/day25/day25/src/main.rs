use std::fs;
fn main() {
    let instructions = fs::read_to_string("../input.txt").unwrap();
    let instructions = instructions.split("\r\n").collect::<Vec<_>>();

    for i in 0.. {
        let ret = execute_program(&instructions, &mut [i, 0, 0, 0]);

        if ret {
            println!("{}", i);
            break;
        }
    }
}

fn is_valid(last: i64, cur: i64) -> bool {
    last == -1 && cur == 0 || last != -1 && cur ^ 1 == last
}

fn get_indx(num_in_str: &str) -> usize {
    (num_in_str.chars().next().unwrap() as u8 - b'a') as usize
}

fn execute_program(instructions: &[&str], registers: &mut [i64; 4]) -> bool {
    // let mut registers = [0, 0, 0, 0]; // subsitute the zero at index 2 to 1 to get answer to part 2
    let mut i = 0_i64;
    let mut last = -1;
    let mut cyc = 0;
    while i >= 0 && i < instructions.len() as i64 && cyc < 10 {
        let instruction = instructions[i as usize];
        // println!("{}", instruction);
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
            "out" => {
                let val;
                if let Ok(iz) = split_up[1].parse::<i64>() {
                    val = iz;
                } else {
                    val = registers[get_indx(split_up[1])];
                }

                if !is_valid(last, val) {
                    return false;
                }
                last = val;
                cyc += 1;
            }
            _ => {}
        }
        i += 1;
    }
    
    true
}
