use std::{collections::HashSet, fs};

const MOVES: [[i64; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];
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

    let mut ans = Vec::new();
    let mut visited = HashSet::new();
    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            if grid[j][i] != 9 {
                let mut res = low_point_score(i, j, &grid, false, &mut visited);
                res += (res != 0) as usize;
                ans.push(res);
            }
        }
    }

    ans.sort_unstable();
    println!(
        "{:?}",
        ans.iter()
            .skip(ans.len() - 3)
            .product::<usize>()
    );
}

fn low_point_score(
    y: usize,
    x: usize,
    grid: &[Vec<u32>],
    dfs_search: bool,
    visited: &mut HashSet<(i64, i64)>,
) -> usize {
    if visited.contains(&(y as i64, x as i64)) {
        return 0;
    }

    if dfs_search {
        visited.insert((y as i64, x as i64));
    }

    let mut ans = 0;
    for [yy, xx] in MOVES {
        let [ny, nx] = [yy + y as i64, xx + x as i64];
        if ny >= 0 && ny < grid.len() as i64 && nx >= 0 && nx < grid[0].len() as i64 {
            if grid[y][x] >= grid[ny as usize][nx as usize] && !dfs_search {
                return 0;
            } else if dfs_search
                && grid[ny as usize][nx as usize] != 9
                && grid[y][x] < grid[ny as usize][nx as usize]
                && !visited.contains(&(ny, nx))
            {
                ans += 1 + low_point_score(ny as usize, nx as usize, grid, dfs_search, visited)
            }
        }
    }

    low_point_score(y, x, grid, true, visited) + ans
}
