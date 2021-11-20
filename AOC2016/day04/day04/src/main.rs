use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let lines = lines.split("\r\n").collect::<Vec<_>>();

    println!("{}", lines.iter().map(|l| room_sum(l)).sum::<i64>());
}

fn room_sum(line: &str) -> i64 {
    let split_up = line.split('-').collect::<Vec<_>>();

    let mut counts = [0; 26];
    let conct = split_up[..split_up.len() - 1].concat();

    for c in conct.chars() {
        counts[(c as u8 - b'a') as usize] += 1;
    }

    let mut counts = counts
        .into_iter()
        .enumerate()
        .map(|(i, a)| (a, (i as u8 + b'a') as char))
        .into_iter()
        .filter(|l| l.0 != 0)
        .collect::<Vec<_>>();

    counts.sort_by(|a, b| {
        if b.0 != a.0 {
            b.0.cmp(&a.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    let last_split = split_up.last().unwrap().split('[').collect::<Vec<_>>();

    let alphas = last_split[1].strip_suffix(']').unwrap();

    if !alphas.chars().enumerate().all(|(i, c)| c == counts[i].1) {
        return 0;
    }

    last_split[0].parse::<i64>().unwrap()
}
