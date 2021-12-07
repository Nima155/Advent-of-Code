use std::fs;
fn main() {
    let nums = fs::read_to_string("../test.txt").unwrap();
    let mut nums = nums
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();

    let length = nums.len();
    
    let median_of_fuel = (nums[length / 2] + nums[length / 2 - 1]) / 2;
    

    let mut ans = 0;

    for fuel in &nums {
        ans += (median_of_fuel - fuel).abs();
    }
    println!("{}", ans);
}
