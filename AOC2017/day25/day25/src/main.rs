use std::{collections::HashMap, fs};
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();

    let states_and_beginning_state = lines.split("\r\n\r\n").collect::<Vec<_>>();

    let cc = states_and_beginning_state[1..]
        .iter()
        .map(|l| l.split("If").map(|l| l.trim()).collect::<Vec<_>>())
        .map(|v| {
            (
                v[0].strip_suffix(':').unwrap().split(' ').last().unwrap(),
                [write_move_switch_state(v[1]), write_move_switch_state(v[2])],
            )
        })
        .collect::<HashMap<_, _>>();

    let state_checksum = states_and_beginning_state[0]
        .split("\r\n")
        .collect::<Vec<_>>();

    run_the_cycle(
        line_extractor(state_checksum[0]),
        state_checksum[1]
            .split(' ')
            .nth_back(1)
            .unwrap()
            .parse::<i64>()
            .unwrap(),
        &cc,
    );
}
struct Instruction<'a> {
    value: i64,
    movement: i64,
    state: &'a str,
}

fn line_extractor(line: &str) -> &str {
    line.split(' ').last().unwrap().strip_suffix('.').unwrap()
}

fn write_move_switch_state(lines: &str) -> Instruction {
    let lines = lines.split("\r\n    - ").collect::<Vec<_>>();

    Instruction {
        value: line_extractor(lines[1]).parse::<i64>().unwrap(),
        movement: if line_extractor(lines[2]) == "right" {
            1
        } else {
            -1
        },
        state: line_extractor(lines[3]),
    }
}

fn run_the_cycle<'a>(
    mut cur_state: &'a str,
    mut steps: i64,
    mappings: &'a HashMap<&str, [Instruction; 2]>,
) {
    let mut visited_states: HashMap<i64, i64> = HashMap::new();
    let mut current_cursor_pos = 0;

    while steps > 0 {
        let [if_zero, if_one] = mappings.get(cur_state).unwrap();

        let entry = visited_states.entry(current_cursor_pos).or_default();
        let correct_if = if *entry == 0 { if_zero } else { if_one };

        cur_state = correct_if.state;
        current_cursor_pos += correct_if.movement;
        *entry = correct_if.value;

        steps -= 1;
    }
    println!(
        "ans: {}, merry christmas!",
        visited_states.values().sum::<i64>()
    )
}
