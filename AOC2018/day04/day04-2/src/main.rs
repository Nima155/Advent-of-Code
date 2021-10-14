use std::{collections::HashMap, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let mut lines_to_vec = lines.split("\r\n").collect::<Vec<_>>();
    lines_to_vec.sort_unstable();

    most_tired_guard(&lines_to_vec);
}

fn most_tired_guard(timeline: &[&str]) {
    let mut sleeping_records = HashMap::new();
    let mut entry = &mut [0; 60];
    let (mut s_time, mut e_time) = (0, 0);
    let (mut targeted, mut targeted_max, mut cur_id) = ("", 0, "");

    for l in timeline {
        let deets = l.split(' ').collect::<Vec<_>>();
        match (l.contains("Guard"), l.contains("falls")) {
            (true, _) => {
                entry = sleeping_records
                    .entry(&deets[3][1..])
                    .or_insert([0; 60]);
                cur_id = &deets[3][1..];
            }
            (_, true) => {
                s_time = deets[1][3..deets[1].len() - 1].parse::<u64>().unwrap();
            }
            _ => {
                e_time = deets[1][3..deets[1].len() - 1].parse::<u64>().unwrap();
                (s_time..e_time).for_each(|m| {
                    entry[m as usize] += 1;
                    if entry[m as usize] > targeted_max {
                        targeted = cur_id;
                        targeted_max = entry[m as usize];
                    }
                });
            }
        }
    }

    let maxed = sleeping_records
        .get_key_value(targeted)
        .unwrap()
        .1
        .iter()
        .enumerate()
        .map(|e| (e.1, e.0))
        .max()
        .unwrap();

    println!("{:?} {}: {}", maxed.1, targeted, maxed.1 * targeted.parse::<usize>().unwrap());
}
