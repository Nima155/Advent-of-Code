use std::{collections::HashMap, fs};
fn main() {
    let instructions = fs::read_to_string("../input.txt").unwrap();
    let instructions = instructions
        .split("\r\n")
        .map(|l| {
            let mut split_up = l.split(' ');
            let [action, steps] = [split_up.next().unwrap(), split_up.next().unwrap()];
            let veco = steps
                .split(',')
                .map(|ins| {
                    let one_two = ins.split("..").collect::<Vec<_>>();
                    (
                        one_two[0][2..].parse::<i64>().unwrap(),
                        one_two[1].parse::<i64>().unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            (action, veco)
        })
        .collect::<Vec<_>>();
    println!("{}", operator(&instructions));
}
// x=-50..50,y=-50..50,z=-50..50
fn operator(instructions: &[(&str, Vec<(i64, i64)>)]) -> i64 {
    let mut states = HashMap::new();
    for (action, steps) in instructions {
        if !(steps[0].0 < -50
            || steps[1].0 < -50
            || steps[2].0 < -50
            || steps[2].1 > 50
            || steps[0].1 > 50
            || steps[1].1 > 50)
        {
            for i in steps[0].0..=steps[0].1 {
                for j in steps[1].0..=steps[1].1 {
                    for v in steps[2].0..=steps[2].1 {
                        states.insert((i, j, v), if *action == "off" { 0 } else { 1 });
                    }
                }
            }
        }
    }
    states.values().sum()
}
