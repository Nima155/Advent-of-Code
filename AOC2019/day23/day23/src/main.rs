use std::{
    collections::{HashMap, VecDeque},
    fs,
    sync::{Arc, Mutex},
    thread,
};

fn main() {
    let read = fs::read_to_string("./input.txt").unwrap();

    let mut program = read
        .split(',')
        .enumerate()
        .map(|(i, f)| (i, f.parse::<i64>().unwrap()))
        .collect::<HashMap<_, _>>();

    run_the_program(&mut program);
}

// Opcode 3 takes a single integer as input and saves it to the position given by its only parameter.
// For example, the instruction 3,50 would take an input value and store it at address 50.
// Opcode 4 outputs the value of its only parameter. For example, the instruction 4,50 would output the value at address 50.

// Now, your ship computer will also need to handle parameters in mode 1, immediate mode. In immediate mode,
// a parameter is interpreted as a value - if the parameter is 50, its value is simply 50.
// 0 is position mode

// first 2 op codes then rest position modes
fn run_the_program(program: &mut HashMap<usize, i64>) -> i64 {
    let queue = Arc::new(Mutex::new(HashMap::with_capacity(50)));
    let comps = (0..50).collect::<Vec<_>>();

    let mut handles = Vec::new();
    for comp in comps {
        let mut program = program.clone();
        let queue = queue.clone();

        let th = thread::spawn(move || {
            let comp = comp;
            // println!("{}", comp);
            let (mut instruction_pointer, mut last_ot_255) = (0, 0);
            let (mut relative_base, mut address_given, mut last_address) = (0, false, -1);

            // println!("{}", comp);
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
                            let ot = if cur_instruction == 104 {
                                parameter
                            } else {
                                *program.entry(parameter as usize).or_default()
                            };
                           
                            let mut guard = queue.lock();
                            output_handler(guard.as_deref_mut().unwrap(), &mut last_address, ot, &mut last_ot_255)
                        }
                        204 => {
                            let ot = *program
                                .entry((parameter + relative_base) as usize)
                                .or_default();
                            
                            let mut guard = queue.lock();
                            output_handler(guard.as_deref_mut().unwrap(), &mut last_address, ot, &mut last_ot_255);
                        }

                        3 => {
                            let mut qu = queue.lock();
                            let qu = qu.as_deref_mut().unwrap();
                            if address_given {
                                if let Some(v) = qu.get_mut(&comp) {
                                    program.insert(
                                        parameter as usize,
                                        v.pop_front().or(Some(-1)).unwrap(),
                                    );
                                } else {
                                    program.insert(parameter as usize, -1);
                                }
                            } else {
                                program.insert(parameter as usize, comp as i64); // change to 2 for answer to part 2
                                address_given = true;
                            }
                        }

                        203 => {
                            // change to 2 for answer to part 2
                            let mut qu = queue.lock();
                            let qu = qu.as_deref_mut().unwrap();
                            if address_given {
                                if let Some(v) = qu.get_mut(&comp) {
                                    program.insert(
                                        (parameter + relative_base) as usize,
                                        v.pop_front().or(Some(-1)).unwrap(),
                                    );
                                } else {
                                    program.insert((parameter + relative_base) as usize, -1);
                                }
                            } else {
                                program.insert((parameter + relative_base) as usize, comp as i64);
                                address_given = true;
                            }
                        }

                        99 => {
                            break;
                        }
                        _ => {}
                    }
                    instruction_pointer += 2;
                }
            }
        });
        handles.push(th);
    }
    for h in handles {
        h.join().unwrap();
    }
    0
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

fn output_handler(queue: &mut HashMap<usize, VecDeque<i64>>, last_address: &mut i64, ot: i64, status_255: &mut i32) {
    if *last_address == -1 {
        if ot == 255 {
            *status_255 += 1;
        }
        *last_address = ot;
    } else {
        *status_255 += (*status_255 != 0) as i32;
        if *status_255 == 3 {
            println!("{}", ot);
        }
        let entry = queue
            .entry(*last_address as usize)
            .or_insert(VecDeque::new());

        entry.push_back(ot);
        if entry.len() % 2 == 0 {
            *last_address = -1;
        }
    }
}
