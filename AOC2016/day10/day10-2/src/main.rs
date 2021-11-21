use std::{collections::HashMap, fs};

macro_rules! instrcution_filler {
    ($maps: expr, $number: expr,  $instructions: expr, $output_bucket: expr, $instructions_of: expr) => {{
        let bucket = $maps.entry($instructions_of.1).or_insert([-1, -1]);
        if $instructions_of.1 != -1 && !$instructions_of.0 {
            if bucket[0] == -1 {
                bucket[0] = $number;
            } else {
                bucket[1] = $number;
                process(($maps, $instructions, $output_bucket), $instructions_of.1);
            }
        } else if $instructions_of.1 != -1 {
            $output_bucket[$instructions_of.1 as usize] = $number;
        }
    }};
}

fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let mut lines = lines.split("\r\n").collect::<Vec<_>>();
    lines.sort_by(|a, b| b.cmp(a));
    let (mut maps, mut insts) = fill_map(&lines);
    let mut output_bucket = [0; 1000];
    for (k, v) in maps.clone() {
        if v[0] != -1 && v[1] != -1 {
            process((&mut maps, &mut insts, &mut output_bucket), k);
        }
    }
    println!(
        "{:?}",
        output_bucket[0] * output_bucket[1] * output_bucket[2]
    );
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Instruction {
    low_to: (bool, i64),
    high_to: (bool, i64),
}
type Ret = (HashMap<i64, [i64; 2]>, HashMap<i64, Vec<Instruction>>);
fn fill_map(lines: &[&str]) -> Ret {
    let (mut mapo, mut mapo_instructions): Ret = (HashMap::new(), HashMap::new());
    for l in lines {
        let split_line = l.split(' ').collect::<Vec<_>>();
        let value = split_line[1].parse::<i64>().unwrap();
        let last = split_line[split_line.len() - 1].parse::<i64>().unwrap();
        if l.starts_with("value") {
            let entry = mapo.entry(last).or_insert([-1, -1]);
            if entry[0] == -1 {
                entry[0] = value;
            } else {
                entry[1] = value;
            }
        } else {
            let entry = mapo_instructions.entry(value).or_default();
            let num = split_line[6].parse::<i64>().unwrap();
            entry.push(Instruction {
                high_to: if split_line[10] != "output" {
                    (false, last)
                } else {
                    (true, last)
                },
                low_to: if split_line[5] != "output" {
                    (false, num)
                } else {
                    (true, num)
                },
            });
        }
    }
    (mapo, mapo_instructions)
}

fn process(
    (maps, instructions, output_bucket): (
        &mut HashMap<i64, [i64; 2]>,
        &mut HashMap<i64, Vec<Instruction>>,
        &mut [i64],
    ),
    cur_bot: i64,
) {
    let v = *maps.get(&cur_bot).unwrap();
    let (low, high) = (i64::min(v[0], v[1]), i64::max(v[0], v[1]));

    if low != -1 && high != -1 && instructions.contains_key(&cur_bot) {
        let v = maps.get_mut(&cur_bot).unwrap();
        v[0] = -1;
        v[1] = -1;
        let instructions_of = instructions.get_mut(&cur_bot).unwrap().pop().unwrap();

        instrcution_filler!(
            maps,
            low,
            instructions,
            output_bucket,
            instructions_of.low_to
        );
        instrcution_filler!(
            maps,
            high,
            instructions,
            output_bucket,
            instructions_of.high_to
        );
    }
}
