use std::cmp::Reverse;


use std::{
    collections::{BinaryHeap, HashMap},
    fs,
};

fn main() {
    let grid_string = fs::read_to_string("../input.txt").unwrap();

    let (mut y, mut x) = (0, 0);
    let mut keys = String::with_capacity(10);
    let grid_vec = grid_string
        .split("\r\n")
        .enumerate()
        .map(|(i, f)| {
            if let Some(j) = f.chars().position(|c| c == '@') {
                y = i;
                x = j;
            }
        
            keys.push_str( &f.chars()
            .filter(|c|{
                c.is_ascii_alphabetic() && c.is_ascii_lowercase() 
            }).collect::<String>());
            
            Vec::from_iter(f.chars())
        })
        .collect::<Vec<_>>();

    find_all_keys(&grid_vec, (y, x), &keys);
}

type StartingPosition = (usize, usize);

const MOVES: [[i32; 2]; 4] = [[0, 1], [1, 0], [-1, 0], [0, -1]];

fn find_all_keys(grid: &[Vec<char>], (y, x): StartingPosition, all_keys: &str) {
    let mut visited: HashMap<(usize, usize, usize), usize> = HashMap::new();

    let mut min_heap = BinaryHeap::new();
  
    let key_count = (0..all_keys.len()).fold(0, |acc, cur| 2_usize.pow(cur as u32) + acc);

    min_heap.push(Reverse((0, (y, x), 0)));

    while !min_heap.is_empty() {
        let Reverse((step, (y, x), keys)) = min_heap.pop().unwrap();
    //    println!("{} {}", keys.len(), key_count);
     

        if keys == key_count {
            println!("{}", step);
            break;
        }
        for [ny, nx] in MOVES {
            let (ny, nx) = (ny + y as i32, nx + x as i32);
            if ny >= 0 && ny < grid.len() as i32 && nx >= 0 && nx < grid[ny as usize].len() as i32 {
                let c = grid[ny as usize][nx as usize];
                let mut key = (ny as usize, nx as usize, keys);

                match c {
                    '.' | '@' => {
                        if !visited.contains_key(&key) || *visited.get(&key).unwrap() > (step + 1) {
                            visited.insert(key, step + 1);
                            min_heap.push(Reverse((step + 1, (ny as usize, nx as usize), key.2)));
                        }
                    }
                    'A'..='Z' => {
                        if (1 & keys >> all_keys.chars().position(|d| d == c.to_ascii_lowercase()).unwrap()) == 1  && ( !visited.contains_key(&key) || *visited.get(&key).unwrap() > (step + 1)) {
                            min_heap.push(Reverse((step + 1, (ny as usize, nx as usize), key.2)));
                        }

                    }
                    'a'..='z' => {
                        let pos = all_keys.chars().position(|e| e == c).unwrap();
                        if (1 & keys >> pos) == 0 {
                            key.2 |= 1 << pos;
                        }
                        if !visited.contains_key(&key) || *visited.get(&key).unwrap() > (step + 1) {
                            visited.insert(key, step + 1);
                            min_heap.push(Reverse((step + 1, (ny as usize, nx as usize), key.2)));
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
