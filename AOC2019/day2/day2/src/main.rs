use std::fs;
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let mut nums = read.split(',').map(|c| c.parse::<u64>().unwrap()).collect::<Vec<_>>();

    nums[1] = 12;
    nums[2] = 2;

    println!("{}", play_the_game(&mut nums))
}
fn play_the_game(program: &mut Vec<u64>) -> u64 {
    let mut instruction_pointer = 0;

    while instruction_pointer < program.len() {
        
        let (indx_dest, indx_f1, indx_f2) = (program[instruction_pointer + 3] as usize, program[instruction_pointer + 1] as usize, program[instruction_pointer + 2] as usize);


        match program[instruction_pointer] {
            1 => {
                program[indx_dest] = program[indx_f1] + program[indx_f2];
            } ,
            2 => {
                program[indx_dest] = program[indx_f1] * program[indx_f2];
            }
            99 => { break }
            _ => {}
        };


        instruction_pointer += 4;

    }
    program[0]
}