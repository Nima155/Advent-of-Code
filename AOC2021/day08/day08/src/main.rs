use std::fs;
// 8 => 7, 1 => 2, 4 => 4, 7 => 3
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").map(|l| {
        l.split(" | ")
            .map(|part| part.split(' ').collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let mut ans = 0;
    for l in &lines {
        for comb in &l[1] {
            match comb.len() {
                7 | 2 | 4 | 3 => { 
                    ans += 1;
                }
                _ => {}
            }
        }
    }
    println!("{}", ans);
}
