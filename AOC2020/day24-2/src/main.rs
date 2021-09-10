use std::{collections::HashMap, fs};

use lazy_static::lazy_static;
use regex::Regex;

macro_rules! hash_map {
    ($($key: expr => $value: expr),*) => {{
        let mut map = HashMap::new();

        $(map.insert($key, $value);)+

        map
    }};
}

fn main() {
    // start from 0, 0
    let read = fs::read_to_string("input.txt").unwrap();

    let lobby_traversals = read.split("\r\n").collect::<Vec<&str>>();

    let mut tile_coordinates: HashMap<(i32, i32), bool> = HashMap::new();
    // s e, se, sw, w, nw, and ne

    let maps = hash_map!(
        "e" => (0, 1),
        "w" => (0, -1),
        "sw" => (-1, 0),
        "nw" => (1, -1),
        "se" => (-1, 1),
        "ne" => (1, 0)
    );
    let mut cycle = 0;

    for l in &lobby_traversals {
        play_the_game(&mut tile_coordinates, l, &maps);
    }

    while cycle < 100 {
        

        
        let mut extensions = HashMap::with_capacity( tile_coordinates.len() + 100);
        for coord in  &tile_coordinates {

            let mut neighbors = [0, 0];
            
            for (ny, nx) in maps.values() {
                let (x, y) = (*nx + coord.0.0, coord.0.1 + *ny);

                if tile_coordinates.contains_key(&(x, y)) {
                    neighbors[*tile_coordinates.get(&(x, y)).unwrap() as usize] += 1; 
                }

                let mut neighbors_of_neighbor = [0, 0];

                for (nyy, nxx) in maps.values() {
                    let (xj, yj) = (x + nxx, y + nyy);

                    if tile_coordinates.contains_key(&(xj, yj)) {
                        neighbors_of_neighbor[*tile_coordinates.get(&(xj, yj)).unwrap() as usize] += 1; 
                    }
                }

                if neighbors_of_neighbor[0] == 2 {
                    extensions.insert((x, y), false);
                }
            }
            match coord.1 {
                true => { extensions.insert(*coord.0 ,*coord.1 ^ (neighbors[0] == 2)); }
                _ => { extensions.insert(*coord.0 ,*coord.1 ^  (neighbors[0] == 0 || neighbors[0] > 2));  }
            }
        }
        
        tile_coordinates = extensions;
        cycle += 1;
        // println!("{:?}", tile_coordinates.values().filter(|f| !**f).count());
    }

    println!("{:?}", tile_coordinates.values().filter(|f| !**f).count());
}
pub fn play_the_game(
    tiles: &mut HashMap<(i32, i32), bool>,
    line: &str,
    directions: &HashMap<&str, (i32, i32)>,
) {
    lazy_static! {
        static ref RE: Regex = Regex::new("(([ns][we])|([we]))").unwrap();
    };
    
    let (mut x, mut y) = (0, 0);
    for mtch in RE.captures_iter(line) {
        let (n_y, n_x) = directions.get(&mtch[1]).unwrap();

        y += n_y;
        x += n_x;
        
    }

    match tiles.get_mut(&(x, y)) {
        Some(zz) => {
            *zz ^= true;
        }
        _ => {
            tiles.insert((x, y), false);
        }
    }
}
