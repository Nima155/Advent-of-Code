use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
fn main() {
    let grid = fs::read_to_string("../input.txt").unwrap();
    let grid = grid
        .split("\r\n")
        .map(|l| {
            l.chars()
                .map(|n| (n as u8 - b'0') as i64)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    
    println!("{}", shortest_path(&grid));
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
            if nx >= 0 && ny >= 0 && ny < grid.len() as i64 && nx < grid[0].len() as i64 {
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
