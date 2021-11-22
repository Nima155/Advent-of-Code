use std::fs;
fn main() {
    let lines = fs::read_to_string("../input.txt").unwrap();
    let mut lines = lines
        .split("\r\n")
        .map(|l| {
            let diffs = l.split(' ').collect::<Vec<_>>();
            (
                diffs[3].parse::<i64>().unwrap(),
                0_i64,
                diffs
                    .last()
                    .unwrap()
                    .strip_suffix('.')
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            )
            // positions time cur_position
        })
        .collect::<Vec<_>>();
    lines.push((11, 0, 0)); // comment out for answer to part 1
    for time in 0.. {
        if time_is_right(time, lines.clone()) {
            println!("{}", time);
            break;
        }
    }
}
type Point = (i64, i64, i64);
fn time_is_right(mut time: i64, mut holes: Vec<Point>) -> bool {
    do_cycle(&mut holes, time);
    let mut i = 0;
    while i < holes.len() {
        time += 1;
        do_cycle(&mut holes, 1);
        if holes[i].2 != 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn do_cycle(holes: &mut Vec<Point>, time_inc: i64) {
    for (total_pos, time, cur_pos) in holes {
        *cur_pos = (*cur_pos + time_inc) % *total_pos;
        *time += time_inc;
    }
}
