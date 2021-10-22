mod controller;
use std::fs;

fn main() {
    let all_lines = fs::read_to_string("../input.txt").unwrap();

    let parts = all_lines.split("\r\n\r\n\r\n\r\n").collect::<Vec<_>>();

    let part1 = parts[0].split("\r\n\r\n").collect::<Vec<_>>();

    // println!("{:?}", part1);
    let mut tc = 0;
    for p in part1 {
        tc += test_input(p) as usize;
    }
    println!("{}", tc);
}
fn test_input(cur_data: &str) -> bool {
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

    let operations = unparsed_data[1]
        .split(' ')
        .map(|c| c.parse::<i64>().unwrap())
        .collect();

    let controller = controller::Controller::new(initial_registers, operations);

    let mut counts = 0;

    counts += (controller.clone().add_i().registers == after_registers) as i32;
    counts += (controller.clone().add_r().registers == after_registers) as i32;
    counts += (controller.clone().mul_i().registers == after_registers) as i32;
    counts += (controller.clone().mul_r().registers == after_registers) as i32;
    counts += (controller.clone().ban_i().registers == after_registers) as i32;
    counts += (controller.clone().ban_r().registers == after_registers) as i32;
    counts += (controller.clone().bor_r().registers == after_registers) as i32;
    counts += (controller.clone().bor_i().registers == after_registers) as i32;
    counts += (controller.clone().eqi_r().registers == after_registers) as i32;
    counts += (controller.clone().eqr_i().registers == after_registers) as i32;
    counts += (controller.clone().eqr_r().registers == after_registers) as i32;
    counts += (controller.clone().gti_r().registers == after_registers) as i32;
    counts += (controller.clone().gtr_r().registers == after_registers) as i32;
    counts += (controller.clone().gtr_i().registers == after_registers) as i32;
    counts += (controller.clone().set_i().registers == after_registers) as i32;
    counts += (controller.clone().set_r().registers == after_registers) as i32;

    counts >= 3
}
