use std::collections::HashSet;

const MOVES: [[i8; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

pub fn conway_small(grid: &[Vec<char>]) {
    let mut visited: HashSet<Vec<Vec<char>>> = HashSet::new();

    let mut last = grid.to_owned();

    while !visited.contains(&last) {
        visited.insert(last.clone());
        let mut last_last = last.clone();
        for (i, row) in last.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                let mut alive_neighbors = 0;
                for [y, x] in MOVES {
                    let (ny, nx) = (i as i8 + y, j as i8 + x);
                    if ny >= 0
                        && ny < grid.len() as i8
                        && nx >= 0
                        && nx < row.len() as i8
                        && last[ny as usize][nx as usize] == '#'
                    {
                        alive_neighbors += 1;
                    }
                }
                match *c {
                    '.' => {
                        if alive_neighbors == 1 || alive_neighbors == 2 {
                            last_last[i][j] = '#';
                        }
                    }
                    '#' => {
                        if alive_neighbors != 1 {
                            last_last[i][j] = '.';
                        }
                    }
                    _ => {}
                }
            }
            
        }
        last = last_last;
        
    }
    // println!("{:?}", last);
    println!("{}", last.iter().flatten().enumerate().map(| (i, c)| if *c == '#' {2_u64.pow(i as u32)} else {0}).sum::<u64>());
}
// A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
// An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
