use std::{
    collections::{HashSet, VecDeque},
    fs,
};
fn main() {
    let grid = fs::read_to_string("../input.txt").unwrap();

    let (mut o_y, mut o_x) = (0, 0);

    let grid = grid
        .split("\r\n")
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, e)| {
                    if e == '0' {
                        o_y = i as i64;
                        o_x = j as i64;
                    }
                    e
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    shortest_to_all(&grid, (o_y, o_x));
}

const ALL_COLLECTED: i64 = !(!0 << 8);
const MOVES: [[i64; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];

fn shortest_to_all(grid: &[Vec<char>], origin: (i64, i64)) {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((origin, 1, 0));

    while !queue.is_empty() {
        let ((y, x), keys, steps) = queue.pop_front().unwrap();

        if keys == ALL_COLLECTED && (y, x) == origin {
            println!("{}", steps);
            break;
        }

        for [yy, xx] in MOVES {
            let [ny, nx] = [yy + y, xx + x];

            if ny >= 0
                && ny < grid.len() as i64
                && nx < grid[0].len() as i64
                && nx >= 0
                && grid[ny as usize][nx as usize] != '#'
            {
                let mut n_key = keys;
                let c = grid[ny as usize][nx as usize];

                if c.is_digit(10) {
                    n_key |= 1 << c.to_digit(10).unwrap();
                }

                let storable_value = ((ny, nx), n_key, steps + 1);
                if !visited.contains(&storable_value) {
                    visited.insert(storable_value);
                    queue.push_back(storable_value);
                }
            }
        }
    }
}
