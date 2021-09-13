use std::{collections::VecDeque, fs};
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let mut program = read
        .split(',')
        .map(|f| f.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    run_the_program(&mut program);
}

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter.
// For example, the instruction 3,50 would take an input value and store it at address 50.
// Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.

// Now, your ship computer will also need to handle parameters in mode 1, immediate mode. In immediate mode,
// a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.
// 0 is position mode

// first 2 op codes then rest position modes
fn run_the_program(program: &mut Vec<i64>) -> i64 {
    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        let last_digit = program[instruction_pointer] % 10;

        if is_valid_instruction(program[instruction_pointer])
            && is_valid_op(program[instruction_pointer])
            || (program[instruction_pointer] < 10 && is_valid_op(program[instruction_pointer]))
        {
            // println!("{}", program[instruction_pointer]);
            let modes = extract_parameter_modes(program[instruction_pointer]);

            let (indx_dest, mut indx_f1, mut indx_f2) = (
                program[instruction_pointer + 3],
                program[instruction_pointer + 1],
                program[instruction_pointer + 2],
            );

            indx_f1 = if modes.is_empty() || *modes.back().unwrap() == 0 {
                program[indx_f1 as usize]
            } else {
                indx_f1
            };
            indx_f2 = if modes.len() < 2 || modes[modes.len() - 2] == 0 {
                program[indx_f2 as usize]
            } else {
                indx_f2
            };
            // indx_dest = if *modes.front().unwrap() == 0 { program[indx_dest as usize] } else {indx_dest};
            let indx_dest_u = indx_dest as usize;

            match last_digit {
                1 => {
                    program[indx_dest_u] = indx_f1 + indx_f2;
                }
                2 => {
                    program[indx_dest_u] = indx_f1 * indx_f2;
                }
                7 => {
                    program[indx_dest_u] = if indx_f1 < indx_f2 { 1 } else { 0 };
                }
                8 => {
                    program[indx_dest_u] = if indx_f1 == indx_f2 { 1 } else { 0 };
                }
                5 if indx_f1 != 0 => {
                    instruction_pointer = indx_f2 as usize;
                    continue;
                }
                6 if indx_f1 == 0 => {
                    instruction_pointer = indx_f2 as usize;
                    continue;
                }
                5 | 6 => {
                    instruction_pointer += 3;
                    continue;
                }
                // 99 => break,
                _ => {}
            };
            instruction_pointer += 4;
        } else {
            let parameter = program[instruction_pointer + 1];

            match program[instruction_pointer] {
                3 => {
                    program[parameter as usize] = 5;
                }
                
                4 => {
                    println!(" {}", program[parameter as usize])
                }
                99 => {
                    break;
                }
                _ => {}
            }
            instruction_pointer += 2;
        }
    }
    program[0]
}

fn is_valid_instruction(mut num: i64) -> bool {
    num /= 100;
    while num > 0 {
        if num % 10 != 0 && num % 10 != 1 {
            return false;
        }

        num /= 10
    }
    true
}
fn is_valid_op(num: i64) -> bool {
    let last_dig = num % 10;

    [1, 2, 7, 8, 5, 6].contains(&last_dig)
}
fn extract_parameter_modes(mut num: i64) -> VecDeque<i64> {
    let mut ans = VecDeque::with_capacity(5);
    num /= 100;
    while num > 0 {
        ans.push_front(num % 10);
        num /= 10;
    }
    ans.shrink_to_fit();
    ans
}
