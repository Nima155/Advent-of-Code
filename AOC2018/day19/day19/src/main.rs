mod controller;
use std::fs;

fn main() {
    let all_lines = fs::read_to_string("../input.txt").unwrap();

    let separated_lines = all_lines.split("\r\n").collect::<Vec<_>>();

    // println!("{:?}", part1);

    run_program(&separated_lines);
}
fn op_parser(ins: &str) -> (&str, Vec<i64>) {
    let vectorized = ins.split(' ').collect::<Vec<_>>();

    return (
        vectorized[0],
        vectorized[1..]
            .iter()
            .map(|l| l.parse::<i64>().unwrap())
            .collect::<Vec<_>>(),
    );
}
fn run_program(separated_lines: &Vec<&str>) {
    use controller::Controller;

    let mut initial_prog = Controller::new(vec![0, 0, 0, 0, 0, 0], 1);


    let instructions = separated_lines[1..].iter().map(|inst| op_parser(inst)).collect::<Vec<_>>();

    // let mut cycle = 0;

    while let Some(indx) =  initial_prog.test_ip(instructions.len()) {
        // cycle += 1;
        let (op, ref operations) = instructions[indx];
        // if cycle % 1000000 == 0 {
        //     println!("{} {:?}", op, operations);
        // }
        initial_prog.new_ops(&operations);

        initial_prog = match op.trim() {
            "addi" => initial_prog.add_i(),
            "addr" => initial_prog.add_r(),
            "muli" => initial_prog.mul_i(),
            "mulr" => initial_prog.mul_r(),
            "bani" => initial_prog.ban_i(),
            "banr" => initial_prog.ban_r(),
            "borr" => initial_prog.bor_r(),
            "bori" => initial_prog.bor_i(),
            "eqir" => initial_prog.eqi_r(),
            "eqri" => initial_prog.eqr_i(),
            "eqrr" => initial_prog.eqr_r(),
            "gtir" => initial_prog.gti_r(),
            "gtrr" => initial_prog.gtr_r(),
            "gtri" => initial_prog.gtr_i(),
            "seti" => initial_prog.set_i(),
            "setr" => initial_prog.set_r(),
            _ => Controller::new(vec![0, 0, 0, 0, 0, 0], 1),
        };
        // println!("{}", indx);
        
    }
    println!("{}", initial_prog.registers[0]);
}
