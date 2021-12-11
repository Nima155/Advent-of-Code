use std::{fs, collections::{HashSet, VecDeque}};
fn main() {
    let grid = fs::read_to_string("../input.txt").unwrap();
    let mut grid = grid
        .split("\r\n")
        .map(|l| {
            l.chars()
                .map(|n| n.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    
    let mut ans = 0;
    for _ in 0..100 {
        ans += flashing_lights(&mut grid);
    }
    println!("{}", ans);
}



fn flashing_lights(grid: &mut [Vec<u32>]) -> u64 {
    let mut nines = VecDeque::new();

    let mut flashed = HashSet::new();
    for (i, row) in grid.iter_mut().enumerate() {
        for (j, n) in row.iter_mut().enumerate() {
            if *n == 9 {
                nines.push_back((i as i64, j as i64));
                flashed.insert((i as i64, j as i64));
            }
            *n = (*n + 1) % 10;
        }
    }

    

    while !nines.is_empty() {
        let (y, x) = nines.pop_front().unwrap();
        for i in -1..2 {
            for j in -1..2 {
                if i != 0 || j != 0 {
                    let [ny, nx] = [y + i, x + j];
                    if ny >= 0 && ny < grid.len() as i64 && nx >= 0 && nx < grid[0].len() as i64 {
                        let [nyy, nxx] = [ny as usize, nx as usize];
                        let value = grid[nyy][nxx];
                        if !flashed.contains(&(ny, nx)) {
                            if value == 9 {
                                flashed.insert((ny, nx));
                                nines.push_back((ny, nx));
                            }
                            grid[nyy][nxx] = (grid[nyy][nxx] + 1) % 10;
                        }
                        
                        
                    }
                }
            }
        }
    }

    flashed.len() as u64
}