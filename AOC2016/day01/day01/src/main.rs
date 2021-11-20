use std::{collections::HashSet, fs};

const MOVES: [[i64; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

fn main() {
    let mut visited = HashSet::new();
    let data = fs::read_to_string("../input.txt").unwrap();
    let data = data.split(", ").collect::<Vec<_>>();

    let (mut y, mut x, mut move_indx, mut part2_over) = (0, 0, 0, false);
    for p in data.iter() {
        if p.starts_with('R') {
            move_indx = (move_indx + 1) % 4;
        } else {
            move_indx = if move_indx > 0 { move_indx - 1 } else { 3 };
        }
        let y_boost = p[1..].parse::<i64>().unwrap() * MOVES[move_indx][0];
        let x_boost = p[1..].parse::<i64>().unwrap() * MOVES[move_indx][1];
        populate_visited(&mut visited, y, x, y_boost, &mut part2_over, false);
        populate_visited(&mut visited, x, y, x_boost, &mut part2_over, true);
        y += y_boost;
        x += x_boost;

        if visited.contains(&(y, x)) && !part2_over {
            part2_over = true;
            println!("Part2: {}", y.abs() + x.abs());
        }

        visited.insert((y, x));
    }
    println!("Part1: {}", y.abs() + x.abs());
}

fn populate_visited(
    visited: &mut HashSet<(i64, i64)>,
    dynamic_point: i64,
    static_point: i64,
    boost: i64,
    part2_over: &mut bool,
    dy_is_x: bool,
) {
    let chg = if boost < 0 { -1 } else { (boost > 0) as i64 };

    let mut dp = dynamic_point + chg;

    while dp != (dynamic_point + boost) {
        // code could be cleaner here... should probably be a macro or just another function
        if (dy_is_x && visited.contains(&(static_point, dp))
            || !dy_is_x && visited.contains(&(dp, static_point)))
            && !*part2_over
        {
            *part2_over = true;
            println!("Part2: {}", static_point.abs() + dp.abs());
        }
        if dy_is_x {
            visited.insert((static_point, dp));
        } else {
            visited.insert((dp, static_point));
        }
        dp += chg;
    }
}
