use std::fs;

fn main() {
    let read = fs::read_to_string("input.txt").unwrap();

    let nums = read.split("\r\n").map(|num| num.parse().unwrap()).collect::<Vec<u64>>();
    
    for i in  25..nums.len() {
        let mut there_is_match = false;
        
        'a:
        for j in i - 25..i {
            for x in i - 25..i {
                if j != x && (nums[j] + nums[x]) == nums[i] {
                    there_is_match = true;
                    break 'a
                }
            }
        }
        if !there_is_match {
            println!("{}", nums[i]);
            break;
        }
    }

}
