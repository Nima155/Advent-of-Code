use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").map(|e| e.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let (mut gamma, mut epsi) = (String::new(), String::new());
    for j in 0..lines[0].len() {
        let column = lines.iter().map(|r| r[j]).collect::<Vec<_>>();
        let ones = column.iter().filter(|e| **e == '1').count();
        if ones > column.len() / 2 {
            epsi.push('0');
            gamma.push('1');
        } else {
            epsi.push('1');
            gamma.push('0');
        }
    } 
    println!("{}", u32::from_str_radix(&epsi, 2).unwrap() * u32::from_str_radix(&gamma, 2).unwrap())
}
