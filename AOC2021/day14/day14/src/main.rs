// nncb
use std::{collections::HashMap, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let mut iterator = lines.split("\r\n\r\n");
    let mut current_strain = iterator.next().unwrap().chars().collect::<Vec<_>>();
    let instructions = iterator
        .next()
        .unwrap()
        .split("\r\n")
        .map(|l| {
            let mut veco = l.split(" -> ");
            (veco.next().unwrap(), veco.next().unwrap())
        })
        .collect::<HashMap<_, _>>();

    for _ in 0..10 {
        current_strain = apply_the_process(&current_strain, &instructions);
    }
    println!("{}", give_count(&current_strain));
}

fn apply_the_process(current_state: &[char], translations: &HashMap<&str, &str>) -> Vec<char> {
    let mut ans = Vec::new();
    for (i, v) in current_state.windows(2).enumerate() {
        let v_str = v.iter().collect::<String>();
        if i == 0 {
            ans.extend(
                format!("{}{}{}", v[0], translations.get(&v_str[..]).unwrap(), v[1]).chars(),
            );
        } else {
            ans.extend(format!("{}{}", translations.get(&v_str[..]).unwrap(), v[1]).chars());
        }
    }

    ans
}

fn give_count(current_state: &[char]) -> u64 {
    let mut counts: [u64; 26] = [0; 26];

    for c in current_state {
        counts[(c.to_ascii_lowercase() as u8 - b'a') as usize] += 1;
    }
    counts.iter().max().unwrap() - counts.iter().filter(|n| **n != 0).min().unwrap()
}
