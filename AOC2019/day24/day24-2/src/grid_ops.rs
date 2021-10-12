use std::collections::BTreeMap;

const MOVES: [[i8; 2]; 4] = [[0, 1], [1, 0], [0, -1], [-1, 0]];

macro_rules! trace_neighbor {
    ($cur_depth: expr, $grids: expr, $sign: tt, $which: expr ) => {
        {
            let ans;
            if $grids.contains_key(&($cur_depth - 1)) {
                // println!("{}", $cur_depth $sign 1);
                let des_grid = $grids.get(&($cur_depth - 1)).unwrap();
                if $which == 0 {
                    ans = (des_grid[des_grid.len() / 2 $sign 1][des_grid[0].len() / 2] == '#') as i64;
                } else {
                    ans = (des_grid[des_grid.len() / 2][des_grid[0].len() / 2 $sign 1] == '#') as i64;
                }
            } else {
                ans = 0;
            };
            return ans;
        }

    };
}

fn neighbor_reducer(
    cur_depth: i64,
    (ny, nx): (i8, i8),
    grids: &BTreeMap<i64, Vec<Vec<char>>>,
    (g_y, g_x): (usize, usize),
    (prev_y, prev_x): (usize, usize),
) -> i64 {
    if ny >= 0
        && ny < g_y as i8
        && nx >= 0
        && nx < g_x as i8
        && grids.get(&cur_depth).unwrap()[ny as usize][nx as usize] == '#'
    {
        return 1;
    }
    match (ny < 0, ny >= g_y as i8, nx < 0, nx >= g_x as i8) {
        (true, _, _, _) => {
            trace_neighbor!(cur_depth, grids, -, 0);
        }
        (_, true, _, _) => {
            trace_neighbor!(cur_depth, grids, +, 0);
        }
        (_, _, true, _) => {
            trace_neighbor!(cur_depth, grids, -, 1)
        }
        (_, _, _, true) => {
            trace_neighbor!(cur_depth, grids, +, 1)
        }
        _ => {}
    }
    if ny == ((g_y / 2) as i8) && nx == ((g_x / 2) as i8) && grids.contains_key(&(cur_depth + 1)) {
        
            let grid = grids.get(&(cur_depth + 1)).unwrap();
            match (
                prev_y == g_y / 2 + 1,
                prev_y == g_y / 2 - 1,
                prev_x == g_x / 2 + 1,
                prev_x == g_x / 2 - 1,
            ) {
                (true, _, _, _) => {
                    return grid[grid.len() - 1].iter().filter(|c| **c == '#').count() as i64;
                }
                (_, true, _, _) => {
                    return grid[0].iter().filter(|c| **c == '#').count() as i64;
                }
                (_, _, true, _) => {
                    return grid
                        .iter()
                        .map(|c| c[c.len() - 1])
                        .filter(|car| *car == '#')
                        .count() as i64;
                }
                (_, _, _, true) => {
                    return grid.iter().map(|c| c[0]).filter(|car| *car == '#').count() as i64;
                }
                _ => {}
            }
        
    }
    0
}

fn grid_tarzan(grids: &mut BTreeMap<i64, Vec<Vec<char>>>) {
    let grid_clone = grids.clone();
    let mut actual = BTreeMap::new();
    
    for (k, last) in grids.iter_mut() {
        let mut last_last = last.clone();

        for (i, row) in last.iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                let mut alive_neighbors = 0;

                if (i, j) != (last.len() / 2, last[0].len() / 2) {
                    for [y, x] in MOVES {
                        let (ny, nx) = (i as i8 + y, j as i8 + x);
                        
                        alive_neighbors += neighbor_reducer(
                            *k,
                            (ny, nx),
                            &grid_clone,
                            (row.len(), row.len()),
                            (i, j),
                        );
                    
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
        }
        actual.insert(*k, last_last);

    }
    *grids = actual;
    // grid_clone.iter().filter(|c| !grids.contains_key(c.0)).collect::<Vec<_>>().iter().for_each(|(dep, _)| {
    //     grids.insert(**dep, vec![vec!['.'; 5]; 5]);
    // });
}

pub fn conway_small(grid: &[Vec<char>]) {
    let mut grids: BTreeMap<i64, Vec<Vec<char>>> = BTreeMap::new();
    grids.insert(0, grid.to_owned());
    let mut min = 0;

    let (mut min_d, mut max_d) = (-1, 1);
    while min < 200  {
        grids.insert(min_d, vec![vec!['.'; 5]; 5]);
        grids.insert(max_d, vec![vec!['.'; 5]; 5]);
        max_d += 1;
        min_d -= 1;
        grid_tarzan(&mut grids);
        
        min += 1;
    }
    println!("{:?}", grids.iter().map(|(_, v)| v.iter().flatten().filter(|c| **c == '#').count()).sum::<usize>());
    
}
// A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
// An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
