use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let vectorized_lines = lines.split("\r\n").collect::<Vec<_>>();


    let (a, b) = vectorized_lines.iter().fold((0, 0), |acc, ele| {
        let inter_med = summifier(ele);
        (acc.0 + inter_med.0, acc.1 + inter_med.1)
    });

    println!("{}", a * b);

}

fn summifier(line: &str) -> (i64, i64) {
    let mut counts = [0; 26];
    for c in line.chars() {
        let indx = ((c as u8) - b'a') as usize;
        counts[indx] += 1;
    };
    (counts.iter().any(|f| *f == 2) as i64, counts.iter().any(|f| *f == 3) as i64)
}