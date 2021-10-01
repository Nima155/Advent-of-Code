use std::{
    collections::{HashMap, VecDeque},
    fs,
};
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let program = read
        .split(',')
        .enumerate()
        .map(|(i, f)| (i, f.parse::<i64>().unwrap()))
        .collect::<HashMap<_, _>>();
    // 21861249
    // 25004376
    let (mut start_x, mut start_y) = (0, 100);

    loop {

        let single_mode = run_the_program(&mut program.clone(), (start_y, start_x), true);
        
        if single_mode && run_the_program(&mut program.clone(), (start_y, start_x), false) {
            println!("{} {} {}lans", (start_y - 99) + start_x * 10000, start_y, start_x);            
            break;
        }
        else if single_mode {
            start_y += 1;
        }
        start_x += 1;
        
    } 

    // run_the_program(&mut program);
}

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter.
// For example, the instruction 3,50 would take an input value and store it at address 50.
// Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.

// Now, your ship computer will also need to handle parameters in mode 1, immediate mode. In immediate mode,
// a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.
// 0 is position mode

// first 2 op codes then rest position modes
fn run_the_program(program: &mut HashMap<usize, i64>, point: (i64, i64), single_mode: bool) -> bool {
    let mut inputs = if !single_mode  {vec![point.0, point.1, point.0 - 99, point.1, point.0 , point.1 + 99, point.0 - 99 , point.1 + 99]} else {vec![point.0, point.1]} ;
    let mut ans = 0;
    while !inputs.is_empty() {
        let mut instruction_pointer = 0;
        let mut relative_base = 0;
        let mut program = program.clone();
        loop {
            let cur_instruction = *program.entry(instruction_pointer).or_default();
            let last_digit = cur_instruction % 10;
            // println!("instruction: {}", cur_instruction);
            if cur_instruction != 99
                && is_valid_instruction(cur_instruction)
                && is_valid_op(cur_instruction)
                || (cur_instruction < 10 && is_valid_op(cur_instruction))
            {
                let mut modes = extract_parameter_modes(cur_instruction);

                let mut arrs = [
                    *program.entry(instruction_pointer + 1).or_default(),
                    *program.entry(instruction_pointer + 2).or_default(),
                    *program.entry(instruction_pointer + 3).or_default(),
                ];
                // println!("instruction: {}", cur_instruction);
                let length = arrs.len() - 1;
                for i in arrs[..length].iter_mut() {
                    let mode = modes.pop_back().or(Some(0)).unwrap();
                    if mode != 1 {
                        // println!("{} {}", *i  + is_in_2nd_gear(mode, relative_base), cur_instruction);
                        *i = *program
                            .entry((*i + is_in_2nd_gear(mode, relative_base)) as usize)
                            .or_default();
                    }
                }
                // println!("{} inst", cur_instruction);
                let [indx_f1, indx_f2, indx_dest_u] = arrs;
                let indx_dest_u = if !modes.is_empty() && modes.pop_back().unwrap() == 2 {
                    indx_dest_u + relative_base
                } else {
                    indx_dest_u
                } as usize;

                match last_digit {
                    1 => {
                        program.insert(indx_dest_u, indx_f1 + indx_f2);
                    }
                    2 => {
                        program.insert(indx_dest_u, indx_f1 * indx_f2);
                    }
                    7 => {
                        program.insert(indx_dest_u, if indx_f1 < indx_f2 { 1 } else { 0 });
                    }
                    8 => {
                        program.insert(indx_dest_u, if indx_f1 == indx_f2 { 1 } else { 0 });
                    }
                    5 if indx_f1 != 0 => {
                        instruction_pointer = indx_f2 as usize;
                        continue;
                    }
                    6 if indx_f1 == 0 => {
                        instruction_pointer = indx_f2 as usize;
                        continue;
                    }

                    9 => {
                        relative_base += indx_f1;
                        instruction_pointer += 2;
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
                let parameter = *program.entry(instruction_pointer + 1).or_default();
                // println!("param: {} cur_inst: {} offset: {}", parameter, cur_instruction, relative_base);

                // println!("{} ops", cur_instruction);
                match cur_instruction {
                    // 203 too low
                    4 | 104 => {
                        ans += 
                            if cur_instruction == 104 {
                                parameter
                            } else {
                                *program.entry(parameter as usize).or_default()
                            };
                        
                    }
                    204 => {
                        ans += 
                            *program
                                .entry((parameter + relative_base) as usize)
                                .or_default();
                        
                    }

                    3 => {
                        // pri/ntln!("{}", 1);
                        program.insert(parameter as usize, inputs.pop().unwrap());
                        // change to 2 for answer to part 2
                    }

                    203 => {
                        // println!("{}", 1);
                        program.insert(
                            (parameter + relative_base) as usize,
                            inputs.pop().unwrap(),
                        ); // change to 2 for answer to part 2
                    }

                    99 => {
                        break;
                    }
                    _ => {}
                }
                instruction_pointer += 2;
            }
        }
    }
    (ans == 4 && !single_mode) || (ans == 1 && single_mode)
}

fn is_valid_instruction(mut num: i64) -> bool {
    num /= 100;
    while num > 0 {
        if num % 10 != 0 && num % 10 != 1 && num % 10 != 2 {
            return false;
        }

        num /= 10
    }
    true
}
fn is_valid_op(num: i64) -> bool {
    let last_dig = num % 10;

    [1, 2, 7, 8, 5, 6, 9].contains(&last_dig)
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

fn is_in_2nd_gear(mode: i64, relative_base: i64) -> i64 {
    if mode == 2 {
        relative_base
    } else {
        0
    }
}
