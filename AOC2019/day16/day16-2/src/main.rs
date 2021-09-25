use std::{collections::VecDeque, fs};
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap().repeat(10000);
    // 0, 1, 0, -1
    let starting_point = read[0..7].parse::<usize>().unwrap();
    
    
    
    let ret = run_100_phases(read[starting_point..].to_string());

    println!("{}", &ret[0..8]);
}

fn run_100_phases(mut input: String) -> String {
    
    for _ in 0..100 {
        let mut intermed: VecDeque<char> = VecDeque::with_capacity(205000);
        for  c in input.chars().rev() {
            intermed.push_front(if !intermed.is_empty() {
                char::from_digit(
                    (c.to_digit(10).unwrap() + intermed.front().unwrap().to_digit(10).unwrap()) % 10,
                    10,
                )
                .unwrap()
            } else {
                c
            });
        }   
        input = String::from_iter(intermed.iter());
    }
    input
}
