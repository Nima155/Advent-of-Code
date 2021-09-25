use std::fs;
fn main() {
    let read = fs::read_to_string("../input.txt").unwrap();
    // 0, 1, 0, -1
    run_100_phases(produce_pattern_iterations(read.len()), read);
}

fn produce_pattern_iterations(cycle: usize) -> Vec<Vec<i32>> {
    let mut ans = vec![vec![0, 1, 0, -1]];
    
    while ans.len() < cycle {
        let back = ans.last().unwrap();
        let (mut lst, mut intermed) = (-12, Vec::with_capacity(back.len()));
        for n in back {
            if *n != lst {
                intermed.push(*n);
            }
            intermed.push(*n);
            lst = *n;
        }
        
        ans.push(intermed);
    }
    ans
}

fn run_100_phases(patterns: Vec<Vec<i32>>, mut input: String) {
    
    for j in 0..100 {
        let mut intermed = String::new();
        for (i, c) in input.chars().enumerate() {
            let mut sum = 0;
            for (d, mult_by) in input.chars().zip(patterns[i].iter().cycle().skip(1)) {    
                sum += d.to_digit(10).unwrap() as i32 * *mult_by;
            }
            
            sum = sum.abs() % 10;
            
            intermed.push(char::from_digit(sum as u32, 10).unwrap());
        }
        
        input = intermed;
    }
    println!("{}", &input[0..8])
}