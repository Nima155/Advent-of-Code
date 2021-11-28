use std::fs;
fn main() {
    let instructions = fs::read_to_string("../input.txt").unwrap();
    let mut instructions = instructions
        .split("\r\n")
        .map(|e| e.split(' ').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("{}", execute_program(&mut instructions));
}

fn get_indx(num_in_str: &str) -> usize {
    (num_in_str.chars().next().unwrap() as u8 - b'a') as usize
}

const EGGS: i64 = 7;
fn execute_program(instructions: &mut [Vec<&str>]) -> i64 {
    let mut registers = [EGGS, 0, 0, 0];
    let mut i = 0_i64;
    while i >= 0 && i < instructions.len() as i64 {
        let split_up = instructions[i as usize].clone();

        match split_up[0] {
            "cpy" => {
                if let Ok(iz) = split_up[1].parse::<i64>() {
                    registers[get_indx(split_up[2])] = iz;
                } else if split_up[2].parse::<usize>().is_err() {
                    registers[get_indx(split_up[2])] = registers[get_indx(split_up[1])];
                }
            }
            "inc" | "dec" => {
                registers[get_indx(split_up[1])] += if split_up[0] == "inc" { 1 } else { -1 };
            }

            "jnz" => {
                let x;
                if let Ok(iz) = split_up[1].parse::<i64>() {
                    x = iz;
                } else {
                    x = registers[get_indx(split_up[1])];
                }
                if x != 0 {
                    if split_up[2].parse::<i64>().is_ok() {
                        i += split_up[2].parse::<i64>().unwrap();
                        continue;
                    }
                    i += registers[get_indx(split_up[2])];
                    continue;
                }
            }
            "tgl" => {
                let mut new_indx = get_indx(split_up[1]) as i64;
                new_indx = registers[new_indx as usize] + i as i64;
                if new_indx < instructions.len() as i64 && new_indx >= 0 {
                    let point_at_instruction = &mut instructions[new_indx as usize];
                    match point_at_instruction.len() {
                        2 => {
                            point_at_instruction[0] = if point_at_instruction[0] == "inc" {
                                "dec"
                            } else {
                                "inc"
                            }
                        }
                        3 => {
                            point_at_instruction[0] = if point_at_instruction[0] == "jnz" {
                                "cpy"
                            } else {
                                "jnz"
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    registers[0]
}
