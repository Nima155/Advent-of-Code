use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let lines = lines.split("\r\n").collect::<Vec<_>>();

    let mut full_data = Vec::new();

    for j in 0..lines[0].len() {
        full_data.push(lines.iter().map(|l| l.chars().nth(j).unwrap()).collect::<Vec<_>>());
    }

    for col in &full_data {
        print!("{}", column_counter(col));
    }
}


fn column_counter(letters: &[char]) -> char {
    let mut counts = [0; 26];


    for l in letters.iter() {
        counts[(*l as u8 - b'a') as usize] += 1;
    }

    (counts.iter().enumerate().min_by(|a, b| a.1.cmp(&b.1)).unwrap().0 as u8 + b'a') as char
}