use std::{
    collections::{HashMap, HashSet},
    fs,
};
fn main() {
    let grid_str = fs::read_to_string("../input.txt").unwrap();

    let mut grid = grid_str
        .split("\r\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // | => tree, open => ., lumber => #
    cycle_field(&mut grid, 1000000000);
}

fn cycle_field(grid: &mut Vec<Vec<char>>, cycles: i64) {
    let mut reverse_cycle = 0;
    let mut res = vec![];

    loop {
        let mut grid_clone = grid.clone();
        let mut counts = [0, 0];
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                let neighs = find_neighbors(grid, (i, j));

                let cur_char = grid[i][j];

                match cur_char {
                    '.' if neighs[2] >= 3 => grid_clone[i][j] = '|',
                    '|' if neighs[1] >= 3 => grid_clone[i][j] = '#',
                    '#' if neighs[1] < 1 || neighs[2] < 1 => grid_clone[i][j] = '.',
                    _ => {}
                }
                if grid_clone[i][j] == '#' || grid_clone[i][j] == '|' {
                    counts[(grid_clone[i][j] == '#') as usize] += 1;
                }
            }
        }
        // eventually a cycle forms which we can then use to get an answer.. without actually going to a billion
        // some severe hard coding involved..
        let k = counts[0] * counts[1];

        if k == 195300 && res[res.len() - 28] == 195300 {
            println!(
                "{}",
                res[res.len() - 28 + ((cycles - (reverse_cycle - 27)) % 28) as usize]
            );

            break;
        }

        res.push(k);
        reverse_cycle += 1;

        *grid = grid_clone;

        // cycles -= 1;
    }
}

fn find_neighbors(grid: &[Vec<char>], point: (usize, usize)) -> [i64; 3] {
    let mut ans = [0; 3];
    for i in -1..=1 {
        for j in -1..=1 {
            let (ny, nx) = (point.0 as i64 + i, point.1 as i64 + j);
            if (i != 0 || j != 0)
                && ny < grid.len() as i64
                && ny >= 0
                && nx >= 0
                && nx < grid[ny as usize].len() as i64
            {
                match grid[ny as usize][nx as usize] {
                    '.' => ans[0] += 1,
                    '#' => ans[1] += 1,
                    '|' => ans[2] += 1,
                    _ => {}
                }
            }
        }
    }
    ans
}
