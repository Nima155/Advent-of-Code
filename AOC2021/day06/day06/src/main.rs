use std::fs;
fn main() {
    let nums = fs::read_to_string("../input.txt").unwrap();
    let nums = nums.as_str().split(',').map(|n| n.parse::<i64>().unwrap()).collect::<Vec<_>>();
    println!("{}", simulate_growth(nums));
}

const CYCLES: i64 = 80;
fn simulate_growth(mut nums: Vec<i64>) -> i64 {
    

    for _ in 0..CYCLES {
        let mut cloned_nums = Vec::new();
        for lantern_fish in nums {

            if lantern_fish == 0 {
                cloned_nums.extend([6, 8].iter());
            } else {
                cloned_nums.push(lantern_fish - 1);
            }

        }
        
        nums = cloned_nums
    }
    nums.len() as i64

}