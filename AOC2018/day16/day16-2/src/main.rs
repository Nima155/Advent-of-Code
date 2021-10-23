mod controller;
use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let all_lines = fs::read_to_string("../input.txt").unwrap();

    let parts = all_lines.split("\r\n\r\n\r\n\r\n").collect::<Vec<_>>();

    let part1 = parts[0].split("\r\n\r\n").collect::<Vec<_>>();

    // println!("{:?}", part1);

    let mut mappings: HashMap<i64, HashSet<i64>> = HashMap::new();

    for p in part1 {
        let mut op_codes_to_ops = 0;
        let (matches, op_num) = test_input(p, &mut op_codes_to_ops);
        let mut indx = 0;

        while indx < 17 {
            let vc = mappings.entry(op_num).or_default();

            if matches >> indx & 1 != 0 {
                vc.insert(indx);
            }

            indx += 1;
        }
    }

    recursive_map(&mut mappings, &mut HashSet::new(), None);

    let part2 = parts[1]
        .split("\r\n")
        .map(|f| {
            f.split(' ')
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    run_program(&mappings, &part2);
}

fn run_program(final_mappings: &HashMap<i64, HashSet<i64>>, part2: &[Vec<i64>]) {
    use controller::Controller;

    let mut initial_prog = Controller::new(vec![0, 0, 0, 0]);

    for ops in part2 {
        initial_prog.new_ops(ops);
        let which_op = *final_mappings.get(&ops[0]).unwrap().iter().next().unwrap();
        initial_prog = match which_op {
            0 => initial_prog.add_i(),
            1 => initial_prog.add_r(),
            2 => initial_prog.mul_i(),
            3 => initial_prog.mul_r(),
            4 => initial_prog.ban_i(),
            5 => initial_prog.ban_r(),
            6 => initial_prog.bor_r(),
            7 => initial_prog.bor_i(),
            8 => initial_prog.eqi_r(),
            9 => initial_prog.eqr_i(),
            10 => initial_prog.eqr_r(),
            11 => initial_prog.gti_r(),
            12 => initial_prog.gtr_r(),
            13 => initial_prog.gtr_i(),
            14 => initial_prog.set_i(),
            15 => initial_prog.set_r(),
            _ => Controller::new(vec![0, 0, 0, 0]),
        };
    }
    println!("{:?}", initial_prog.registers[0]);
}

fn recursive_map(
    mappings: &mut HashMap<i64, HashSet<i64>>,
    visited: &mut HashSet<i64>,
    designated_val: Option<i64>,
) {
    let mut to_be_explored = Vec::new();

    for (_, v) in mappings.iter_mut() {
        if designated_val.is_some() && v.contains(&designated_val.unwrap()) && v.len() > 1 {
            v.remove(&designated_val.unwrap());
        }
        if v.len() == 1 {
            to_be_explored.push(*v.iter().next().unwrap());
        }
    }

    for n in to_be_explored {
        if !visited.contains(&n) {
            visited.insert(n);
            recursive_map(mappings, visited, Some(n));
        }
    }
}

fn test_input(cur_data: &str, possible_ops: &mut i64) -> (i64, i64) {
    let unparsed_data = cur_data.split("\r\n").collect::<Vec<_>>();

    let initial_registers = unparsed_data[0].split(": ").collect::<Vec<_>>()[1];
    let initial_registers = initial_registers[1..initial_registers.len() - 1]
        .split(", ")
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let after_registers = unparsed_data[2].split(":  ").collect::<Vec<_>>()[1];
    let after_registers = after_registers[1..after_registers.len() - 1]
        .split(", ")
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let operations: Vec<i64> = unparsed_data[1]
        .split(' ')
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    use controller::Controller;
    let mut controller = Controller::new(initial_registers);
    controller.new_ops(&operations);

    // build a map
    if controller.clone().add_i().registers == after_registers {
        *possible_ops |= 1;
    }
    if controller.clone().add_r().registers == after_registers {
        *possible_ops |= 2;
    }
    if controller.clone().mul_i().registers == after_registers {
        *possible_ops |= 4;
    }
    if controller.clone().mul_r().registers == after_registers {
        *possible_ops |= 8;
    }
    if controller.clone().ban_i().registers == after_registers {
        *possible_ops |= 16;
    }
    if controller.clone().ban_r().registers == after_registers {
        *possible_ops |= 32;
    }
    if controller.clone().bor_r().registers == after_registers {
        *possible_ops |= 64;
    }
    if controller.clone().bor_i().registers == after_registers {
        *possible_ops |= 128;
    }
    if controller.clone().eqi_r().registers == after_registers {
        *possible_ops |= 256;
    }
    if controller.clone().eqr_i().registers == after_registers {
        *possible_ops |= 512;
    }
    if controller.clone().eqr_r().registers == after_registers {
        *possible_ops |= 1024;
    }
    if controller.clone().gti_r().registers == after_registers {
        *possible_ops |= 2048;
    }
    if controller.clone().gtr_r().registers == after_registers {
        *possible_ops |= 4096;
    }
    if controller.clone().gtr_i().registers == after_registers {
        *possible_ops |= 8192;
    }
    if controller.clone().set_i().registers == after_registers {
        *possible_ops |= 16384;
    }
    if controller.clone().set_r().registers == after_registers {
        *possible_ops |= 32768;
    }

    (*possible_ops, operations[0])
}
