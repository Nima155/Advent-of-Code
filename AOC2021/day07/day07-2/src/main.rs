use std::fs;
fn main() {
    let nums = fs::read_to_string("../input.txt").unwrap();
    let nums = nums
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    

    let length = nums.len();
    
    let mean_of_fuel: i64 = (nums.iter().map(|i| *i as f64).sum::<f64>() / length as f64 - 0.1).round() as i64;
    
    
    let mut ans = 0;

    for fuel in &nums {
        ans += gauss_sum((mean_of_fuel - fuel).abs());
    }
    println!("{}", ans);
}
// 93214084 too high!
fn gauss_sum(n: i64) -> i64 {
    (n * (n + 1)) / 2
}