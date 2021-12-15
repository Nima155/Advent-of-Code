// nncb

use std::{collections::HashMap, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let mut iterator = lines.split("\r\n\r\n");
    let current_state = iterator.next().unwrap();
    let mut counts = [0; 26];

    for c in current_state.chars() {
        counts[(c as u8 - b'A') as usize] += 1;
    }

    let mut current_strain = HashMap::new();
    for (a, b) in current_state.chars().zip(current_state[1..].chars()) {
        let fm = format!("{}{}", a, b);
        let ent = current_strain.entry(fm).or_default();
        *ent += 1;
    }

    let instructions = iterator
        .next()
        .unwrap()
        .split("\r\n")
        .map(|l| {
            let veco = l.split(" -> ").collect::<Vec<_>>();
            (
                veco[0],
                [
                    format!("{}{}", &veco[0][0..1], veco[1]),
                    format!("{}{}", veco[1], &veco[0][1..2]),
                ],
            )
        })
        .collect::<HashMap<_, _>>();

    for _ in 0..40 {
        current_strain = apply_the_process(&current_strain, &instructions, &mut counts);
    }
    println!(
        "{:?}",
        counts.iter().max().unwrap() - counts.iter().filter(|c| **c != 0).min().unwrap()
    );
}

fn apply_the_process(
    current_state: &HashMap<String, i64>,
    translations: &HashMap<&str, [String; 2]>,
    counts: &mut [i64],
) -> HashMap<String, i64> {
    let mut ans = HashMap::new();

    for (k, v) in current_state {
        let [o, t] = translations.get(k.as_str()).unwrap();
        // println!("{} {}", t, o);
        let e_1 = ans.entry(o.clone()).or_default();
        *e_1 += v;
        let targ_c = o.chars().last().unwrap();

        counts[(targ_c as u8 - b'A') as usize] += v; // - (k.chars().filter(|c| *c == targ_c).count() as i64  >= 2) as i64;
        let e_2 = ans.entry(t.to_string()).or_default();
        *e_2 += v;
    }

    ans
}
