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

    for l in lobby_traversals {
        play_the_game(&mut tile_coordinates, l, &maps);
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
