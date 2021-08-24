use std::collections::HashSet;

pub fn find_possible_arrangements(ordered_nums: &HashSet<u16>, cur: u16, max_of: u16) -> u64 {
    let mut memo = [0u64; 200];
    fn finder(ordered_nums: &HashSet<u16>, cur: u16, max_of: u16, memo: &mut [u64]) -> u64 {
        if memo[cur as usize] != 0 {
            return memo[cur as usize];
        }

        if cur == max_of {
            return 1;
        }

        let mut ans: u64 = 0;
        for num in 1..4 {
            if ordered_nums.contains(&(cur + num)) {
                ans += finder(ordered_nums, cur + num, max_of, memo);
            }
        }
        memo[cur as usize] = ans;
        ans
    }
    finder(ordered_nums, cur, max_of, &mut memo)
}
