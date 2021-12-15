use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
fn main() {
    let grid = fs::read_to_string("../input.txt").unwrap();
    let mut last_grid = grid
        .split("\r\n")
        .map(|l| {
            l.chars()
                .map(|n| (n as u8 - b'0') as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    expand_grid(&mut last_grid);

    println!("{}", shortest_path(&last_grid));
}


fn expand_grid(last_grid: &mut Vec<Vec<i64>>)  {

    let (mut i, mut j) = (0, 0);
    let (rr, cc) = (last_grid.len(), last_grid[0].len());
    let mut row = 0;

    while i < rr * 5 {
        let mut temp_grid = Vec::new();
        while j < cc * 4 {
            
            let prev_r = row;
            while row < (i + rr) {
                let clone_row = last_grid[row].clone();

                last_grid[row].extend(
                    clone_row[j..j + cc]
                        .iter()
                        .map(|c| i64::max((*c + 1) % 10, 1)),
                );

                if j == 0 {
                    temp_grid.push(
                        last_grid[row][j..j + cc]
                            .iter()
                            .map(|c| i64::max((*c + 1) % 10, 1))
                            .collect::<Vec<_>>(),
                    );
                }
                row += 1;
            }
            row = prev_r;
            j += cc;
        }

        if i + rr < rr * 5 {
            last_grid.extend(temp_grid);
        }

        j = 0;
        row = i + rr;
        i += rr;
    }
}

const MOVES: [[i64; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

fn shortest_path(grid: &[Vec<i64>]) -> i64 {
    let mut visited = HashMap::new();

    let mut priority_queue = BinaryHeap::new();
    priority_queue.push(Reverse((0, (0, 0))));

    while !priority_queue.is_empty() {
        let Reverse((steps, (yy, xx))) = priority_queue.pop().unwrap();

        if yy == (grid.len() - 1) as i64 && xx == (grid[0].len() - 1) as i64 {
            return steps;
        }

        for [y, x] in MOVES {
            let (ny, nx) = (yy + y, xx + x);
            if nx >= 0 && ny >= 0 && ny < grid.len() as i64 && nx < grid[ny as usize].len() as i64 {
                let score = grid[ny as usize][nx as usize];
                if !visited.contains_key(&(ny, nx)) || visited[&(ny, nx)] > steps + score {
                    visited.insert((ny, nx), steps + score);
                    priority_queue.push(Reverse((steps + score, (ny, nx))));
                }
            }
        }
    }

    0
}
