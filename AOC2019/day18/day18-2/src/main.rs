use std::cmp::Reverse;

use std::collections::{HashMap, HashSet};
use std::{collections::BinaryHeap, fs};

fn main() {
    let grid_string = fs::read_to_string("./input.txt").unwrap();
    let (mut starting_positions, mut starting_positions_quad) = (Vec::new(), Vec::new());
    let mut keys = String::with_capacity(10);
    let mut quadrants = [String::new(), String::new(), String::new(), String::new()];
    let mut quadrant = 0;

    let grid_vec = grid_string
        .split("\r\n")
        .enumerate()
        .map(|(i, f)| {
            f.chars().enumerate().for_each(|(j, c)| match c {
                '@' => {
                    starting_positions.push((i, j));
                    starting_positions_quad.push((i, j, quadrant));
                    quadrant += 1;
                }
                'a'..='z' => {
                    keys.push(c);
                }
                _ => {}
            });

            // keys.push_str(
            //     &f.chars()
            //         .filter(|c| c.is_ascii_alphabetic() && c.is_ascii_lowercase())
            //         .collect::<String>(),
            // );

            Vec::from_iter(f.chars())
        })
        .collect::<Vec<_>>();

    // let key_count = (0..keys.len()).fold(0, |acc, cur| 2_usize.pow(cur as u32) + acc);
    quadrants_finder(
        &grid_vec,
        &starting_positions_quad,
        &mut HashSet::new(),
        &mut quadrants,
    );

    find_all_keys(&grid_vec, &starting_positions, &keys, &quadrants);
}

type StartingPosition = [(usize, usize)];

const MOVES: [[i32; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];
fn find_all_keys(
    grid: &[Vec<char>],
    starting_positions: &StartingPosition,
    all_keys: &str,
    quadrants: &[String; 4],
) {
    let mut visited: HashMap<(usize, usize, usize), usize> = HashMap::new();

    let mut min_heap = [
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
        BinaryHeap::new(),
    ];

    let key_count = (0..all_keys.len()).fold(0, |acc, cur| 2_usize.pow(cur as u32) + acc);

    for (i, e) in starting_positions.iter().enumerate() {
        min_heap[i].push(Reverse((
            0,
            *e,
            quadrants.iter().enumerate().fold(0, |mut acc, (indx, strs)| {
                if indx != i {
                    acc |= strs.chars().fold(0, |mut acc_2, car| {
                        acc_2 |= 1 << all_keys.chars().position(|e| e == car).unwrap();
                        acc_2
                    });
                }
                acc
            }),
        )));
    }
    let mut all_vis = HashMap::new();
    'a: loop {
        
        for i in 0..4 {
            if !min_heap[i].is_empty() {
                let Reverse((step, (y, x), keys)) = min_heap[i].pop().unwrap();
                //    println!("{} {}", keys.len(), key_count);
                if keys == key_count {
                    all_vis.entry(i).or_insert(step);
                        
                    
                    if all_vis.len() == 4 {
                        println!("{:?} {}", all_vis, all_vis.values().sum::<usize>());
                        break 'a;
                    }
                    break;
                }
                for [ny, nx] in MOVES {
                    let (ny, nx) = (ny + y as i32, nx + x as i32);
                    if ny >= 0
                        && ny < grid.len() as i32
                        && nx >= 0
                        && nx < grid[ny as usize].len() as i32
                    {
                        let c = grid[ny as usize][nx as usize];
                        let mut key = (ny as usize, nx as usize, keys);

                        match c {
                            '.' | '@' => {
                                if !visited.contains_key(&key)
                                    || *visited.get(&key).unwrap() > (step + 1)
                                {
                                    visited.insert(key, step + 1);
                                    min_heap[i].push(Reverse((
                                        step + 1,
                                        (ny as usize, nx as usize),
                                        key.2,
                                    )));
                                }
                            }
                            'A'..='Z' => {
                                if (1 & keys
                                    >> all_keys
                                        .chars()
                                        .position(|d| d == c.to_ascii_lowercase())
                                        .unwrap())
                                    == 1
                                    && (!visited.contains_key(&key)
                                        || *visited.get(&key).unwrap() > (step + 1))
                                {
                                    min_heap[i].push(Reverse((
                                        step + 1,
                                        (ny as usize, nx as usize),
                                        key.2,
                                    )));
                                }
                            }
                            'a'..='z' => {
                                
                                let pos = all_keys.chars().position(|e| e == c).unwrap();
                                if (1 & keys >> pos) == 0 {
                                    key.2 |= 1 << pos;
                                }
                                if !visited.contains_key(&key)
                                    || *visited.get(&key).unwrap() > (step + 1)
                                {
                                    visited.insert(key, step + 1);
                                    min_heap[i].push(Reverse((
                                        step + 1,
                                        (ny as usize, nx as usize),
                                        key.2,
                                    )));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}
fn quadrants_finder(
    grid: &[Vec<char>],
    starting_positions: &[(usize, usize, u8)],
    visited: &mut HashSet<(usize, usize)>,
    quadrant: &mut [String; 4],
) {
    let mut new_starting = Vec::new();
    for (y, x, quad) in starting_positions {
        visited.insert((*y, *x));
        for [ny, nx] in MOVES {
            let (ny, nx) = (*y as i32 + ny, *x as i32 + nx);
            if ny >= 0
                && ny < grid.len() as i32
                && nx >= 0
                && nx < grid[0].len() as i32
                && !visited.contains(&(ny as usize, nx as usize))
            {
                match grid[ny as usize][nx as usize] {
                    c @ 'a'..='z' => {
                        quadrant[*quad as usize].push(c);
                        new_starting.push((ny as usize, nx as usize, *quad));
                    }
                    '.' | '@' | 'A'..='Z' => {
                        new_starting.push((ny as usize, nx as usize, *quad));
                    }
                    _ => {}
                }
            }
        }
    }
    if !new_starting.is_empty() {
        quadrants_finder(grid, &new_starting, visited, quadrant);
    }
}
