use std::fs;
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();

    let nums = read.split(',').map(|c| c.parse::<u64>().unwrap()).collect::<Vec<_>>();

    const DESIRED_OUTCOME: u64 = 19690720;

    'a: for n in 0..100 {
        for j in 0..100 {
            if n != j {
                let mut nums_clone = nums.clone();
                nums_clone[1] = n;
                nums_clone[2] = j;
                if play_the_game(&mut nums_clone) == DESIRED_OUTCOME {
                    println!("{}",100 * n + j);
                    break 'a;
                }
            }
        }
    }

    

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