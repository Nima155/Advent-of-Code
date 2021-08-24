use std::fs;

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let nums = read
        .split("\r\n")
        .map(|num| num.parse().unwrap())
        .collect::<Vec<u64>>();

    for i in 25..nums.len() {
        let mut there_is_match = false;

        'a: for j in i - 25..i {
            for x in i - 25..i {
                if j != x && (nums[j] + nums[x]) == nums[i] {
                    there_is_match = true;
                    break 'a;
                }
            }
        }
        if !there_is_match {
            println!("{}", nums[i]);
            'b: for j in 0..i {
                let mut sm = nums[j];
                let (mut min, mut max) = (nums[j], nums[j]);
                for x in j + 1..i {
                    sm += nums[x];
                    min = min.min(nums[x]);
                    max = max.max(nums[x]);

                    if sm > nums[i] {
                        break;
                    }
                    if sm == nums[i] {
                        println!("{}", min + max);
                        break 'b;
                    }
                }
            }
        }
    }
}
