use std::fs;

const MOVES: [[i64;2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
fn main() {
    let grid = fs::read_to_string("../input.txt").unwrap();
    let grid = grid
        .split("\r\n")
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            ans += low_point_score(i, j, &grid);
        }
    }
    println!("{}", ans);
}

fn low_point_score(y: usize, x: usize, grid: &[Vec<u32>]) -> u32 {
    for [yy, xx] in MOVES {
        let [ny, nx] = [yy + y as i64, xx + x as i64];
        if ny >= 0 && ny < grid.len() as i64 && nx >= 0 && nx < grid[0].len() as i64 {
            if grid[y][x] >= grid[ny as usize][nx as usize] {
                return 0;
            }
        }
    }
    grid[y][x] + 1
}