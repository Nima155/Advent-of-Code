use std::{fs, collections::HashSet};
fn main() {
    let all_nums = fs::read_to_string("../input.txt").unwrap();

    let mut visited = HashSet::new();

    let mut fr = 0;
    for n in all_nums.split("\r\n").map(|n| n.parse::<i64>().unwrap()).cycle() {
        if visited.contains(&(n + fr)) {
            println!("{}", n + fr);
            break;
        }
        fr += n;
        visited.insert(fr);
    }
    
}
